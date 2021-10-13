// Front-end components api.

use super::prelude::*;

pub(crate) fn config_route(route: Route) -> Route {
    route
        .at("/api/binding/list", get(binding_list))
        .at("/api/executor/list", get(executor_list))
        .at("/api/permission/list", get(permission_list))
}

#[handler]
async fn binding_list(pool: Data<&db::ConnectionPool>) -> impl IntoResponse {
    use model::{BindingSelection, ExecutorProcessorBindQueryBuilder};

    if let Ok(conn) = pool.get() {
        return Json(
            Into::<UnifiedResponseMessages<Vec<model::BindingSelection>>>::into(
                web::block::<_, _, diesel::result::Error>(move || {
                    ExecutorProcessorBindQueryBuilder::query_binding_columns()
                        .load::<BindingSelection>(&conn)
                })
                .await,
            ),
        );
    }

    Json(UnifiedResponseMessages::<Vec<model::BindingSelection>>::error())
}

#[handler]
async fn executor_list(pool: Data<&db::ConnectionPool>) -> impl IntoResponse {
    use model::{ExecutorProcessorQueryBuilder, ExecutorSelection};

    if let Ok(conn) = pool.get() {
        return Json(
            Into::<UnifiedResponseMessages<Vec<model::ExecutorSelection>>>::into(
                web::block::<_, _, diesel::result::Error>(move || {
                    ExecutorProcessorQueryBuilder::query_selection_columns()
                        .load::<ExecutorSelection>(&conn)
                })
                .await,
            ),
        );
    }

    Json(UnifiedResponseMessages::<Vec<model::ExecutorSelection>>::error())
}

#[handler]
async fn permission_list(pool: Data<&db::ConnectionPool>) -> impl IntoResponse {
    use db::schema::casbin_rule;

    // TODO: Awaiting follow-up adjustment.
    if let Ok(conn) = pool.get() {
        let permissions = web::block::<_, _, diesel::result::Error>(move || {
            casbin_rule::table
                .select((casbin_rule::v1, casbin_rule::v2))
                .filter(casbin_rule::ptype.eq("p"))
                .filter(casbin_rule::v0.eq_any(&[
                    "task_admin",
                    "processor_admin",
                    "group_admin",
                    "user_admin",
                    "log_admin",
                ]))
                .load::<(String, String)>(&conn)
        })
        .await;

        let response_permissions: UnifiedResponseMessages<Vec<(String, String)>> =
            permissions.into();

        return Json(response_permissions);
    }
    Json(UnifiedResponseMessages::<Vec<(String, String)>>::error())
}
