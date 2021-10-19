// Front-end components api.

use crate::prelude::*;

pub(crate) fn route_config() -> Route {
    let route: Route = Route::new();
    route.at("/api/permission/list", get(permission_list))
}

#[handler]
async fn permission_list(pool: Data<&Arc<db::ConnectionPool>>) -> impl IntoResponse {
    use db::schema::casbin_rule;

    // TODO: Awaiting follow-up adjustment.
    if let Ok(conn) = pool.get() {
        let permissions = spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
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
            permissions.map(|r| r.into()).unwrap_or_else(|e| {
                UnifiedResponseMessages::<Vec<(String, String)>>::error()
                    .customized_error_msg(e.to_string())
            });

        return Json(response_permissions);
    }
    Json(UnifiedResponseMessages::<Vec<(String, String)>>::error())
}
