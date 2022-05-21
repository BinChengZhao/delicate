#![recursion_limit = "256"]
#![allow(clippy::expect_fun_call)]
#![allow(clippy::let_and_return)]
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]

//! delicate-scheduler.

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate diesel_migrations;

#[macro_use]
pub(crate) mod macros;
pub(crate) mod actions;
pub(crate) mod components;
pub(crate) mod db;
pub(crate) mod prelude;

pub(crate) use prelude::*;

fn main() -> AnyResut<()> {
    // Loads environment variables.
    dotenv().ok();

    // Automatic execution of database migration
    db::init();

    // Automatic initialization of log consumers
    let _fw_handle = init_logger();

    // Initialize custom asynchronous runtime
    let raw_runtime = Builder::new_multi_thread()
        .thread_name_fn(|| {
            static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
            let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
            format!("scheduler-{}", id)
        })
        .thread_stack_size(4 * 1024 * 1024)
        .enable_all()
        .build()
        .expect("Init Tokio runtime failed.");
    let arc_runtime = Arc::new(raw_runtime);
    let arc_runtime_cloned = arc_runtime.clone();

    let scheduler_listening_address = env::var("SCHEDULER_LISTENING_ADDRESS")
        .expect("Without `SCHEDULER_LISTENING_ADDRESS` set in .env");

    arc_runtime.block_on(async {
        let app = Route::new().nest_no_strip(
            "/api",
            Route::new()
                .nest_no_strip("/api/task", actions::task::route_config())
                .nest_no_strip("/api/user", actions::user::route_config())
                .nest_no_strip("/api/role", actions::role::route_config())
                .nest_no_strip("/api/task_log", actions::task_log::route_config())
                .nest_no_strip("/api/tasks_state", actions::data_reports::route_config())
                .nest_no_strip("/api/task_instance", actions::task_instance::route_config())
                .nest_no_strip("/api/binding", actions::components::binding::route_config())
                .nest_no_strip("/api/operation_log", actions::operation_log::route_config())
                .nest_no_strip(
                    "/api/executor_group",
                    actions::executor_group::route_config(),
                )
                .nest_no_strip(
                    "/api/executor_processor",
                    actions::executor_processor::route_config(),
                )
                .nest_no_strip(
                    "/api/executor_processor_bind",
                    actions::executor_processor_bind::route_config(),
                )
                .nest_no_strip(
                    "/api/executor",
                    actions::components::executor::route_config(),
                )
                .nest_no_strip(
                    "/api/permission",
                    actions::components::permission::route_config(),
                )
                .nest_no_strip(
                    "/api/user_login_log",
                    actions::user_login_log::route_config(),
                ),
        );

        let app = init_scheduler(app, arc_runtime_cloned).await;

        let listener = TcpListener::bind(scheduler_listening_address);
        let server = Server::new(listener);
        Ok(server.run(app).await?)
    })
}

fn init_logger() -> FileLogWriterHandle {
    let log_level: Level =
        FromStr::from_str(&env::var("LOG_LEVEL").unwrap_or_else(|_| String::from("info")))
            .expect("Log level acquired fail.");

    // Prepare a `FileLogWriter` and a handle to it, and keep the handle alive
    // until the program ends (it will flush and shutdown the `FileLogWriter` when dropped).
    // For the `FileLogWriter`, use the settings that fit your needs
    let (file_writer, _fw_handle) = FileLogWriter::builder(FileSpec::default())
        .rotate(
            // If the program runs long enough,
            Criterion::Age(Age::Day),  // - create a new file every day
            Naming::Timestamps,        // - let the rotated files have a timestamp in their name
            Cleanup::KeepLogFiles(15), // - keep at most seven log files
        )
        .write_mode(WriteMode::Async)
        .try_build_with_handle()
        .expect("flexi_logger init failed");

    FmtSubscriber::builder()
        // will be written to file_writer.
        .with_max_level(log_level)
        .with_thread_names(true)
        .with_writer(move || file_writer.clone())
        // completes the builder.
        .init();

    _fw_handle
}

