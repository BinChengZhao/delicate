// Front-end component api.

use super::prelude::*;

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(binding_list).service(executor_list);
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
