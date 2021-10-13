use super::prelude::*;

pub(crate) fn config_route(route: Route) -> Route {
    route
        .at("/api/operation_log/list", post(show_operation_log))
        .at("/api/operation_log/detail", post(show_operation_log_detail))
}

#[handler]

async fn show_operation_log(
    Json(query_params): Json<model::QueryParamsOperationLog>,
    pool: Data<&db::ConnectionPool>,
) -> impl IntoResponse {
    if let Ok(conn) = pool.get() {
        return Json(Into::<
            UnifiedResponseMessages<PaginateData<model::FrontEndOperationLog>>,
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

                let front_end_operation_log: Vec<model::FrontEndOperationLog> =
                    operation_log.into_iter().map(|log| log.into()).collect();
                Ok(PaginateData::<model::FrontEndOperationLog>::default()
                    .set_data_source(front_end_operation_log)
                    .set_page_size(per_page)
                    .set_total(count)
                    .set_state_desc::<state::operation_log::OperationType>())
            })
            .await,
        ));
    }

    Json(UnifiedResponseMessages::<
        PaginateData<model::FrontEndOperationLog>,
    >::error())
}
#[handler]

async fn show_operation_log_detail(
    Json(query_params): Json<model::OperationLogId>,
    pool: Data<&db::ConnectionPool>,
) -> impl IntoResponse {
    use db::schema::operation_log_detail;

    if let Ok(conn) = pool.get() {
        return Json(Into::<
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

    Json(UnifiedResponseMessages::<Vec<model::OperationLogDetail>>::error())
}
