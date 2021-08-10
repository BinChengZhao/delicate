use super::prelude::*;
use db::schema::executor_processor;
pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(show_executor_processors)
        .service(create_executor_processor)
        .service(update_executor_processor)
        .service(delete_executor_processor)
        .service(activate_executor_processor);
}

#[post("/api/executor_processor/create")]
async fn create_executor_processor(
    req: HttpRequest,
    web::Json(executor_processor): web::Json<model::NewExecutorProcessor>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    let operation_log_pair_option =
        generate_operation_executor_processor_addtion_log(&req.get_session(), &executor_processor)
            .ok();
    send_option_operation_log_pair(operation_log_pair_option).await;

    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<usize>>::into(
            web::block(move || {
                diesel::insert_into(executor_processor::table)
                    .values(&executor_processor)
                    .execute(&conn)
            })
            .await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<()>::error())
}

#[post("/api/executor_processor/list")]
async fn show_executor_processors(
    web::Json(query_params): web::Json<model::QueryParamsExecutorProcessor>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<
            UnifiedResponseMessages<PaginateData<model::ExecutorProcessor>>,
        >::into(
            web::block::<_, _, diesel::result::Error>(move || {
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
                    .set_total(count))
            })
            .await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<
        PaginateData<model::ExecutorProcessor>,
    >::error())
}

#[post("/api/executor_processor/update")]
async fn update_executor_processor(
    req: HttpRequest,
    web::Json(executor_processor): web::Json<model::UpdateExecutorProcessor>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    let operation_log_pair_option =
        generate_operation_executor_processor_modify_log(&req.get_session(), &executor_processor)
            .ok();
    send_option_operation_log_pair(operation_log_pair_option).await;

    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<usize>>::into(
            web::block(move || {
                diesel::update(&executor_processor)
                    .set(&executor_processor)
                    .execute(&conn)
            })
            .await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<usize>::error())
}
#[post("/api/executor_processor/delete")]
async fn delete_executor_processor(
    req: HttpRequest,
    web::Json(model::ExecutorProcessorId {
        executor_processor_id,
    }): web::Json<model::ExecutorProcessorId>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    use db::schema::executor_processor::dsl::*;

    let operation_log_pair_option = generate_operation_executor_processor_delete_log(
        &req.get_session(),
        &CommonTableRecord::default().set_id(executor_processor_id),
    )
    .ok();
    send_option_operation_log_pair(operation_log_pair_option).await;

    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<usize>>::into(
            web::block(move || {
                diesel::delete(executor_processor.find(executor_processor_id)).execute(&conn)
            })
            .await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<usize>::error())
}

// Update `status` and `token`.

#[post("/api/executor_processor/activate")]
async fn activate_executor_processor(
    web::Json(model::ExecutorProcessorId {
        executor_processor_id,
    }): web::Json<model::ExecutorProcessorId>,
    pool: ShareData<db::ConnectionPool>,
    scheduler: SharedSchedulerMetaInfo,
) -> HttpResponse {
    let uniform_data: UnifiedResponseMessages<()> =
        do_activate(pool, executor_processor_id, scheduler)
            .await
            .into();
    HttpResponse::Ok().json(uniform_data)
}
async fn do_activate(
    pool: ShareData<db::ConnectionPool>,
    executor_processor_id: i64,
    scheduler: SharedSchedulerMetaInfo,
) -> Result<(), CommonError> {
    let bind_info = activate_executor(pool.get()?, executor_processor_id, scheduler).await?;
    activate_executor_row(pool.get()?, executor_processor_id, bind_info).await?;
    Ok(())
}
async fn activate_executor(
    conn: db::PoolConnection,
    executor_processor_id: i64,
    scheduler: SharedSchedulerMetaInfo,
) -> Result<service_binding::BindResponse, CommonError> {
    let query = web::block::<_, model::UpdateExecutorProcessor, diesel::result::Error>(move || {
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
    .await?;

    let model::UpdateExecutorProcessor {
        id,
        name,
        host,
        machine_id,
        ..
    }: model::UpdateExecutorProcessor = query;

    let client = RequestClient::default();
    let url = "http://".to_string() + &host + "/api/executor/bind";

    let private_key = scheduler.get_app_security_key();
    let scheduler_host = scheduler.get_app_host_name().clone();
    let signed_scheduler = service_binding::BindRequest::default()
        .set_scheduler_host(scheduler_host)
        .set_executor_processor_id(id)
        .set_executor_processor_host(host)
        .set_executor_processor_name(name)
        .set_executor_machine_id(machine_id)
        .set_time(get_timestamp())
        .sign(private_key)?;

    let response: Result<service_binding::EncryptedBindResponse, CommonError> = client
        .post(url)
        .send_json(&signed_scheduler)
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

    web::block::<_, usize, diesel::result::Error>(move || {
        diesel::update(executor_processor.find(executor_processor_id))
            .set((
                token.eq(&bind_info.token.unwrap_or_default()),
                status.eq(state::executor_processor::State::Enabled as i16),
            ))
            .execute(&conn)
    })
    .await?;

    Ok(())
}
