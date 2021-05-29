use super::prelude::*;

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(show_executor_processors)
        .service(create_executor_processor)
        .service(update_executor_processor)
        .service(delete_executor_processor);
}

#[post("/api/executor_processor/create")]
async fn create_executor_processor(
    web::Json(executor_processor): web::Json<model::NewExecutorProcessor>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    use db::schema::executor_processor;

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
            UnifiedResponseMessages<model::PaginateExecutorProcessor>,
        >::into(
            web::block::<_, _, diesel::result::Error>(move || {
                let query_builder = model::ExecutorProcessorQueryBuilder::query_all_columns();

                let executor_processors = query_params
                    .clone()
                    .query_filter(query_builder)
                    .paginate(query_params.page)
                    .load::<model::ExecutorProcessor>(&conn)?;

                let per_page = query_params.per_page;
                let count_builder = model::ExecutorProcessorQueryBuilder::query_count();
                let count = query_params
                    .query_filter(count_builder)
                    .get_result::<i64>(&conn)?;

                Ok(
                    model::executor_processor::PaginateExecutorProcessor::default()
                        .set_tasks(executor_processors)
                        .set_per_page(per_page)
                        .set_total_page(count),
                )
            })
            .await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<model::PaginateTask>::error())
}

#[post("/api/executor_processor/update")]
async fn update_executor_processor(
    web::Json(executor_processor): web::Json<model::UpdateExecutorProcessor>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
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
    web::Json(model::ExecutorProcessorId {
        executor_processor_id,
    }): web::Json<model::ExecutorProcessorId>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    use db::schema::executor_processor::dsl::*;

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
) -> HttpResponse {
    HttpResponse::Ok().json(UnifiedResponseMessages::<()>::error())
}

async fn do_activate(
    conn: db::PoolConnection,
    executor_processor_id: i64,
) -> actix_web_error::Result<()> {
    use db::schema::executor_processor;

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
    let url = "http://".to_string() + &host + "/bind";
    let signed_scheduler = security::SignedBindRequest::default();
    let response = client
        .post(url)
        .send_json(&signed_scheduler)
        .await?
        .json::<security::BindResponse>()
        .await?;

    Ok(())
}
