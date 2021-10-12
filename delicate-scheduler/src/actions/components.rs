// Front-end components api.

use super::prelude::*;

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(binding_list)
        .service(executor_list)
        .service(casbin_test)
        .service(permission_list);
}

#[get("/api/binding/list")]
async fn binding_list(pool: ShareData<db::ConnectionPool>) -> HttpResponse {
    use model::{BindingSelection, ExecutorProcessorBindQueryBuilder};

    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<
            UnifiedResponseMessages<Vec<model::BindingSelection>>,
        >::into(
            web::block::<_, _, diesel::result::Error>(move || {
                ExecutorProcessorBindQueryBuilder::query_binding_columns()
                    .load::<BindingSelection>(&conn)
            })
            .await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<Vec<model::BindingSelection>>::error())
}

#[get("/api/executor/list")]
async fn executor_list(pool: ShareData<db::ConnectionPool>) -> HttpResponse {
    use model::{ExecutorProcessorQueryBuilder, ExecutorSelection};

    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<
            UnifiedResponseMessages<Vec<model::ExecutorSelection>>,
        >::into(
            web::block::<_, _, diesel::result::Error>(move || {
                ExecutorProcessorQueryBuilder::query_selection_columns()
                    .load::<ExecutorSelection>(&conn)
            })
            .await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<Vec<model::ExecutorSelection>>::error())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct CasbinDemo {
    name: String,
    permissions: Option<Vec<String>>,
    role: Option<String>,
    operation: isize,
}

// TODO: Remove it later.
#[post("/api/casbin/test")]
async fn casbin_test(
    enforcer: ShareData<RwLock<Enforcer>>,
    web::Json(CasbinDemo {
        name,
        permissions,
        role,
        operation,
    }): web::Json<CasbinDemo>,
) -> HttpResponse {
    match operation {
        1 => {
            enforcer
                .write()
                .await
                .add_permission_for_user(&name, permissions.expect(""))
                .await
                .expect("");
        }
        2 => {
            enforcer
                .write()
                .await
                .delete_permission_for_user(&name, permissions.expect(""))
                .await
                .expect("");
        }
        3 => {
            enforcer
                .write()
                .await
                .add_role_for_user(&name, &(role.expect("")), None)
                .await
                .expect("");
        }
        4 => {
            enforcer
                .write()
                .await
                .delete_role_for_user(&name, &(role.expect("")), None)
                .await
                .expect("");
        }

        5 => {
            let roles = enforcer.write().await.get_roles_for_user(&name, None);

            dbg!(roles);
        }

        6 => {
            let users = enforcer
                .write()
                .await
                .get_users_for_role(&(role.expect("")), None);

            dbg!(users);
        }

        _ => {}
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<()>::success())
}

#[get("/api/permission/list")]
async fn permission_list(pool: ShareData<db::ConnectionPool>) -> HttpResponse {
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

        return HttpResponse::Ok().json(UnifiedResponseMessages::success_with_data(
            response_permissions,
        ));
    }
    HttpResponse::Ok().json(UnifiedResponseMessages::<()>::error())
}
