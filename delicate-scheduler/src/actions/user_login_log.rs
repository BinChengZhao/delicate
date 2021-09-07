use super::prelude::*;

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(show_user_login_log);
}

#[post("/api/user_login_log/list")]
async fn show_user_login_log(
    web::Json(query_params): web::Json<model::QueryParamsUserLoginLog>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<
            UnifiedResponseMessages<PaginateData<model::UserLoginLog>>,
        >::into(
            web::block::<_, _, diesel::result::Error>(move || {
                let query_builder = model::UserLoginLogQueryBuilder::query_all_columns();

                let user_login_log = query_params
                    .clone()
                    .query_filter(query_builder)
                    .paginate(query_params.page)
                    .set_per_page(query_params.per_page)
                    .load::<model::UserLoginLog>(&conn)?;

                let per_page = query_params.per_page;
                let count_builder = model::UserLoginLogQueryBuilder::query_count();
                let count = query_params
                    .query_filter(count_builder)
                    .get_result::<i64>(&conn)?;

                let front_end_user_login_log: Vec<model::UserLoginLog> =
                    user_login_log.into_iter().collect();
                Ok(PaginateData::<model::UserLoginLog>::default()
                    .set_data_source(front_end_user_login_log)
                    .set_page_size(per_page)
                    .set_total(count)
                    .set_state_desc::<state::user_login_log::LoginCommand>()
                    .set_state_desc::<state::user_login_log::LoginType>())
            })
            .await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<PaginateData<model::UserLoginLog>>::error())
}
