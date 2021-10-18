// Front-end components api.

use crate::prelude::*;

pub(crate) fn route_config() -> Route {
    let route: Route = Route::new();
    route.at("/api/binding/list", get(binding_list))
}

#[handler]
async fn binding_list(pool: Data<&Arc<db::ConnectionPool>>) -> impl IntoResponse {
    use model::{BindingSelection, ExecutorProcessorBindQueryBuilder};

    if let Ok(conn) = pool.get() {
        return Json(
            Into::<UnifiedResponseMessages<Vec<model::BindingSelection>>>::into(
                spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
                    ExecutorProcessorBindQueryBuilder::query_binding_columns()
                        .load::<BindingSelection>(&conn)
                })
                .await
                .map(|r| r.into())
                .unwrap_or_else(|e| {
                    UnifiedResponseMessages::<Vec<model::BindingSelection>>::error()
                        .customized_error_msg(e.to_string())
                }),
            ),
        );
    }

    Json(UnifiedResponseMessages::<Vec<model::BindingSelection>>::error())
}
