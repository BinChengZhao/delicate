// Front-end components api.

use crate::prelude::*;

pub(crate) fn route_config() -> Route {
    Route::new().at("/api/executor/list", get(executor_list))
}

#[handler]
async fn executor_list(pool: Data<&Arc<db::ConnectionPool>>) -> impl IntoResponse {
    use model::{ExecutorProcessorQueryBuilder, ExecutorSelection};

    if let Ok(conn) = pool.get() {
        return Json(Into::<UnifiedResponseMessages<Vec<model::ExecutorSelection>>>::into(
            spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
                ExecutorProcessorQueryBuilder::query_selection_columns()
                    .load::<ExecutorSelection>(&conn)
            })
            .await
            .map(|r| r.into())
            .unwrap_or_else(|e| {
                UnifiedResponseMessages::<Vec<model::ExecutorSelection>>::error()
                    .customized_error_msg(e.to_string())
            }),
        ));
    }

    Json(UnifiedResponseMessages::<Vec<model::ExecutorSelection>>::error())
}
