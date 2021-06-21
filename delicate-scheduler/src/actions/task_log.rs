use super::prelude::*;

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_task_logs)
        .service(show_task_logs)
        .service(kill_task_instance);
}

// Depending on the event, scheduler records/updates different logs.
// Bulk operations are supported for log messages passed from delicate-executor.
#[post("/api/task_logs/event_trigger")]
async fn create_task_logs(
    web::Json(events_collection): web::Json<delicate_utils_task_log::SignedExecutorEventCollection>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    use delicate_utils_task_log::EventType;

    // TODO: Set token.
    if let Ok(delicate_utils_task_log::ExecutorEventCollection { events, .. }) =
        events_collection.get_executor_event_collection_after_verify(Some(""))
    {
        if let Ok(conn) = pool.get() {
            return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<usize>>::into(
                web::block::<_, _, diesel::result::Error>(move || {
                    conn.transaction(|| {
                        let mut effect_num = 0;

                        let mut new_task_logs: Vec<model::NewTaskLog> = Vec::new();
                        let mut supply_task_logs: Vec<model::SupplyTaskLogTuple> = Vec::new();

                        events.into_iter().for_each(|e| {
                            match Into::<EventType>::into(e.event_type) {
                                EventType::TaskPerform => new_task_logs.push(e.into()),
                                EventType::Unknown => {}
                                _ => supply_task_logs.push(e.into()),
                            }
                        });

                        effect_num += batch_insert_task_logs(&conn, new_task_logs)?;

                        effect_num += batch_update_task_logs(&conn, supply_task_logs)?;

                        Ok(effect_num)
                    })
                })
                .await,
            ));
        }
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<usize>::error())
}

#[post("/api/task_log/list")]
async fn show_task_logs(
    web::Json(query_params): web::Json<model::QueryParamsTaskLog>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(
            Into::<UnifiedResponseMessages<model::PaginateTaskLogs>>::into(
                web::block::<_, _, diesel::result::Error>(move || {
                    let query_builder = model::TaskLogQueryBuilder::query_all_columns();

                    let task_logs = query_params
                        .clone()
                        .query_filter(query_builder)
                        .paginate(query_params.page)
                        .load::<model::TaskLog>(&conn)?;

                    let per_page = query_params.per_page;
                    let count_builder = model::TaskLogQueryBuilder::query_count();
                    let count = query_params
                        .query_filter(count_builder)
                        .get_result::<i64>(&conn)?;

                    Ok(model::task_log::PaginateTaskLogs::default()
                        .set_task_logs(task_logs)
                        .set_per_page(per_page)
                        .set_total_page(count))
                })
                .await,
            ),
        );
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<model::PaginateTask>::error())
}

#[post("/api/task_instance/kill")]
async fn kill_task_instance(
    web::Json(task_record): web::Json<model::TaskRecord>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    let response_result = kill_one_task_instance(pool, task_record).await;

    if let Ok(response) = response_result {
        return HttpResponse::Ok().json(response);
    }

    HttpResponse::Ok().json(
        UnifiedResponseMessages::<()>::error()
            .customized_error_msg(response_result.expect_err("").to_string()),
    )
}

fn batch_insert_task_logs(
    conn: &db::PoolConnection,
    mut new_task_logs: Vec<model::NewTaskLog>,
) -> QueryResult<usize> {
    use db::schema::{task, task_log};
    use std::collections::HashMap;

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
            let task = &tasks[&t.id];
            t.name.clone_from(&task.name);
            t.description.clone_from(&task.description);
            t.command.clone_from(&task.command);
            t.frequency.clone_from(&task.frequency);
            t.cron_expression.clone_from(&task.cron_expression);
            t.tag.clone_from(&task.tag);
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
    pool: ShareData<db::ConnectionPool>,
    model::TaskRecord {
        task_id,
        record_id,
        executor_processor_id,
    }: model::TaskRecord,
) -> Result<UnifiedResponseMessages<()>, CommonError> {
    use db::schema::task_log;

    let token = model::get_executor_token_by_id(executor_processor_id, pool.get()?).await;

    let conn = pool.get()?;
    let host = web::block::<_, String, diesel::result::Error>(move || {
        let host = task_log::table
            .find(&record_id)
            .filter(task_log::status.eq(state::task_log::State::Running as i16))
            .select(task_log::executor_processor_host)
            .first::<String>(&conn)?;
        diesel::update(task_log::table.find(&record_id))
            .set(task_log::status.eq(state::task_log::State::TmanualCancellation as i16))
            .execute(&conn)?;
        Ok(host)
    })
    .await?;

    let client = RequestClient::default();
    let url = "http://".to_string() + &host + "/api/task_instance/kill";

    let record = delicate_utils_task_log::CancelTaskRecord::default()
        .set_task_id(task_id)
        .set_record_id(record_id)
        .set_time(get_timestamp())
        .sign(token.as_deref())?;

    let response = client
        .post(url)
        .send_json(&record)
        .await?
        .json::<UnifiedResponseMessages<()>>()
        .await?;

    Ok(response)
}
