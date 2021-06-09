mod component;
mod prelude;
use prelude::*;

/// This handler uses json extractor
#[get("/api/task/create")]
async fn create_task(
    web::Json(signed_task_package): web::Json<SignedTaskPackage>,
    shared_delay_timer: SharedDelayTimer,
    executor_conf: SharedExecutorSecurityConf,
) -> impl Responder {
    let response = UnitUnifiedResponseMessages::error();
    // if let Ok(task) = Task::try_from(task_conf.0) {
    //     response = shared_delay_timer.add_task(task).into();
    // }
    let guard = executor_conf.get_bind_scheduler_inner_ref().await;
    let token = guard.as_ref().map(|b| (&b.1).deref());
    let task_package = signed_task_package
        .get_task_package_after_verify(token)
        .map(|t| {});

    HttpResponse::Ok().json(response)
}

#[get("/remove_task/{id}")]
async fn remove_task(
    web::Path(task_id): web::Path<u64>,
    shared_delay_timer: SharedDelayTimer,
) -> HttpResponse {
    let response: UnitUnifiedResponseMessages = shared_delay_timer.remove_task(task_id).into();
    HttpResponse::Ok().json(response) // <- send response
}

// TODO: Recive by json.
#[get("/cancel_task/{task_id}/{record_id}")]
async fn cancel_task(
    web::Path((task_id, record_id)): web::Path<(u64, i64)>,
    shared_delay_timer: SharedDelayTimer,
) -> HttpResponse {
    let response: UnitUnifiedResponseMessages =
        shared_delay_timer.cancel_task(task_id, record_id).into();
    HttpResponse::Ok().json(response) // <- send response
}

#[allow(dead_code)]
async fn maintenance(shared_delay_timer: SharedDelayTimer) -> impl Responder {
    HttpResponse::Ok().json(Into::<UnitUnifiedResponseMessages>::into(
        shared_delay_timer.stop_delay_timer(),
    ))
}

//Health Screening
#[get("/health_screen")]
async fn health_screen(system_mirror: SharedSystemMirror) -> impl Responder {
    HttpResponse::Ok().json(
        UnifiedResponseMessages::<SystemSnapshot>::success_with_data(
            system_mirror.refresh_all().await,
        ),
    )
}

#[get("/bind_executor")]
// token.

// Or use middleware to reach consensus.
// Register token at executor startup, check token when RequestScheduler bind executor.

// Or set security level, no authentication at level 0, public and private keys required at level 1.
async fn bind_executor(
    web::Json(request_bind_scheduler): web::Json<SignedBindRequest>,
    security_conf: web::Data<ExecutorSecurityConf>,
) -> impl Responder {
    let verify_result = request_bind_scheduler.verify(security_conf.get_ref().get_rsa_public_key());
    if verify_result.is_ok() {
        let SignedBindRequest { bind_request, .. } = request_bind_scheduler;

        let token: String = repeat_with(fastrand::alphanumeric).take(32).collect();
        security_conf
            .get_bind_scheduler_inner_mut()
            .await
            .replace((bind_request, token.clone()));

        let bind_response = BindResponse {
            time: get_timestamp() as i64,
            token,
        }
        .encrypt_self(security_conf.get_rsa_public_key());

        let response: UnifiedResponseMessages<EncryptedBindResponse> = Into::into(bind_response);
        return HttpResponse::Ok().json(response);
    }

    HttpResponse::Ok().json(
        UnifiedResponseMessages::<EncryptedBindResponse>::error()
            .customized_error_msg(verify_result.expect_err("").to_string()),
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let delay_timer = DelayTimerBuilder::default().enable_status_report().build();

    let shared_delay_timer: SharedDelayTimer = ShareData::new(delay_timer);
    let shared_security_conf: SharedExecutorSecurityConf =
        ShareData::new(ExecutorSecurityConf::default());
    let shared_system_mirror: SharedSystemMirror = ShareData::new(SystemMirror::default());

    HttpServer::new(move || {
        App::new()
            .service(bind_executor)
            .service(create_task)
            .service(remove_task)
            .service(cancel_task)
            .service(health_screen)
            .app_data(shared_delay_timer.clone())
            .app_data(shared_security_conf.clone())
            .app_data(shared_system_mirror.clone())
    })
    .bind(
        env::var("EXECUTOR_LISTENING_ADDRESS")
            .expect("Without `EXECUTOR_LISTENING_ADDRESS` set in .env"),
    )?
    .run()
    .await
}