async fn init_scheduler(app: Route, arc_runtime_cloned: Arc<Runtime>) -> impl Endpoint {
    let scheduler_front_end_domain: String = env::var("SCHEDULER_FRONT_END_DOMAIN")
        .expect("Without `SCHEDULER_FRONT_END_DOMAIN` set in .env");

    let request_client = RequestClient::new();

    let cors = Cors::new()
        .allow_origin(&scheduler_front_end_domain)
        .allow_method(HttpMethod::GET)
        .allow_method(HttpMethod::POST)
        .allow_method(HttpMethod::OPTIONS)
        .allow_header("content-type")
        .allow_credentials(true)
        .max_age(3600);

    let delay_timer = DelayTimerBuilder::default()
        .tokio_runtime_shared_by_custom(arc_runtime_cloned)
        .enable_status_report()
        .build();
    let connection_pool = db::get_connection_pool();
    let arc_delay_timer = Arc::new(delay_timer);
    let arc_connection_pool = Arc::new(connection_pool);

    let shared_delay_timer = AddData::new(arc_delay_timer.clone());
    let shared_connection_pool = AddData::new(arc_connection_pool.clone());
    let shared_scheduler_meta_info: AddData<Arc<SchedulerMetaInfo>> =
        AddData::new(Arc::new(SchedulerMetaInfo::default()));
    let shared_request_client = AddData::new(request_client.clone());

    #[cfg(AUTH_CASBIN)]
    let enforcer = get_casbin_enforcer(arc_connection_pool.clone()).await;
    #[cfg(AUTH_CASBIN)]
    let shared_enforcer = Arc::new(RwLock::new(enforcer));

    #[cfg(AUTH_CASBIN)]
    let app = app
        .with(CasbinService)
        .with(AddData::new(shared_enforcer.clone()));

    // All ready work when the delicate-application starts.
    launch_ready_operation(
        arc_connection_pool.clone(),
        request_client,
        #[cfg(AUTH_CASBIN)]
        shared_enforcer.clone(),
    )
    .await;

    app.with(shared_delay_timer)
        .with(shared_connection_pool)
        .with(shared_scheduler_meta_info)
        .with(shared_request_client)
        .with(components::session::auth_middleware())
        .with(components::session::cookie_middleware())
        .with(components::session::session_middleware())
        .with(cors)
        .with(components::logger_id::logger_id_middleware())
}

// All ready work when the delicate-application starts.
async fn launch_ready_operation(
    pool: Arc<db::ConnectionPool>,
    request_client: RequestClient,
    #[cfg(AUTH_CASBIN)] enforcer: Arc<RwLock<Enforcer>>,
) {
    launch_health_check(pool.clone(), request_client);
    launch_operation_log_consumer(pool);

    #[cfg(AUTH_CASBIN)]
    {
        // When the delicate starts, it checks if the resource acquisition is normal.
        let redis_url = env::var("REDIS_URL").expect("The redis url could not be acquired.");
        let redis_client = redis::Client::open(redis_url)
            .expect("The redis client resource could not be initialized.");
        launch_casbin_rule_events_consumer(redis_client, enforcer);
    }
}

// Heartbeat checker
// That constantly goes to detect whether the machine survives with the machine's indicators.
fn launch_health_check(pool: Arc<db::ConnectionPool>, request_client: RequestClient) {
    tokio_spawn(loop_health_check(pool, request_client));
}

// Operation log asynchronous consumer
//
// The user's operations in the system are logged to track,
// But in order not to affect the performance of the system,
// These logs go through the channel with the asynchronous state machine to consume.
fn launch_operation_log_consumer(pool: Arc<db::ConnectionPool>) {
    tokio_spawn(loop_operate_logs(pool));
}
