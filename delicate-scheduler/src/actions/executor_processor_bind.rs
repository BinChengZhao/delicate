use super::prelude::*;

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(show_executor_processor_binds)
        .service(create_executor_processor_bind)
        .service(update_executor_processor_bind)
        .service(delete_executor_processor_bind);
}

#[post("/api/executor_processor_bind/create")]
async fn create_executor_processor_bind(
    web::Json(executor_processor_bind): web::Json<model::NewExecutorProcessorBind>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    use db::schema::executor_processor_bind;

    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<usize>>::into(
            web::block(move || {
                diesel::insert_into(executor_processor_bind::table)
                    .values(&executor_processor_bind)
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
            UnifiedResponseMessages<model::PaginateExecutorProcessorBind>,
        >::into(
            web::block::<_, _, diesel::result::Error>(move || {
                let query_builder = model::ExecutorProcessorBindQueryBuilder::query_all_columns();

                let executor_processor_binds = query_params
                    .clone()
                    .query_filter(query_builder)
                    .paginate(query_params.page)
                    .load::<model::ExecutorProcessorBind>(&conn)?;

                let per_page = query_params.per_page;
                let count_builder = model::ExecutorProcessorBindQueryBuilder::query_count();
                let count = query_params
                    .query_filter(count_builder)
                    .get_result::<i64>(&conn)?;

                Ok(
                    model::executor_processor_bind::PaginateExecutorProcessorBind::default()
                        .set_tasks(executor_processor_binds)
                        .set_per_page(per_page)
                        .set_total_page(count),
                )
            })
            .await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<model::PaginateTask>::error())
}

#[post("/api/executor_processor_bind/update")]
async fn update_executor_processor_bind(
    web::Json(executor_processor_bind): web::Json<model::UpdateExecutorProcessorBind>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<usize>>::into(
            web::block(move || {
                diesel::update(&executor_processor_bind)
                    .set(&executor_processor_bind)
                    .execute(&conn)
            })
            .await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<usize>::error())
}
#[post("/api/executor_processor_bind/delete")]
async fn delete_executor_processor_bind(
    web::Json(model::ExecutorProcessorBindId {
        executor_processor_bind_id,
    }): web::Json<model::ExecutorProcessorBindId>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    use db::schema::executor_processor_bind::dsl::*;

    // TODO: Manybe soft-delete.
    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<usize>>::into(
            web::block(move || {
                diesel::delete(executor_processor_bind.find(executor_processor_bind_id))
                    .execute(&conn)
            })
            .await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<usize>::error())
}
