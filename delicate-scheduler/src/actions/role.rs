use super::prelude::*;
use db::model::casbin_rule::RoleId;

pub(crate) fn route_config() -> Route {
    let route: Route = Route::new();
    route
        .at("/api/role/list", get(list))
        .at("/api/role/permission_detail", post(permission_detail))
        .at("/api/role/users", post(users))
}

#[handler]

async fn list() -> impl IntoResponse {
    Json(UnifiedResponseMessages::<[&'static str; 7]>::success_with_data(ROLES))
}

#[handler]
async fn permission_detail(
    enforcer: Data<&Arc<RwLock<Enforcer>>>,
    Json(RoleId { role_id }): Json<RoleId>,
) -> impl IntoResponse {
    // [
    //   ["role_name", "business", "action"]
    // ]
    if let Some(role_name) = ROLES.get(role_id) {
        let permissions = enforcer
            .read()
            .await
            .get_filtered_policy(0, vec![role_name.to_string()]);
        return Json(UnifiedResponseMessages::<Vec<Vec<String>>>::success_with_data(permissions));
    }

    Json(UnifiedResponseMessages::<Vec<Vec<String>>>::error())
}

#[handler]

async fn users(
    enforcer: Data<&Arc<RwLock<Enforcer>>>,
    Json(RoleId { role_id }): Json<RoleId>,
) -> impl IntoResponse {
    if let Some(role_name) = ROLES.get(role_id) {
        let users = enforcer.read().await.get_users_for_role(role_name, None);
        return Json(UnifiedResponseMessages::<Vec<String>>::success_with_data(
            users,
        ));
    }
    Json(UnifiedResponseMessages::<Vec<String>>::error())
}
