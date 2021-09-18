use super::prelude::*;
use db::model::casbin_rule::RoleName;

#[allow(dead_code)]
pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(list).service(permission_detail).service(users);
}

#[get("/api/role/list")]
async fn list() -> HttpResponse {
    const ROLES: [&str; 8] = [
        "developer",
        "task_admin",
        "processor_admin",
        "group_admin",
        "user_admin",
        "log_admin",
        "team_leader",
        "super_admin",
    ];

    HttpResponse::Ok().json(UnifiedResponseMessages::<[&'static str; 8]>::success_with_data(ROLES))
}

#[post("/api/role/permission_detail")]
async fn permission_detail(
    enforcer: ShareData<RwLock<Enforcer>>,
    web::Json(RoleName { role_name }): web::Json<RoleName>,
) -> HttpResponse {
    // [
    //   ["role_name", "business", "action"]
    // ]
    let permissions = enforcer
        .read()
        .await
        .get_filtered_grouping_policy(0, vec![role_name]);
    HttpResponse::Ok()
        .json(UnifiedResponseMessages::<Vec<Vec<String>>>::success_with_data(permissions))
}

#[post("/api/role/users")]
async fn users(
    enforcer: ShareData<RwLock<Enforcer>>,
    web::Json(RoleName { role_name }): web::Json<RoleName>,
) -> HttpResponse {
    let users = enforcer.read().await.get_users_for_role(&role_name, None);
    HttpResponse::Ok().json(UnifiedResponseMessages::<Vec<String>>::success_with_data(
        users,
    ))
}
