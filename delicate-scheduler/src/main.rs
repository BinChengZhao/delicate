#![recursion_limit = "256"]
#![allow(clippy::expect_fun_call)]
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]

//! delicate-scheduler.

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate diesel_migrations;

pub(crate) mod actions;
pub(crate) mod components;
pub(crate) mod db;
pub(crate) mod prelude;
#[macro_use]
pub(crate) mod macros;

pub(crate) use prelude::*;
use {cfg_mysql_support, cfg_postgres_support};

#[actix_web::main]
async fn main() -> AnyResut<()> {
    // Loads environment variables.
    dotenv().ok();

    db::init();

    let scheduler_listening_address = env::var("SCHEDULER_LISTENING_ADDRESS")
        .expect("Without `SCHEDULER_LISTENING_ADDRESS` set in .env");

    let scheduler_front_end_domain: String = env::var("SCHEDULER_FRONT_END_DOMAIN")
        .expect("Without `SCHEDULER_FRONT_END_DOMAIN` set in .env");

    FmtSubscriber::builder()
        // will be written to stdout.
        .with_max_level(Level::INFO)
        .with_thread_names(true)
        // completes the builder.
        .init();

    let delay_timer = DelayTimerBuilder::default().enable_status_report().build();
    let shared_delay_timer = ShareData::new(delay_timer);

    let connection_pool = db::get_connection_pool();
    let shared_connection_pool = ShareData::new(connection_pool);
    let shared_scheduler_meta_info: SharedSchedulerMetaInfo =
        ShareData::new(SchedulerMetaInfo::default());

    let result = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&scheduler_front_end_domain)
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);

        App::new()
            .configure(actions::task::config)
            .configure(actions::user::config)
            .configure(actions::task_log::config)
            .configure(actions::executor_group::config)
            .configure(actions::executor_processor::config)
            .configure(actions::executor_processor_bind::config)
            .configure(actions::data_reports::config)
            .configure(actions::components::config)
            .configure(actions::operation_log::config)
            .configure(actions::user_login_log::config)
            .app_data(shared_delay_timer.clone())
            .app_data(shared_connection_pool.clone())
            .app_data(shared_scheduler_meta_info.clone())
            .wrap(components::session::auth_middleware())
            .wrap(components::session::session_middleware())
            .wrap(cors)
            .wrap(MiddlewareLogger::default())
    })
    .bind(scheduler_listening_address)?
    .run()
    .await;

    Ok(result?)
}
