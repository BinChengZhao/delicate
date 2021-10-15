use super::prelude::*;

pub(crate) fn config_route(route: Route) -> Route {
    route
        .at(
            "/api/executor_processor_bind/list",
            post(show_executor_processor_binds),
        )
        .at(
            "/api/executor_processor_bind/create",
            post(create_executor_processor_bind),
        )
        .at(
            "/api/executor_processor_bind/update",
            post(update_executor_processor_bind),
        )
        .at(
            "/api/executor_processor_bind/delete",
            post(delete_executor_processor_bind),
        )
}

#[handler]

async fn create_executor_processor_bind(
    req: &Request,
    Json(executor_processor_binds): Json<model::NewExecutorProcessorBinds>,
    pool: Data<&db::ConnectionPool>,
) -> impl IntoResponse {
    use db::schema::executor_processor_bind;

    let operation_log_pair_option = generate_operation_executor_processor_bind_addtion_log(
        &req.get_session(),
        &executor_processor_binds,
    )
    .ok();
    send_option_operation_log_pair(operation_log_pair_option).await;

    if let Ok(conn) = pool.get() {
        let f_result = spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
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

            diesel::insert_into(executor_processor_bind::table)
                .values(&new_binds[..])
                .execute(&conn)
        })
        .await;

        let count = f_result
            .map(Into::<UnifiedResponseMessages<usize>>::into)
            .unwrap_or_else(|e| {
                UnifiedResponseMessages::<usize>::error().customized_error_msg(e.to_string())
            });
        return Json(count);
    }

    Json(UnifiedResponseMessages::<usize>::error())
}

#[handler]

async fn show_executor_processor_binds(
    Json(query_params): Json<model::QueryParamsExecutorProcessorBind>,
    pool: Data<&db::ConnectionPool>,
) -> impl IntoResponse {
    if let Ok(conn) = pool.get() {
        let f_result = spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
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
        .await;

        let binds = f_result
            .map(|binds_result| {
                Into::<UnifiedResponseMessages<PaginateData<model::ExecutorProcessorBind>>>::into(
                    binds_result,
                )
            })
            .unwrap_or_else(|e| {
                UnifiedResponseMessages::<PaginateData<model::ExecutorProcessorBind>>::error()
                    .customized_error_msg(e.to_string())
            });
        return Json(binds).into_response();
    }

    Json(UnifiedResponseMessages::<PaginateData<()>>::error()).into_response()
}

#[handler]

async fn update_executor_processor_bind(
    req: &Request,
    Json(executor_processor_bind): Json<model::UpdateExecutorProcessorBind>,
    pool: Data<&db::ConnectionPool>,
) -> impl IntoResponse {
    return Json(Into::<UnifiedResponseMessages<()>>::into(
        pre_update_executor_processor_bind(req, executor_processor_bind, pool).await,
    ));
}

async fn pre_update_executor_processor_bind(
    req: &Request,
    executor_processor_bind: model::UpdateExecutorProcessorBind,
    pool: Data<&db::ConnectionPool>,
) -> Result<(), CommonError> {
    use db::schema::{executor_processor, executor_processor_bind, task, task_bind};
    use delicate_utils_task::{TaskPackage, TaskUnit};
    use state::task::State;

    let conn = pool.get()?;
    let executor_processor_bind_id = executor_processor_bind.id;
    let executor_processor_bind_executor_id = executor_processor_bind.executor_id;
    let request_client = req
        .extensions()
        .get::<RequestClient>()
        .expect("Missing Components `RequestClient`");

    let operation_log_pair_option = generate_operation_executor_processor_bind_modify_log(
        &req.get_session(),
        &executor_processor_bind,
    )
    .ok();
    send_option_operation_log_pair(operation_log_pair_option).await;

    let (older_executor_id, older_executor_host, older_executor_token) =
        spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
            let older_executor = executor_processor_bind::table
                .inner_join(executor_processor::table)
                .filter(executor_processor_bind::id.eq(executor_processor_bind.id))
                .select((
                    executor_processor_bind::executor_id,
                    executor_processor::host,
                    executor_processor::token,
                ))
                .first::<(i64, String, String)>(&conn)?;

            diesel::update(&executor_processor_bind)
                .set(&executor_processor_bind)
                .execute(&conn)?;
            Ok(older_executor)
        })
        .await??;

    if older_executor_id == executor_processor_bind_executor_id {
        return Ok(());
    }

    // Task migration needs to be performed only when `executor_id` is modified.
    let conn = pool.get()?;
    let task_packages: Vec<(TaskPackage, (String, String))> =
        spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
            let task_packages: Vec<(TaskPackage, (String, String))> = task_bind::table
                .inner_join(executor_processor_bind::table.inner_join(executor_processor::table))
                .inner_join(task::table)
                .filter(task::status.eq(State::Enabled as i16))
                .filter(executor_processor_bind::id.eq(executor_processor_bind_id))
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

            Ok(task_packages)
        })
        .await??;

    let task_ids = task_packages.iter().map(|&(ref t, _)| t.id);

    let remove_task_units: JoinAll<_> = task_ids
        .filter_map(|task_id| {
            let executor_host =
                "http://".to_string() + (older_executor_host.deref()) + "/api/task/remove";
            TaskUnit::default()
                .set_task_id(task_id)
                .set_time(get_timestamp())
                .sign(Some(older_executor_token.deref()))
                .map(|t| (t, executor_host))
                .ok()
        })
        .map(|(signed_task_unit, executor_host)| {
            request_client
                .post(executor_host)
                .json(&signed_task_unit)
                .send()
        })
        .collect();

    let create_task_packages: JoinAll<_> = task_packages
        .into_iter()
        .filter_map(|(t, (host, token))| {
            let executor_host = "http://".to_string() + (host.deref()) + "/api/task/create";
            t.sign(Some(&token)).map(|t| (t, executor_host)).ok()
        })
        .map(|(signed_task_package, executor_host)| {
            request_client
                .post(executor_host)
                .json(&signed_task_package)
                .send()
        })
        .collect();

    join(
        handle_response::<_, UnifiedResponseMessages<()>>(remove_task_units),
        handle_response::<_, UnifiedResponseMessages<()>>(create_task_packages),
    )
    .await;
    Ok(())
}

#[handler]
async fn delete_executor_processor_bind(
    req: &Request,
    Json(model::ExecutorProcessorBindId {
        executor_processor_bind_id,
    }): Json<model::ExecutorProcessorBindId>,
    pool: Data<&db::ConnectionPool>,
) -> impl IntoResponse {
    use db::schema::executor_processor_bind::dsl::*;

    let operation_log_pair_option = generate_operation_executor_processor_bind_delete_log(
        &req.get_session(),
        &CommonTableRecord::default().set_id(executor_processor_bind_id),
    )
    .ok();
    send_option_operation_log_pair(operation_log_pair_option).await;

    // TODO: Check if there are associated tasks on the binding.
    if let Ok(conn) = pool.get() {
        let f_result = spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
            diesel::delete(executor_processor_bind.find(executor_processor_bind_id)).execute(&conn)
        })
        .await;

        let count = f_result
            .map(Into::<UnifiedResponseMessages<usize>>::into)
            .unwrap_or_else(|e| {
                UnifiedResponseMessages::<usize>::error().customized_error_msg(e.to_string())
            });

        return Json(count);
    }

    Json(UnifiedResponseMessages::<usize>::error())
}
