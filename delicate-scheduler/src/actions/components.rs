// Front-end component api.

use super::prelude::*;

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(binding_list);
}

#[get("/api/binding/list")]
async fn binding_list(pool: ShareData<db::ConnectionPool>) -> HttpResponse {
    use model::{ExecutorBinding, ExecutorProcessorBindQueryBuilder};

    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(
            Into::<UnifiedResponseMessages<Vec<model::ExecutorBinding>>>::into(
                web::block::<_, _, diesel::result::Error>(move || {
                    ExecutorProcessorBindQueryBuilder::query_binding_columns()
                        .load::<ExecutorBinding>(&conn)
                })
                .await,
            ),
        );
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<Vec<model::ExecutorBinding>>::error())
}
