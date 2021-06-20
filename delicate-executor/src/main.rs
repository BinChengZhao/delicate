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
    let guard = executor_conf.get_bind_scheduler_token_ref().await;
    let token = guard.as_ref().map(|s| s.deref());
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
    let guard = executor_conf.get_bind_scheduler_token_ref().await;
    let token = guard.as_ref().map(|s| s.deref());
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
    let guard = executor_conf.get_bind_scheduler_token_ref().await;
    let token = guard.as_ref().map(|s| s.deref());
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
    let guard = executor_conf.get_bind_scheduler_token_ref().await;
    let token = guard.as_ref().map(|s| s.deref());
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
    shared_delay_timer: SharedDelayTimer,
) -> impl Responder {
    let verify_result = request_bind_scheduler.verify(security_conf.get_ref().get_rsa_public_key());
    if verify_result.is_ok() {
        let SignedBindRequest { bind_request, .. } = request_bind_scheduler;

        let token: Option<String> = security_conf.generate_token();

        // FIXME:
        // first erase the first 6 bits.
        // then take two sets of ids.
        // bind_request.executor_machine_id;
        shared_delay_timer.update_id_generator_conf(1, 1);

        *security_conf.get_bind_scheduler_inner_mut().await = Some(bind_request);
        *security_conf.get_bind_scheduler_token_mut().await = token.clone();

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

    let shared_security_conf: SharedExecutorSecurityConf =
        ShareData::new(ExecutorSecurityConf::default());

    let shared_system_mirror: SharedSystemMirror = ShareData::new(SystemMirror::default());

    let mut delay_timer = DelayTimerBuilder::default().enable_status_report().build();
    launch_status_reporter(&mut delay_timer, shared_security_conf.clone());
    let shared_delay_timer: SharedDelayTimer = ShareData::new(delay_timer);

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
            .app_data(shared_system_mirror.clone())
    })
    .bind(
        env::var("EXECUTOR_LISTENING_ADDRESS")
            .expect("Without `EXECUTOR_LISTENING_ADDRESS` set in .env"),
    )?
    .run()
    .await
}

fn launch_status_reporter(
    delay_timer: &mut DelayTimer,
    _shared_security_conf: SharedExecutorSecurityConf,
) {
    let status_reporter_option = delay_timer.take_status_reporter();

    if let Some(status_reporter) = status_reporter_option {
        rt_spawn(async move {
            // After taking the lock, get the resource quickly and release the lock.

            let mut _scheduler: Option<BindRequest> = None;
            let _convert_event = |public_event: PublicEvent, _conf: &BindRequest| {
                let mut event = ExecutorEvent::default();
                match public_event {
                    PublicEvent::FinishTask(mut body) => {
                        event.id = body.get_record_id();
                        event.task_id = body.get_task_id() as i64;
                        event.event_type = EventType::TaskFinish as i16;
                        event.output = body.get_finish_output().map(|o| o.into());
                    }
                    PublicEvent::RemoveTask(_task_id) => {
                        todo!();
                    }
                    PublicEvent::RunningTask(task_id, record_id) => {
                        event.id = record_id;
                        event.task_id = task_id as i64;
                        event.event_type = EventType::TaskPerform as i16;
                    }
                    PublicEvent::TimeoutTask(task_id, record_id) => {
                        event.id = record_id;
                        event.task_id = task_id as i64;
                        event.event_type = EventType::TaskTimeout as i16;
                    }
                }
            };

            loop {
                //     let event : Vec<i8> = Vec::new();
                for _ in 0..10 {
                    let event_future = rt_timeout(
                        Duration::from_secs(3),
                        status_reporter.next_public_event_with_async_wait(),
                    );

                    match event_future.await {
                        Err(_) => break,
                        Ok(Err(_)) => {
                            return;
                        }
                        Ok(Ok(_event)) => {}
                    }
                    //         if timeout(future).is_err{
                    //             break;
                    //         }

                    //         if ok(result){
                    //            if result.is_err(){
                    //                // channel close.
                    //                 return;
                    //             }
                    //             else{
                    //                 event.push(1);
                    //             }
                    //         }
                }

                //     if !event.is_empty(){
                //         client.send();
                //     }
            }
            // while let Ok(event) = status_reporter.next_public_event_with_async_wait().await {
            //     if let Some(scheduler_guard) = shared_security_conf
            //         .get_bind_scheduler_inner_ref()
            //         .await
            //         .as_ref()
            //     {
            //         if &scheduler_host != &scheduler_guard.scheduler_host {
            //             Clone::clone_from(&mut scheduler_host, &scheduler_guard.scheduler_host);
            //         }
            //     }

            //     if !scheduler_host.is_empty() {
            //         // RequestClient::new()
            //         //     .post(&self, scheduler_host + "/api/task_logs/event_trigger")
            //         //     .send_json(1)
            //         //     .await;
            //         todo!();
            //     }
            // }
        })
    }
}
