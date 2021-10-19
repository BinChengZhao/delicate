use super::prelude::*;

pub(crate) fn route_config() -> Route {
    Route::new()
        .at("/api/operation_log/list", post(show_operation_log))
        .at("/api/operation_log/detail", post(show_operation_log_detail))
}

#[handler]

async fn show_operation_log(
    Json(query_params): Json<model::QueryParamsOperationLog>,
    pool: Data<&Arc<db::ConnectionPool>>,
) -> impl IntoResponse {
    if let Ok(conn) = pool.get() {
        let f_result = spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
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
        .await;

        let page = f_result
            .map(|page_result| {
                Into::<UnifiedResponseMessages<PaginateData<model::FrontEndOperationLog>>>::into(
                    page_result,
                )
            })
            .unwrap_or_else(|e| {
                UnifiedResponseMessages::<PaginateData<model::FrontEndOperationLog>>::error()
                    .customized_error_msg(e.to_string())
            });
        return Json(page);
    }

    Json(UnifiedResponseMessages::<
        PaginateData<model::FrontEndOperationLog>,
    >::error())
}
#[handler]

async fn show_operation_log_detail(
    Json(query_params): Json<model::OperationLogId>,
    pool: Data<&Arc<db::ConnectionPool>>,
) -> impl IntoResponse {
    use db::schema::operation_log_detail;

    if let Ok(conn) = pool.get() {
        let f_result = spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
            let operation_log_extend = operation_log_detail::table
                .filter(operation_log_detail::operation_log_id.eq(query_params.operation_log_id))
                .load::<model::OperationLogDetail>(&conn)?;

            Ok(operation_log_extend)
        })
        .await;

        let log_detail = f_result
            .map(|log_detail_result| {
                Into::<UnifiedResponseMessages<Vec<model::OperationLogDetail>>>::into(
                    log_detail_result,
                )
            })
            .unwrap_or_else(|e| {
                UnifiedResponseMessages::<Vec<model::OperationLogDetail>>::error()
                    .customized_error_msg(e.to_string())
            });
        return Json(log_detail);
    }

    Json(UnifiedResponseMessages::<Vec<model::OperationLogDetail>>::error())
}
