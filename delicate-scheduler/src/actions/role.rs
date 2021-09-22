use super::prelude::*;
use db::model::casbin_rule::RoleId;

#[allow(dead_code)]
pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(list).service(permission_detail).service(users);
}

#[get("/api/role/list")]
async fn list() -> HttpResponse {
    HttpResponse::Ok().json(UnifiedResponseMessages::<[&'static str; 7]>::success_with_data(ROLES))
}

#[post("/api/role/permission_detail")]
async fn permission_detail(
    enforcer: ShareData<RwLock<Enforcer>>,
    web::Json(RoleId { role_id }): web::Json<RoleId>,
) -> HttpResponse {
    // [
    //   ["role_name", "business", "action"]
    // ]
    if let Some(role_name) = ROLES.get(role_id) {
        let permissions = enforcer
            .read()
            .await
            .get_filtered_grouping_policy(0, vec![role_name.to_string()]);
        return HttpResponse::Ok()
            .json(UnifiedResponseMessages::<Vec<Vec<String>>>::success_with_data(permissions));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<()>::error())
}

#[post("/api/role/users")]
async fn users(
    enforcer: ShareData<RwLock<Enforcer>>,
    web::Json(RoleId { role_id }): web::Json<RoleId>,
) -> HttpResponse {
    if let Some(role_name) = ROLES.get(role_id) {
        let users = enforcer.read().await.get_users_for_role(&role_name, None);
        return HttpResponse::Ok().json(UnifiedResponseMessages::<Vec<String>>::success_with_data(
            users,
        ));
    }
    HttpResponse::Ok().json(UnifiedResponseMessages::<()>::error())
}
