use super::prelude::*;

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(show_executor_processor_binds)
        .service(create_executor_processor_bind)
        .service(update_executor_processor_bind)
        .service(delete_executor_processor_bind);
}

#[post("/api/executor_processor_bind/create")]
async fn create_executor_processor_bind(
    req: HttpRequest,
    web::Json(executor_processor_binds): web::Json<model::NewExecutorProcessorBinds>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    use db::schema::executor_processor_bind;

    let operation_log_pair_option = generate_operation_executor_processor_bind_addtion_log(
        &req.get_session(),
        &executor_processor_binds,
    )
    .ok();

    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<usize>>::into(
            web::block(move || {
                let new_binds: Vec<model::NewExecutorProcessorBind> = executor_processor_binds
                    .executor_ids
                    .iter()
                    .map(|executor_id| model::NewExecutorProcessorBind {
                        name: executor_processor_binds.name.clone(),
                        group_id: executor_processor_binds.group_id,
                        executor_id: *executor_id,
                        weight: executor_processor_binds.weight,
                    })
                    .collect();

                operation_log_pair_option
                    .map(|operation_log_pair| operate_log(&conn, operation_log_pair));

                diesel::insert_into(executor_processor_bind::table)
                    .values(&new_binds[..])
                    .execute(&conn)
            })
            .await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<()>::error())
}

#[post("/api/executor_processor_bind/list")]
async fn show_executor_processor_binds(
    web::Json(query_params): web::Json<model::QueryParamsExecutorProcessorBind>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<
            UnifiedResponseMessages<PaginateData<model::ExecutorProcessorBind>>,
        >::into(
            web::block::<_, _, diesel::result::Error>(move || {
                let query_builder = model::ExecutorProcessorBindQueryBuilder::query_all_columns();

                let executor_processor_binds = query_params
                    .clone()
                    .query_filter(query_builder)
                    .paginate(query_params.page)
                    .set_per_page(query_params.per_page)
                    .load::<model::ExecutorProcessorBind>(&conn)?;

                let per_page = query_params.per_page;
                let count_builder = model::ExecutorProcessorBindQueryBuilder::query_count();
                let count = query_params
                    .query_filter(count_builder)
                    .get_result::<i64>(&conn)?;

                Ok(PaginateData::<model::ExecutorProcessorBind>::default()
                    .set_data_source(executor_processor_binds)
                    .set_page_size(per_page)
                    .set_total(count))
            })
            .await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<PaginateData<()>>::error())
}

#[post("/api/executor_processor_bind/update")]
async fn update_executor_processor_bind(
    req: HttpRequest,
    web::Json(executor_processor_bind): web::Json<model::UpdateExecutorProcessorBind>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<()>>::into(
        pre_update_executor_processor_bind(req, executor_processor_bind, pool).await,
    ));
}

async fn pre_update_executor_processor_bind(
    req: HttpRequest,
    executor_processor_bind: model::UpdateExecutorProcessorBind,
    pool: ShareData<db::ConnectionPool>,
) -> Result<(), CommonError> {
    use db::schema::{executor_processor, executor_processor_bind, task, task_bind};
    use delicate_utils_task::{TaskPackage, TaskUnit};
    use state::task::State;

    let conn = pool.get()?;
    let operation_log_pair_option = generate_operation_executor_processor_bind_modify_log(
        &req.get_session(),
        &executor_processor_bind,
    )
    .ok();

    let task_packages: Vec<(TaskPackage, (String, String))> =
        web::block::<_, _, diesel::result::Error>(move || {
            let task_packages: Vec<(TaskPackage, (String, String))> = task_bind::table
                .inner_join(executor_processor_bind::table.inner_join(executor_processor::table))
                .inner_join(task::table)
                .filter(task::status.eq(State::Enabled as i16))
                .select((
                    (
                        task::id,
                        task::command,
                        task::frequency,
                        task::cron_expression,
                        task::timeout,
                        task::maximum_parallel_runnable_num,
                    ),
                    (executor_processor::host, executor_processor::token),
                ))
                .load::<(TaskPackage, (String, String))>(&conn)?;

            diesel::update(&executor_processor_bind)
                .set(&executor_processor_bind)
                .execute(&conn)?;

            operation_log_pair_option
                .map(|operation_log_pair| operate_log(&conn, operation_log_pair));

            Ok(task_packages)
        })
        .await?;

    let remove_task_units: JoinAll<_> = task_packages
        .iter()
        .filter_map(|&(ref t, (ref host, ref token))| {
            let executor_host = "http://".to_string() + host + "/api/task/remove";
            TaskUnit::default()
                .set_task_id(t.id)
                .set_time(get_timestamp())
                .sign(Some(token))
                .map(|t| (t, executor_host))
                .ok()
        })
        .map(|(signed_task_unit, executor_host)| {
            RequestClient::default()
                .post(executor_host)
                .send_json(&signed_task_unit)
        })
        .collect::<Vec<_>>()
        .into_iter()
        .collect();

    let create_task_packages: JoinAll<_> = task_packages
        .into_iter()
        .filter_map(|(t, (host, token))| {
            let executor_host = "http://".to_string() + &host + "/api/task/create";
            t.sign(Some(&token)).map(|t| (t, executor_host)).ok()
        })
        .map(|(signed_task_package, executor_host)| {
            RequestClient::default()
                .post(executor_host)
                .send_json(&signed_task_package)
        })
        .collect::<Vec<_>>()
        .into_iter()
        .collect();

    join(remove_task_units, create_task_packages).await;
    Ok(())
}
#[post("/api/executor_processor_bind/delete")]
async fn delete_executor_processor_bind(
    req: HttpRequest,
    web::Json(model::ExecutorProcessorBindId {
        executor_processor_bind_id,
    }): web::Json<model::ExecutorProcessorBindId>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    use db::schema::executor_processor_bind::dsl::*;

    let operation_log_pair_option = generate_operation_executor_processor_bind_delete_log(
        &req.get_session(),
        &CommonTableRecord::default().set_id(executor_processor_bind_id),
    )
    .ok();

    // TODO: Check if there are associated tasks on the binding.
    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<usize>>::into(
            web::block(move || {
                operation_log_pair_option
                    .map(|operation_log_pair| operate_log(&conn, operation_log_pair));

                diesel::delete(executor_processor_bind.find(executor_processor_bind_id))
                    .execute(&conn)
            })
            .await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<usize>::error())
}
