use super::prelude::*;
use model::schema::{user, user_auth, user_login_log};
use model::user::{
    get_encrypted_certificate_by_raw_certificate, UserAndPermissions, UserAndRoles, UserName,
};

pub(crate) fn config_route(route: Route) -> Route {
    route
        .at("/api/user/create", post(create_user))
        .at("/api/user/list", post(show_users))
        .at("/api/user/update", post(update_user))
        .at("/api/user/delete", post(delete_user))
        // FIXME:
        // .at("/api/user/login", post(login_user))
        // .at("/api/user/logout", post(logout_user))
        // .at("/api/user/check", post(check_user))
        .at("/api/user/change_password", post(change_password))
        .at("/api/user/roles", post(roles))
        .at("/api/user/permissions", post(permissions))
        .at("/api/user/append_permission", post(append_permission))
        .at("/api/user/delete_permission", post(delete_permission))
        .at("/api/user/append_role", post(append_role))
        .at("/api/user/delete_role", post(delete_role))
}

#[handler]

async fn create_user(
    req: &Request,
    Json(user): Json<model::QueryNewUser>,
    pool: Data<&db::ConnectionPool>,
) -> impl IntoResponse {
    let validate_result: Result<(), ValidationErrors> = user.validate();
    if validate_result.is_err() {
        return Json(Into::<UnifiedResponseMessages<()>>::into(validate_result));
    }

    let new_user = Into::<model::NewUser>::into(&user);

    // FIXME:

    // let operation_log_pair_option =
    //     generate_operation_user_addtion_log(&req.get_session(), &new_user).ok();
    // send_option_operation_log_pair(operation_log_pair_option).await;

    if let Ok(conn) = pool.get() {
        let f_result = spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
            conn.transaction(|| {
                diesel::insert_into(user::table)
                    .values(&new_user)
                    .execute(&conn)?;

                let last_id = diesel::select(db::last_insert_id).get_result::<u64>(&conn)?;

                let user_auths: model::NewUserAuths =
                    From::<(model::QueryNewUser, u64)>::from((user, last_id));

                diesel::insert_into(user_auth::table)
                    .values(&user_auths.0[..])
                    .execute(&conn)?;

                Ok(())
            })
        })
        .await;

        let resp = f_result
            .map(|resp_result| Into::<UnifiedResponseMessages<()>>::into(resp_result))
            .unwrap_or_else(|e| {
                UnifiedResponseMessages::<()>::error().customized_error_msg(e.to_string())
            });
        return Json(resp);
    }

    Json(UnifiedResponseMessages::<()>::error())
}

#[handler]

async fn show_users(
    Json(query_params): Json<model::QueryParamsUser>,
    pool: Data<&db::ConnectionPool>,
) -> impl IntoResponse {
    if let Ok(conn) = pool.get() {
        let f_result = spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
            let query_builder = model::UserQueryBuilder::query_all_columns();

            let users = query_params
                .clone()
                .query_filter(query_builder)
                .paginate(query_params.page)
                .set_per_page(query_params.per_page)
                .load::<model::User>(&conn)?;

            let per_page = query_params.per_page;
            let count_builder = model::UserQueryBuilder::query_count();
            let count = query_params
                .query_filter(count_builder)
                .get_result::<i64>(&conn)?;

            Ok(PaginateData::<model::User>::default()
                .set_data_source(users)
                .set_page_size(per_page)
                .set_total(count))
        })
        .await;

        let page = f_result
            .map(|page_result| {
                Into::<UnifiedResponseMessages<PaginateData<model::User>>>::into(page_result)
            })
            .unwrap_or_else(|e| {
                UnifiedResponseMessages::<PaginateData<model::User>>::error()
                    .customized_error_msg(e.to_string())
            });
        return Json(page);
    }

    Json(UnifiedResponseMessages::<PaginateData<model::User>>::error())
}

#[handler]

