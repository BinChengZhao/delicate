use super::prelude::*;

pub(crate) fn route_config() -> Route {
    Route::new().at("/api/task_instance/kill", post(kill_task_instance))
}

// Depending on the event, scheduler records/updates different logs.
// Bulk operations are supported for log messages passed from delicate-executor.
#[handler]

async fn create_task_logs(
    Json(events_collection): Json<delicate_utils_task_log::SignedExecutorEventCollection>,
    pool: Data<&Arc<db::ConnectionPool>>,
) -> impl IntoResponse {
    let r = async {
        debug!(
            "Event collection - {:?}",
            &events_collection.event_collection
        );

        pre_create_task_logs(events_collection, pool).await
    }
    .instrument(span!(
        Level::INFO,
        "status-reporter",
        log_id = get_unique_id_string().deref()
    ))
    .await;

    let response = Into::<UnifiedResponseMessages<usize>>::into(r);
    Json(response)
}

async fn pre_create_task_logs(
    events_collection: delicate_utils_task_log::SignedExecutorEventCollection,
    pool: Data<&Arc<db::ConnectionPool>>,
) -> Result<usize, CommonError> {
    use delicate_utils_task_log::EventType;

    let executor_processor_id = (events_collection)
        .event_collection
        .events
        .get(0)
        .map(|e| e.executor_processor_id)
        .ok_or_else(|| CommonError::DisPass(" `event_collection` is empty . ".into()))?;

    let token = model::get_executor_token_by_id(executor_processor_id, pool.get()?).await;

    let delicate_utils_task_log::ExecutorEventCollection { events, .. } =
        events_collection.get_executor_event_collection_after_verify(token.as_deref())?;

    let conn = pool.get()?;

    let mut effect_num = 0;
    let mut new_task_logs: Vec<model::NewTaskLog> = Vec::new();
    let mut supply_task_logs: Vec<model::SupplyTaskLogTuple> = Vec::new();

    events
        .into_iter()
        .for_each(|e| match Into::<EventType>::into(e.event_type) {
            EventType::TaskPerform => new_task_logs.push(e.into()),
            EventType::Unknown => {}
            _ => supply_task_logs.push(e.into()),
        });

    debug!("{:?}, {:?}", &new_task_logs, &supply_task_logs);

    let num = spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
        conn.transaction(|| {
            effect_num += batch_insert_task_logs(&conn, new_task_logs)?;

            effect_num += batch_update_task_logs(&conn, supply_task_logs)?;

            Ok(effect_num)
        })
    })
    .await??;

    Ok(num)
}

#[handler]

async fn show_task_logs(
    Json(query_params): Json<model::QueryParamsTaskLog>,
    pool: Data<&Arc<db::ConnectionPool>>,
) -> impl IntoResponse {
    if let Ok(conn) = pool.get() {
        let f_result = spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
            let query_builder = model::TaskLogQueryBuilder::query_all_columns();

            let task_logs = query_params
                .clone()
                .query_filter(query_builder)
                .paginate(query_params.page)
                .set_per_page(query_params.per_page)
                .load::<model::TaskLog>(&conn)?;

            let per_page = query_params.per_page;
            let count_builder = model::TaskLogQueryBuilder::query_count();
            let count = query_params
                .query_filter(count_builder)
                .get_result::<i64>(&conn)?;

            let front_end_task_logs: Vec<model::FrontEndTaskLog> =
                task_logs.into_iter().map(|t| t.into()).collect();
            Ok(PaginateData::<model::FrontEndTaskLog>::default()
                .set_data_source(front_end_task_logs)
                .set_page_size(per_page)
                .set_total(count)
                .set_state_desc::<state::task_log::State>())
        })
        .await;

        let page = f_result
            .map(|page_result| {
                Into::<UnifiedResponseMessages<PaginateData<model::FrontEndTaskLog>>>::into(
                    page_result,
                )
            })
            .unwrap_or_else(|e| {
                UnifiedResponseMessages::<PaginateData<model::FrontEndTaskLog>>::error()
                    .customized_error_msg(e.to_string())
            });
        return Json(page);
    }

    Json(UnifiedResponseMessages::<
        PaginateData<model::FrontEndTaskLog>,
    >::error())
}

#[handler]

async fn show_task_log_detail(
    Json(query_params): Json<model::RecordId>,
    pool: Data<&Arc<db::ConnectionPool>>,
) -> impl IntoResponse {
    use db::schema::task_log_extend;

    if let Ok(conn) = pool.get() {
        let f_result = spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
            let task_log_extend = task_log_extend::table
                .find(query_params.record_id.0)
                .first::<model::TaskLogExtend>(&conn)?;

            Ok(task_log_extend)
        })
        .await;

        let log_extend = f_result
            .map(|log_extend_result| {
                Into::<UnifiedResponseMessages<model::TaskLogExtend>>::into(log_extend_result)
            })
            .unwrap_or_else(|e| {
                UnifiedResponseMessages::<model::TaskLogExtend>::error()
                    .customized_error_msg(e.to_string())
            });
        return Json(log_extend);
    }

    Json(UnifiedResponseMessages::<model::TaskLogExtend>::error())
}

#[handler]

async fn kill_task_instance(
    req: &Request,
    Json(task_record): Json<model::TaskRecord>,
    pool: Data<&Arc<db::ConnectionPool>>,
) -> impl IntoResponse {
    let response_result = kill_one_task_instance(req, pool, task_record).await;

    let response = Into::<UnifiedResponseMessages<()>>::into(response_result);
    Json(response)
}

