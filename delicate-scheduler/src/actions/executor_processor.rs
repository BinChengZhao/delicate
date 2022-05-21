use super::prelude::*;
use db::schema::executor_processor;

pub(crate) fn route_config() -> Route {
    Route::new()
        .at(
            "/api/executor_processor/list",
            post(show_executor_processors),
        )
        .at(
            "/api/executor_processor/create",
            post(create_executor_processor),
        )
        .at(
            "/api/executor_processor/update",
            post(update_executor_processor),
        )
        .at(
            "/api/executor_processor/delete",
            post(delete_executor_processor),
        )
        .at(
            "/api/executor_processor/activate",
            post(activate_executor_processor),
        )
}

#[handler]
async fn create_executor_processor(
    req: &Request,
    Json(executor_processor): Json<model::NewExecutorProcessor>,
    pool: Data<&Arc<db::ConnectionPool>>,
) -> impl IntoResponse {
    let operation_log_pair_option =
        generate_operation_executor_processor_addtion_log(req.get_session(), &executor_processor)
            .ok();
    send_option_operation_log_pair(operation_log_pair_option).await;

    if let Ok(conn) = pool.get() {
        let f_result = spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
            diesel::insert_into(executor_processor::table)
                .values(&executor_processor)
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
async fn show_executor_processors(
    Json(query_params): Json<model::QueryParamsExecutorProcessor>,
    pool: Data<&Arc<db::ConnectionPool>>,
) -> impl IntoResponse {
    if let Ok(conn) = pool.get() {
        let f_result = spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
            let query_builder = model::ExecutorProcessorQueryBuilder::query_all_columns();

            let executor_processors = query_params
                .clone()
                .query_filter(query_builder)
                .paginate(query_params.page)
                .set_per_page(query_params.per_page)
                .load::<model::ExecutorProcessor>(&conn)?;

            let per_page = query_params.per_page;
            let count_builder = model::ExecutorProcessorQueryBuilder::query_count();
            let count = query_params
                .query_filter(count_builder)
                .get_result::<i64>(&conn)?;

            Ok(PaginateData::<model::ExecutorProcessor>::default()
                .set_data_source(executor_processors)
                .set_page_size(per_page)
                .set_total(count)
                .set_state_desc::<state::executor_processor::State>())
        })
        .await;
        let processor = f_result
            .map(|processor_result| {
                Into::<UnifiedResponseMessages<PaginateData<model::ExecutorProcessor>>>::into(
                    processor_result,
                )
            })
            .unwrap_or_else(|e| {
                UnifiedResponseMessages::<PaginateData<model::ExecutorProcessor>>::error()
                    .customized_error_msg(e.to_string())
            });
        return Json(processor);
    }

    Json(UnifiedResponseMessages::<
        PaginateData<model::ExecutorProcessor>,
    >::error())
}

#[handler]
async fn update_executor_processor(
    req: &Request,
    Json(executor_processor): Json<model::UpdateExecutorProcessor>,
    pool: Data<&Arc<db::ConnectionPool>>,
) -> impl IntoResponse {
    let operation_log_pair_option =
        generate_operation_executor_processor_modify_log(req.get_session(), &executor_processor)
            .ok();
    send_option_operation_log_pair(operation_log_pair_option).await;

    if let Ok(conn) = pool.get() {
        let f_result = spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
            diesel::update(&executor_processor)
                .set(&executor_processor)
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
async fn delete_executor_processor(
    req: &Request,
    Json(model::ExecutorProcessorId {
        executor_processor_id,
    }): Json<model::ExecutorProcessorId>,
    pool: Data<&Arc<db::ConnectionPool>>,
) -> impl IntoResponse {
    use db::schema::executor_processor::dsl::*;

    let operation_log_pair_option = generate_operation_executor_processor_delete_log(
        req.get_session(),
        &CommonTableRecord::default().set_id(executor_processor_id),
    )
    .ok();
    send_option_operation_log_pair(operation_log_pair_option).await;

    if let Ok(conn) = pool.get() {
        let f_result = spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
            diesel::delete(executor_processor.find(executor_processor_id)).execute(&conn)
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
async fn activate_executor_processor(
    req: &Request,
    Json(model::ExecutorProcessorId {
        executor_processor_id,
    }): Json<model::ExecutorProcessorId>,
    pool: Data<&Arc<db::ConnectionPool>>,
    scheduler: Data<&Arc<SchedulerMetaInfo>>,
) -> impl IntoResponse {
    let uniform_data: UnifiedResponseMessages<()> =
        do_activate(req, pool, executor_processor_id, scheduler)
            .await
            .into();
    Json(uniform_data)
}
async fn do_activate(
    req: &Request,
    pool: Data<&Arc<db::ConnectionPool>>,
    executor_processor_id: i64,
    scheduler: Data<&Arc<SchedulerMetaInfo>>,
) -> Result<(), CommonError> {
    let bind_info = activate_executor(req, pool.get()?, executor_processor_id, scheduler).await?;
    activate_executor_row(pool.get()?, executor_processor_id, bind_info).await?;
    Ok(())
}
async fn activate_executor(
    req: &Request,
    conn: db::PoolConnection,
    executor_processor_id: i64,
    scheduler: Data<&Arc<SchedulerMetaInfo>>,
) -> Result<service_binding::BindResponse, CommonError> {
    let query = spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
        executor_processor::table
            .find(executor_processor_id)
            .select((
                executor_processor::id,
                executor_processor::name,
                executor_processor::description,
                executor_processor::host,
                executor_processor::machine_id,
                executor_processor::tag,
            ))
            .first(&conn)
    })
    .await??;

    let model::UpdateExecutorProcessor {
        id,
        name,
        host,
        machine_id,
        ..
    }: model::UpdateExecutorProcessor = query;

    let request_client = req
        .extensions()
        .get::<RequestClient>()
        .expect("Missing Components `RequestClient`");
    let url = "http://".to_string() + (host.deref()) + "/api/executor/bind";

    let private_key = scheduler.get_app_security_key();
    let scheduler_host = scheduler.get_app_host_name().clone();
    let signed_scheduler = service_binding::BindRequest::default()
        .set_scheduler_host(scheduler_host)
        .set_executor_processor_id(id)
        .set_executor_processor_host(host)
        .set_executor_processor_name(name)
        .set_executor_machine_id(machine_id)
        .set_time(timestamp())
        .sign(private_key)?;

    let response: Result<service_binding::EncryptedBindResponse, CommonError> = request_client
        .post(url)
        .json(&signed_scheduler)
        .send()
        .await?
        .json::<UnifiedResponseMessages<service_binding::EncryptedBindResponse>>()
        .await?
        .into();

    Ok(response?.decrypt_self(private_key)?)
}

async fn activate_executor_row(
    conn: db::PoolConnection,
    executor_processor_id: i64,
    bind_info: service_binding::BindResponse,
) -> Result<(), CommonError> {
    use db::schema::executor_processor::dsl::{executor_processor, status, token};

    // TODO:
    // Consider caching tokens to be used when collecting executor-events, and health checks.
    // This will avoid querying the database.
    // However, cached record operations cannot be placed in the context of the operation db update token.

    spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
        diesel::update(executor_processor.find(executor_processor_id))
            .set((
                token.eq(&bind_info.token.unwrap_or_default()),
                status.eq(state::executor_processor::State::Enabled as i16),
            ))
            .execute(&conn)
    })
    .await??;

    Ok(())
}
