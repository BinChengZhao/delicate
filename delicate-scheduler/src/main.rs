#![recursion_limit = "256"]
#![allow(clippy::expect_fun_call)]
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]

//! delicate-scheduler.
// TODO: diesel's io operations are offloaded to `actix_web::web::block`.

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate diesel_migrations;

mod db;
#[macro_use]
mod macros;

use {cfg_mysql_support, cfg_postgres_support};

use actix_web::web::{self, Data as ShareData};
use actix_web::{post, App, HttpResponse, HttpServer};
use diesel::prelude::*;

use db::model;
use delay_timer::prelude::*;
use diesel::query_dsl::RunQueryDsl;
use dotenv::dotenv;

// use db::schema::posts::dsl::*;

// TODO: return front-end json is small hump patten.

#[post("/api/task/create")]
async fn create_task(
    task: web::Json<model::NewTask>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    use db::schema::task;

    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<usize>>::into(
            web::block(move || {
                diesel::insert_into(task::table)
                    .values(&*task)
                    .execute(&conn)
            })
            .await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<()>::error())
}

#[post("/api/task/list")]
async fn show_tasks(
    web::Json(query_params): web::Json<model::QueryParamsTask>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<Vec<model::Task>>>::into(
            web::block(move || query_params.query(&conn)).await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<Vec<model::Task>>::error())
}

#[post("/api/task/delete")]
async fn delete_task(
    web::Path(task_id): web::Path<i64>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    use db::schema::task::dsl::*;

    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<usize>>::into(
            web::block(move || diesel::delete(task.find(task_id)).execute(&conn)).await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<usize>::error())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    db::init();

    let connection_pool = db::get_connection_pool();

    let delay_timer = DelayTimerBuilder::default().enable_status_report().build();

    let shared_delay_timer = ShareData::new(delay_timer);
    let shared_connection_pool = ShareData::new(connection_pool);

    HttpServer::new(move || {
        App::new()
            // TODO: Try use App::configure.
            .service(show_tasks)
            .service(create_task)
            .service(update_task)
            .service(delete_task)
            .app_data(shared_delay_timer.clone())
            .app_data(shared_connection_pool.clone())
    })
    .bind("127.0.0.1:8090")?
    .run()
    .await
}

#[post("/api/task/update")]
async fn update_task(
    web::Json(task_value): web::Json<model::NewTask>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<usize>>::into(
            web::block(move || diesel::update(&task_value).set(&task_value).execute(&conn)).await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<usize>::error())
}

use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub(crate) trait UniformData: Default + Debug + Clone + Serialize {}

impl<T: Default + Debug + Clone + Serialize> UniformData for T {}

/// Uniform public message response format.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub(crate) struct UnifiedResponseMessages<T: UniformData> {
    code: i8,
    msg: String,
    data: T,
}

impl<T: UniformData> UnifiedResponseMessages<T> {
    #[allow(dead_code)]
    pub(crate) fn success() -> Self {
        UnifiedResponseMessages::default()
    }

    pub(crate) fn success_with_data(data: T) -> Self {
        UnifiedResponseMessages {
            data,
            ..Default::default()
        }
    }

    pub(crate) fn error() -> Self {
        UnifiedResponseMessages {
            code: -1,
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub(crate) fn error_with_data(data: T) -> Self {
        let code = -1;
        UnifiedResponseMessages {
            code,
            data,
            ..Default::default()
        }
    }

    pub(crate) fn customized_error_msg(mut self, msg: String) -> Self {
        self.msg = msg;

        self
    }

    #[allow(dead_code)]
    pub(crate) fn customized_error_code(mut self, code: i8) -> Self {
        self.code = code;

        self
    }

    #[allow(dead_code)]
    pub(crate) fn reverse(mut self) -> Self {
        self.code = -1 - self.code;
        self
    }
}

impl<T: UniformData, E: std::error::Error> From<Result<T, E>> for UnifiedResponseMessages<T> {
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(d) => Self::success_with_data(d),
            Err(e) => Self::error().customized_error_msg(e.to_string()),
        }
    }
}
