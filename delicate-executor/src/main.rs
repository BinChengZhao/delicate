mod component;
mod prelude;
use prelude::*;

/// This handler uses json extractor
#[post("/api/task/create")]
async fn create_task(
    web::Json(signed_task_package): web::Json<SignedTaskPackage>,
    shared_delay_timer: SharedDelayTimer,
    executor_conf: SharedExecutorSecurityConf,
) -> impl Responder {
    let response: UnitUnifiedResponseMessages =
        Into::into(pre_create_task(signed_task_package, shared_delay_timer, executor_conf).await);

    HttpResponse::Ok().json(response)
}

pub async fn pre_create_task(
    signed_task_package: SignedTaskPackage,
    shared_delay_timer: SharedDelayTimer,
    executor_conf: SharedExecutorSecurityConf,
) -> Result<(), CommonError> {
    let guard = executor_conf.get_bind_scheduler_inner_ref().await;
    let token = guard.as_ref().map(|b| (&b.1).deref());
    let task = signed_task_package
        .get_task_package_after_verify(token)
        .map(TryInto::<Task>::try_into)??;

    Ok(shared_delay_timer.add_task(task)?)
}

#[post("/api/task/remove")]
async fn remove_task(
    web::Json(signed_task_unit): web::Json<SignedTaskUnit>,
    shared_delay_timer: SharedDelayTimer,
    executor_conf: SharedExecutorSecurityConf,
) -> HttpResponse {
    let response: UnitUnifiedResponseMessages =
        pre_remove_task(signed_task_unit, shared_delay_timer, executor_conf)
            .await
            .into();
    HttpResponse::Ok().json(response)
}

pub async fn pre_remove_task(
    signed_task_unit: SignedTaskUnit,
    shared_delay_timer: SharedDelayTimer,
    executor_conf: SharedExecutorSecurityConf,
) -> Result<(), CommonError> {
    let guard = executor_conf.get_bind_scheduler_inner_ref().await;
    let token = guard.as_ref().map(|b| (&b.1).deref());
    let task_unit = signed_task_unit.get_task_unit_after_verify(token)?;
    Ok(shared_delay_timer.remove_task(task_unit.task_id as u64)?)
}

#[post("/api/task/advance")]
async fn advance_task(
    web::Json(signed_task_unit): web::Json<SignedTaskUnit>,
    shared_delay_timer: SharedDelayTimer,
    executor_conf: SharedExecutorSecurityConf,
) -> HttpResponse {
    let response: UnitUnifiedResponseMessages =
        pre_advance_task(signed_task_unit, shared_delay_timer, executor_conf)
            .await
            .into();
    HttpResponse::Ok().json(response)
}

pub async fn pre_advance_task(
    signed_task_unit: SignedTaskUnit,
    shared_delay_timer: SharedDelayTimer,
    executor_conf: SharedExecutorSecurityConf,
) -> Result<(), CommonError> {
    let guard = executor_conf.get_bind_scheduler_inner_ref().await;
    let token = guard.as_ref().map(|b| (&b.1).deref());
    let task_unit = signed_task_unit.get_task_unit_after_verify(token)?;
    Ok(shared_delay_timer.advance_task(task_unit.task_id as u64)?)
}

#[post("/api/task_instance/kill")]
async fn cancel_task(
    web::Json(signed_cancel_task_record): web::Json<SignedCancelTaskRecord>,
    shared_delay_timer: SharedDelayTimer,
    executor_conf: SharedExecutorSecurityConf,
) -> HttpResponse {
    let response: UnitUnifiedResponseMessages =
        pre_cancel_task(signed_cancel_task_record, shared_delay_timer, executor_conf)
            .await
            .into();
    HttpResponse::Ok().json(response)
}

pub async fn pre_cancel_task(
    signed_cancel_task_record: SignedCancelTaskRecord,
    shared_delay_timer: SharedDelayTimer,
    executor_conf: SharedExecutorSecurityConf,
) -> Result<(), CommonError> {
    let guard = executor_conf.get_bind_scheduler_inner_ref().await;
    let token = guard.as_ref().map(|b| (&b.1).deref());
    let cancel_task_record =
        signed_cancel_task_record.get_cancel_task_record_after_verify(token)?;
    Ok(shared_delay_timer.cancel_task(
        cancel_task_record.task_id as u64,
        cancel_task_record.record_id,
    )?)
}

#[allow(dead_code)]
async fn maintenance(shared_delay_timer: SharedDelayTimer) -> impl Responder {
    HttpResponse::Ok().json(Into::<UnitUnifiedResponseMessages>::into(
        shared_delay_timer.stop_delay_timer(),
    ))
}

//Health Screening
#[post("/api/executor/health_screen")]
async fn health_screen(system_mirror: SharedSystemMirror) -> impl Responder {
    HttpResponse::Ok().json(
        UnifiedResponseMessages::<SystemSnapshot>::success_with_data(
            system_mirror.refresh_all().await,
        ),
    )
}

#[post("/api/executor/bind")]
// Or set security level, no authentication at level 0, public and private keys required at level 1.
async fn bind_executor(
    web::Json(request_bind_scheduler): web::Json<SignedBindRequest>,
    security_conf: web::Data<ExecutorSecurityConf>,
) -> impl Responder {
    let verify_result = request_bind_scheduler.verify(security_conf.get_ref().get_rsa_public_key());
    if verify_result.is_ok() {
        let SignedBindRequest { bind_request, .. } = request_bind_scheduler;

        let token: String = security_conf.generate_token();
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
    // Loads environment variables.
    dotenv().ok();

    let delay_timer = DelayTimerBuilder::default().enable_status_report().build();
    let shared_delay_timer: SharedDelayTimer = ShareData::new(delay_timer);

    let shared_security_conf: SharedExecutorSecurityConf =
        ShareData::new(ExecutorSecurityConf::default());

    // let shared_system_mirror: SharedSystemMirror = ShareData::new(SystemMirror::default());

    HttpServer::new(move || {
        App::new()
            .service(bind_executor)
            .service(create_task)
            .service(remove_task)
            .service(cancel_task)
            .service(advance_task)
            .service(health_screen)
            .app_data(shared_delay_timer.clone())
            .app_data(shared_security_conf.clone())
        // .app_data(shared_system_mirror.clone())
    })
    .bind(
        env::var("EXECUTOR_LISTENING_ADDRESS")
            .expect("Without `EXECUTOR_LISTENING_ADDRESS` set in .env"),
    )?
    .run()
    .await
}