async fn update_user(
    req: &Request,
    Json(user_value): Json<model::UpdateUser>,
    pool: Data<&db::ConnectionPool>,
) -> impl IntoResponse {
    // FIXME:

    // let operation_log_pair_option =
    //     generate_operation_user_modify_log(&req.get_session(), &user_value).ok();
    // send_option_operation_log_pair(operation_log_pair_option).await;

    if let Ok(conn) = pool.get() {
        let f_result = spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
            diesel::update(&user_value).set(&user_value).execute(&conn)
        })
        .await;

        let count = f_result
            .map(|count_result| Into::<UnifiedResponseMessages<usize>>::into(count_result))
            .unwrap_or_else(|e| {
                UnifiedResponseMessages::<usize>::error().customized_error_msg(e.to_string())
            });
        return Json(count);
    }

    Json(UnifiedResponseMessages::<usize>::error())
}

#[handler]

async fn change_password(
    req: &Request,
    Json(user_value): Json<model::UserChangePassword>,
    pool: Data<&db::ConnectionPool>,
) -> impl IntoResponse {
    // FIXME:

    // let session = req.get_session();
    // let user_id = session
    //     .get::<u64>("user_id")
    //     .unwrap_or_default()
    //     .unwrap_or_default();

    // if let Ok(conn) = pool.get() {
    //     return Json(Into::<UnifiedResponseMessages<usize>>::into(
    //         spawn_blocking::<_,Result<_, diesel::result::Error>>(move || {
    //             let user_auth_id = user_auth::table
    //                 .select(user_auth::id)
    //                 .filter(user_auth::user_id.eq(&user_id))
    //                 .filter(user_auth::identity_type.eq(user_value.identity_type))
    //                 .filter(user_auth::certificate.eq(
    //                     get_encrypted_certificate_by_raw_certificate(&user_value.current_password),
    //                 ))
    //                 .first::<i64>(&conn)?;

    //             diesel::update(user_auth::table.find(user_auth_id))
    //                 .set(
    //                     user_auth::certificate.eq(get_encrypted_certificate_by_raw_certificate(
    //                         &user_value.modified_password,
    //                     )),
    //                 )
    //                 .execute(&conn)
    //         })
    //         .await,
    //     ));
    // }

    Json(UnifiedResponseMessages::<usize>::error())
}
#[handler]

async fn delete_user(
    req: &Request,
    Json(model::UserId { user_id }): Json<model::UserId>,
    pool: Data<&db::ConnectionPool>,
) -> impl IntoResponse {
    // FIXME:
    // let operation_log_pair_option = generate_operation_user_delete_log(
    //     &req.get_session(),
    //     &CommonTableRecord::default().set_id(user_id as i64),
    // )
    // .ok();
    // send_option_operation_log_pair(operation_log_pair_option).await;

    if let Ok(conn) = pool.get() {
        let f_result = spawn_blocking::<_, Result<_, diesel::result::Error>>(move || {
            conn.transaction(|| {
                diesel::delete(user::table.filter(user::id.eq(user_id))).execute(&conn)?;
                diesel::delete(user_auth::table.filter(user_auth::user_id.eq(user_id)))
                    .execute(&conn)?;

                Ok(())
            })
        })
        .await;

        let resp = f_result
            .map(|resp_result| Into::<UnifiedResponseMessages<()>>::into(resp_result))
            .unwrap_or_else(|e| {
                UnifiedResponseMessages::<()>::error().customized_error_msg(e.to_string())
            });
        return Json(resp);
    }

    Json(UnifiedResponseMessages::<()>::error())
}

// FIXME:
// #[handler]

// async fn login_user(
//     req: &Request,
//     Json(user_login): Json<model::UserAuthLogin>,
//     session: Session,
//     pool: Data<&db::ConnectionPool>,
// ) -> impl IntoResponse {
//     let login_result: UnifiedResponseMessages<()> =
//         pre_login_user(req, user_login, session, pool).await.into();

//     Json(login_result)
// }