fn batch_insert_task_logs(
    conn: &db::PoolConnection,
    mut new_task_logs: Vec<model::NewTaskLog>,
) -> QueryResult<usize> {
    use db::schema::{task, task_log};

    if !new_task_logs.is_empty() {
        let task_ids: Vec<i64> = new_task_logs.iter().map(|e| e.task_id).collect();

        let tasks: HashMap<i64, model::task::SupplyTask> =
            model::TaskQueryBuilder::query_supply_task_log()
                .filter(task::id.eq_any(&task_ids[..]))
                .load::<model::task::SupplyTask>(conn)?
                .into_iter()
                .map(|t| (t.id, t))
                .collect();

        new_task_logs.iter_mut().for_each(|t| {
            if let Some(task) = tasks.get(&t.task_id) {
                t.name.clone_from(&task.name);
                t.description.clone_from(&task.description);
                t.command.clone_from(&task.command);
                t.frequency.clone_from(&task.frequency);
                t.cron_expression.clone_from(&task.cron_expression);
                t.tag.clone_from(&task.tag);
                t.maximum_parallel_runnable_num
                    .clone_from(&task.maximum_parallel_runnable_num);
            }
        });

        return diesel::insert_into(task_log::table)
            .values(&new_task_logs[..])
            .execute(conn);
    }

    Ok(0)
}

fn batch_update_task_logs(
    conn: &db::PoolConnection,
    supply_task_logs: Vec<model::SupplyTaskLogTuple>,
) -> QueryResult<usize> {
    use db::schema::task_log_extend;

    let mut effect_num = 0;

    for supply_task_log in supply_task_logs.iter() {
        effect_num += diesel::update(&supply_task_log.0)
            .set(&supply_task_log.0)
            .execute(conn)?;
    }

    let supply_task_logs_extend: Vec<model::SupplyTaskLogExtend> = supply_task_logs
        .into_iter()
        .map(|model::SupplyTaskLogTuple(_, t)| t)
        .collect();

    diesel::insert_into(task_log_extend::table)
        .values(&supply_task_logs_extend[..])
        .execute(conn)?;

    Ok(effect_num)
}

async fn kill_one_task_instance(
    req: &Request,
    pool: Data<&Arc<db::ConnectionPool>>,
    model::TaskRecord {
        task_id,
        record_id,
        executor_processor_id,
    }: model::TaskRecord,
) -> Result<(), CommonError> {
    use db::schema::task_log;

    let operation_log_pair_option = generate_operation_task_log_modify_log(
        req.get_session(),
        &CommonTableRecord::default()
            .set_id(record_id.0)
            .set_description("kill task instance."),
    )
    .ok();
    send_option_operation_log_pair(operation_log_pair_option).await;

    let request_client = req
        .extensions()
        .get::<RequestClient>()
        .expect("Missing Components `RequestClient`");

    let token = model::get_executor_token_by_id(executor_processor_id, pool.get()?).await;

    let conn = pool.get()?;
    let host = spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
        let host = task_log::table
            .find(&record_id.0)
            .filter(task_log::status.eq(state::task_log::State::Running as i16))
            .select(task_log::executor_processor_host)
            .first::<String>(&conn)?;

        diesel::update(task_log::table.find(&record_id.0))
            .filter(task_log::status.eq(state::task_log::State::Running as i16))
            .set(task_log::status.eq(state::task_log::State::TmanualCancellation as i16))
            .execute(&conn)?;

        Ok(host)
    })
    .await??;

    let url = "http://".to_string() + (host.deref()) + "/api/task_instance/kill";

    let record = delicate_utils_task_log::CancelTaskRecord::default()
        .set_task_id(task_id)
        .set_record_id(record_id.0)
        .set_time(timestamp())
        .sign(token.as_deref())?;

    request_client
        .post(url)
        .json(&record)
        .send()
        .await?
        .json::<UnifiedResponseMessages<()>>()
        .await?
        .into()
}

#[handler]
async fn delete_task_log(
    req: &Request,
    Json(delete_params): Json<model::DeleteParamsTaskLog>,
    pool: Data<&Arc<db::ConnectionPool>>,
) -> impl IntoResponse {
    let operation_log_pair_option =
        generate_operation_task_delete_log(req.get_session(), &delete_params).ok();
    send_option_operation_log_pair(operation_log_pair_option).await;

    if let Ok(conn) = pool.get() {
        return Json(Into::<UnifiedResponseMessages<()>>::into(
            pre_delete_task_log(delete_params, conn).await,
        ));
    }

    Json(UnifiedResponseMessages::<()>::error())
}

async fn pre_delete_task_log(
    delete_params: model::DeleteParamsTaskLog,
    conn: db::PoolConnection,
) -> Result<(), CommonError> {
    use db::schema::{task_log, task_log_extend};

    // Because `diesel` does not support join table deletion, so here is divided into two steps to delete logs.

    // 1. query the primary key of task-log according to the given conditions, with a single maximum limit of 524288 items.

    // 2. the primary key in batches of 2048 items and then start executing the deletion, task-log and task-log-extend.

    spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
        let query_builder = model::TaskLogQueryBuilder::query_id_column();
        let task_log_ids = delete_params
            .query_filter(query_builder)
            .load::<i64>(&conn)?;

        let ids_chunk = task_log_ids.chunks(2048);
        for ids in ids_chunk {
            conn.transaction::<_, diesel::result::Error, _>(|| {
                diesel::delete(task_log::table.filter(task_log::id.eq_any(ids))).execute(&conn)?;
                diesel::delete(task_log_extend::table.filter(task_log_extend::id.eq_any(ids)))
                    .execute(&conn)?;
                Ok(())
            })?;
        }

        Ok(())
    })
    .await??;
    Ok(())
}
