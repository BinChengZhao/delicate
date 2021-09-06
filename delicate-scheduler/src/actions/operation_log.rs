use super::prelude::*;

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(show_operation_log)
        .service(show_operation_log_detail);
}

#[post("/api/operation_log/list")]
async fn show_operation_log(
    web::Json(query_params): web::Json<model::QueryParamsOperationLog>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<
            UnifiedResponseMessages<PaginateData<model::OperationLog>>,
        >::into(
            web::block::<_, _, diesel::result::Error>(move || {
                let query_builder = model::OperationLogQueryBuilder::query_all_columns();

                let operation_log = query_params
                    .clone()
                    .query_filter(query_builder)
                    .paginate(query_params.page)
                    .set_per_page(query_params.per_page)
                    .load::<model::OperationLog>(&conn)?;

                let per_page = query_params.per_page;
                let count_builder = model::OperationLogQueryBuilder::query_count();
                let count = query_params
                    .query_filter(count_builder)
                    .get_result::<i64>(&conn)?;

                let front_end_operation_log: Vec<model::OperationLog> =
                    operation_log.into_iter().collect();
                Ok(PaginateData::<model::OperationLog>::default()
                    .set_data_source(front_end_operation_log)
                    .set_page_size(per_page)
                    .set_total(count)
                    .set_state_desc::<state::operation_log::OperationType>())
            })
            .await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<PaginateData<model::OperationLog>>::error())
}

#[post("/api/operation_log/detail")]
async fn show_operation_log_detail(
    web::Json(query_params): web::Json<model::OperationLogId>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    use db::schema::operation_log_detail;

    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<
            UnifiedResponseMessages<Vec<model::OperationLogDetail>>,
        >::into(
            web::block::<_, _, diesel::result::Error>(move || {
                let operation_log_extend = operation_log_detail::table
                    .filter(
                        operation_log_detail::operation_log_id.eq(query_params.operation_log_id),
                    )
                    .load::<model::OperationLogDetail>(&conn)?;

                Ok(operation_log_extend)
            })
            .await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<Vec<model::OperationLogDetail>>::error())
}