async fn pre_login_user(
    req: &Request,
    model::UserAuthLogin {
        login_type,
        account,
        password,
    }: model::UserAuthLogin,
    // session: Session,
    pool: Data<&db::ConnectionPool>,
) -> Result<(), CommonError> {
    use model::user_login_log::NewUserLoginLog;

    // FIXME:

    todo!();
    // let connection = req.connection_info();
    // let client_ip = connection.realip_remote_addr();
    // let mut new_user_login_log = NewUserLoginLog::default();
    // new_user_login_log
    //     .set_lastip(client_ip)
    //     .set_login_type(login_type);

    // let conn = pool.get()?;
    // let user_package: (model::UserAuth, model::User) =
    //     spawn_blocking::<_,Result<_, diesel::result::Error>>(move || {
    //         let login_result = user_auth::table
    //             .inner_join(user::table)
    //             .select((user_auth::all_columns, user::all_columns))
    //             .filter(user_auth::identity_type.eq(login_type))
    //             .filter(user_auth::identifier.eq(&account))
    //             .filter(
    //                 user_auth::certificate
    //                     .eq(get_encrypted_certificate_by_raw_certificate(&password)),
    //             )
    //             .first::<(model::UserAuth, model::User)>(&conn);

    //         login_result
    //             .as_ref()
    //             .map(|(_, user)| {
    //                 new_user_login_log
    //                     .set_user_name(user.user_name.clone())
    //                     .set_user_id(user.id)
    //                     .set_command(state::user_login_log::LoginCommand::LoginSuccess as u8);
    //             })
    //             .map_err(|_| {
    //                 new_user_login_log
    //                     .set_user_name(account)
    //                     .set_command(state::user_login_log::LoginCommand::Logoutfailure as u8);
    //             })
    //             .ok();

    //         diesel::insert_into(user_login_log::table)
    //             .values(&new_user_login_log)
    //             .execute(&conn)
    //             .ok();

    //         login_result
    //     })
    //     .await?;
    // save_session(session, user_package)
}

fn save_session(
    // session: Session,
    (_, user): (model::UserAuth, model::User),
) -> Result<(), CommonError> {
    // FIXME:

    // session.set("login_time", get_timestamp())?;
    // session.set("user_id", user.id)?;
    // session.set("user_name", user.user_name)?;
    // session.set("nick_name", user.nick_name)?;
    // Ok(())
    todo!();
}

// FIXME:

// #[handler]

// async fn check_user(session: Session, pool: Data<&db::ConnectionPool>) -> impl IntoResponse {
//     let check_result = pre_check_user(session, pool).await;
//     if let Ok(user) = check_result {
//         return Json(UnifiedResponseMessages::<model::User>::success_with_data(
//             user,
//         ));
//     };

//     // FIXME:
//     todo!();
//     // Json(
//     //     UnifiedResponseMessages::<()>::error()
//     //         .customized_error_msg(check_result.expect_err("").to_string()),
//     // )
// }

// FIXME:

// async fn pre_check_user(
//     session: Session,
//     pool: Data<&db::ConnectionPool>,
// ) -> Result<model::User, CommonError> {
//     let conn = pool.get()?;
//     let user_id = session
//         .get::<u64>("user_id")?
//         .ok_or_else(|| CommonError::DisPass("Without set `user_id` .".into()))?;

//     let user = spawn_blocking::<_,Result<_, diesel::result::Error>>(move || {
//         let user = user::table
//             .select(user::all_columns)
//             .find(user_id)
//             .first::<model::User>(&conn)?;

//         Ok(user)
//     })
//     .await?;

//     Ok(user)
// }

// FIXME:
// #[handler]

// async fn logout_user(session: Session) -> impl IntoResponse {
//     Json({
//         session.clear();
//         UnifiedResponseMessages::<()>::success()
//     })
// }

#[handler]

async fn roles(
    enforcer: Data<&RwLock<Enforcer>>,
    Json(UserName { user_name }): Json<UserName>,
) -> impl IntoResponse {
    let mut enforcer_guard = enforcer.write().await;
    let mut roles = enforcer_guard.get_roles_for_user(&user_name, None);
    let implicit_roles = enforcer_guard.get_implicit_roles_for_user(&user_name, None);
    roles.extend(implicit_roles.into_iter());

    Json(UnifiedResponseMessages::<Vec<String>>::success_with_data(
        roles,
    ))
}

#[handler]

async fn permissions(
    enforcer: Data<&RwLock<Enforcer>>,
    Json(UserName { user_name }): Json<UserName>,
) -> impl IntoResponse {
    let mut enforcer_guard = enforcer.write().await;

    let mut permissions = enforcer_guard.get_permissions_for_user(&user_name, None);
    let implicit_permissions = enforcer_guard.get_implicit_permissions_for_user(&user_name, None);
    permissions.extend(implicit_permissions.into_iter());

    Json(UnifiedResponseMessages::<Vec<Vec<String>>>::success_with_data(permissions))
}

