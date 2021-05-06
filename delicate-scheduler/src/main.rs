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

use actix_session::CookieSession;
use actix_web::web::{self, Data as ShareData};
use actix_web::{post, App, HttpResponse, HttpServer};
use diesel::prelude::*;
use flexi_logger::{Age, Cleanup, Criterion, LogTarget, Logger, Naming};
use std::env;

use anyhow::Result as AnyResut;
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
    // TODO: Need pagination.
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
async fn main() -> AnyResut<()> {
    dotenv().ok();
    db::init();
    let logger = Logger::with_str("info")
        .log_target(LogTarget::File)
        .buffer_and_flush()
        .rotate(
            Criterion::Age(Age::Day), 
            Naming::Timestamps,       
            Cleanup::KeepLogFiles(10),
        )
        .start()?;

        
        let delay_timer = DelayTimerBuilder::default().enable_status_report().build();
        let shared_delay_timer = ShareData::new(delay_timer);
        
        let connection_pool = db::get_connection_pool();
    let shared_connection_pool = ShareData::new(connection_pool);


    let result = HttpServer::new(move || {
        App::new()
            // TODO: Try use App::configure.
            .service(show_tasks)
            .service(create_task)
            .service(update_task)
            .service(delete_task)
            .app_data(shared_delay_timer.clone())
            .app_data(shared_connection_pool.clone())
            .wrap(
                CookieSession::signed(
                    &env::var("SESSION_TOKEN")
                        .expect("Without `SESSION_TOKEN` set in .env")
                        .into_bytes(),
                )
                .domain(
                    env::var("SCHEDULER_DOMAIN").expect("Without `SCHEDULER_DOMAIN` set in .env"),
                )
                .name(env::var("SCHEDULER_NAME").expect("Without `SCHEDULER_NAME` set in .env"))
                .secure(true),
            )
    })
    .bind(env::var("SCHEDULER_LISTENING_ADDRESS").expect("Without `SCHEDULER_LISTENING_ADDRESS` set in .env"))?
    .run()
    .await;

    // Finish processing the buffer log first, then process the result.
    logger.shutdown();
    Ok(result?)
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
