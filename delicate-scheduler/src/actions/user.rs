use super::prelude::*;
use validator::{Validate, ValidationError};

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user);
    //     .service(show_users)
    //     .service(update_user)
    //     .service(delete_user);
}

#[post("/api/user/create")]
async fn create_user(
    web::Json(user): web::Json<model::QueryNewUser>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    use db::schema::{user, user_auth};

    let validate_result = user.validate();
    if validate_result.is_err() {
        return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<()>>::into(validate_result));
    }

    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<()>>::into(
            web::block::<_, _, diesel::result::Error>(move || {
                conn.transaction(|| {
                    diesel::insert_into(user::table)
                        .values(&(Into::<model::NewUser>::into(&user)))
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
            .await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<()>::error())
}

// #[post("/api/user/list")]
// async fn show_users(
//     web::Json(query_params): web::Json<model::QueryParamsuser>,
//     pool: ShareData<db::ConnectionPool>,
// ) -> HttpResponse {
//     if let Ok(conn) = pool.get() {
//         return HttpResponse::Ok().json(
//             Into::<UnifiedResponseMessages<model::Paginateuser>>::into(
//                 web::block(move || {
//                     let query_builder = model::userQueryBuilder::query_all_columns();

//                     let users = query_params
//                         .clone()
//                         .query_filter(query_builder)
//                         .paginate(query_params.page)
//                         .load::<model::user>(&conn);

//                     if let Err(users_err) = users {
//                         return Err(users_err);
//                     }

//                     let per_page = query_params.per_page;
//                     let count_builder = model::userQueryBuilder::query_count();
//                     let count = query_params
//                         .query_filter(count_builder)
//                         .get_result::<i64>(&conn);

//                     if let Err(count_err) = count {
//                         return Err(count_err);
//                     }

//                     Ok(model::user::Paginateuser::default()
//                         .set_users(users.unwrap())
//                         .set_per_page(per_page)
//                         .set_total_page(count.unwrap()))
//                 })
//                 .await,
//             ),
//         );
//     }

//     HttpResponse::Ok().json(UnifiedResponseMessages::<model::Paginateuser>::error())
// }

// #[post("/api/user/update")]
// async fn update_user(
//     web::Json(user_value): web::Json<model::Newuser>,
//     pool: ShareData<db::ConnectionPool>,
// ) -> HttpResponse {
//     if let Ok(conn) = pool.get() {
//         return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<usize>>::into(
//             web::block(move || diesel::update(&user_value).set(&user_value).execute(&conn)).await,
//         ));
//     }

//     HttpResponse::Ok().json(UnifiedResponseMessages::<usize>::error())
// }
// #[post("/api/user/delete")]
// async fn delete_user(
//     web::Path(user_id): web::Path<i64>,
//     pool: ShareData<db::ConnectionPool>,
// ) -> HttpResponse {
//     use db::schema::user::dsl::*;

//     if let Ok(conn) = pool.get() {
//         return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<usize>>::into(
//             web::block(move || diesel::delete(user.find(user_id)).execute(&conn)).await,
//         ));
//     }

//     HttpResponse::Ok().json(UnifiedResponseMessages::<usize>::error())
// }