#[handler]

async fn append_role(
    req: &Request,
    enforcer: Data<&RwLock<Enforcer>>,
    Json(user_and_roles): Json<UserAndRoles>,
) -> impl IntoResponse {
    // FIXME:

    // let operation_log_pair_option =
    //     generate_operation_user_role_addtion_log(&req.get_session(), &user_and_roles).ok();
    // send_option_operation_log_pair(operation_log_pair_option).await;

    let UserAndRoles {
        user_name,
        mut operate_roles,
    } = user_and_roles;

    operate_roles.sort_unstable();
    operate_roles.dedup();
    let append_roles: Vec<String> = operate_roles
        .iter()
        .filter_map(|role_id| ROLES.get(*role_id).map(|role_name| role_name.to_string()))
        .collect();

    if append_roles.is_empty() {
        return Json(UnifiedResponseMessages::<bool>::error());
    }

    let mut enforcer_guard = enforcer.write().await;
    let operated_result = enforcer_guard
        .add_roles_for_user(&user_name, append_roles, None)
        .await;

    let msg = Into::<UnifiedResponseMessages<bool>>::into(operated_result);
    Json(msg)
}

#[handler]

async fn delete_role(
    req: &Request,
    enforcer: Data<&RwLock<Enforcer>>,
    Json(user_and_roles): Json<UserAndRoles>,
) -> impl IntoResponse {
    // FIXME:

    // let operation_log_pair_option =
    //     generate_operation_user_role_delete_log(&req.get_session(), &user_and_roles).ok();
    // send_option_operation_log_pair(operation_log_pair_option).await;

    let UserAndRoles {
        user_name,
        mut operate_roles,
    } = user_and_roles;

    operate_roles.sort_unstable();
    operate_roles.dedup();

    let delete_roles: Vec<&str> = operate_roles
        .iter()
        .filter_map(|role_id| ROLES.get(*role_id).copied())
        .collect();

    if delete_roles.is_empty() {
        return Json(UnifiedResponseMessages::<bool>::error());
    }

    let mut enforcer_guard = enforcer.write().await;

    for role in delete_roles {
        enforcer_guard
            .delete_role_for_user(&user_name, role, None)
            .await
            .map_err(|e| error!("role: {}, error: {}", role, e))
            .unwrap_or_default();
    }

    Json(UnifiedResponseMessages::<bool>::success())
}

#[handler]

async fn append_permission(
    req: &Request,
    enforcer: Data<&RwLock<Enforcer>>,
    Json(user_and_permissions): Json<UserAndPermissions>,
) -> impl IntoResponse {
    // FIXME:

    // let operation_log_pair_option =
    //     generate_operation_user_permission_addtion_log(&req.get_session(), &user_and_permissions)
    //         .ok();
    // send_option_operation_log_pair(operation_log_pair_option).await;

    let UserAndPermissions {
        user_name,
        operate_permissions,
    } = user_and_permissions;

    let mut enforcer_guard = enforcer.write().await;
    let operated_result = enforcer_guard
        .add_permissions_for_user(&user_name, operate_permissions)
        .await;
    let msg = Into::<UnifiedResponseMessages<bool>>::into(operated_result);
    Json(msg)
}

#[handler]
async fn delete_permission(
    req: &Request,
    enforcer: Data<&RwLock<Enforcer>>,
    Json(user_and_permissions): Json<UserAndPermissions>,
) -> impl IntoResponse {
    // FIXME:

    // let operation_log_pair_option =
    //     generate_operation_user_permission_delete_log(&req.get_session(), &user_and_permissions)
    //         .ok();
    // send_option_operation_log_pair(operation_log_pair_option).await;

    let UserAndPermissions {
        user_name,
        operate_permissions,
    } = user_and_permissions;

    let mut enforcer_guard = enforcer.write().await;

    for operate_permission in operate_permissions {
        enforcer_guard
            .delete_permission_for_user(&user_name, operate_permission)
            .await
            .map_err(|e| error!("error: {}", e))
            .unwrap_or_default();
    }

    let msg = UnifiedResponseMessages::<()>::success();
    Json(msg)
}
