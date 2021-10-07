mod component;
mod prelude;
use prelude::*;

#[post("/api/task/create")]
#[instrument]
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
    info!("pre_create_task: {}", &signed_task_package.task_package);
    let guard = executor_conf.get_bind_scheduler_token_ref().await;
    let token = guard.as_ref().map(|s| s.deref());
    let task = signed_task_package
        .get_task_package_after_verify(token)
        .map(TryInto::<Task>::try_into)??;

    Ok(shared_delay_timer.add_task(task)?)
}

#[post("/api/task/update")]
#[instrument]
async fn update_task(
    web::Json(signed_task_package): web::Json<SignedTaskPackage>,
    shared_delay_timer: SharedDelayTimer,
    executor_conf: SharedExecutorSecurityConf,
) -> impl Responder {
    let response: UnitUnifiedResponseMessages =
        Into::into(pre_update_task(signed_task_package, shared_delay_timer, executor_conf).await);

    HttpResponse::Ok().json(response)
}

pub async fn pre_update_task(
    signed_task_package: SignedTaskPackage,
    shared_delay_timer: SharedDelayTimer,
    executor_conf: SharedExecutorSecurityConf,
) -> Result<(), CommonError> {
    info!("pre_update_task: {}", &signed_task_package.task_package);
    let guard = executor_conf.get_bind_scheduler_token_ref().await;
    let token = guard.as_ref().map(|s| s.deref());
    let task = signed_task_package
        .get_task_package_after_verify(token)
        .map(TryInto::<Task>::try_into)??;

    Ok(shared_delay_timer.update_task(task)?)
}

#[post("/api/task/remove")]
#[instrument]
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
    info!("pre_remove_task: {}", &signed_task_unit);

    let guard = executor_conf.get_bind_scheduler_token_ref().await;
    let token = guard.as_ref().map(|s| s.deref());
    let task_unit = signed_task_unit.get_task_unit_after_verify(token)?;
    Ok(shared_delay_timer.remove_task(task_unit.task_id as u64)?)
}

#[post("/api/task/advance")]
#[instrument]
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
    info!("pre_advance_task: {}", &signed_task_unit);
    let guard = executor_conf.get_bind_scheduler_token_ref().await;
    let token = guard.as_ref().map(|s| s.deref());
    let task_unit = signed_task_unit.get_task_unit_after_verify(token)?;
    Ok(shared_delay_timer.advance_task(task_unit.task_id as u64)?)
}

#[post("/api/task_instance/kill")]
#[instrument]
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
    info!("pre_cancel_task: {}", &signed_cancel_task_record);

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

// Health Screening

#[post("/api/executor/health_screen")]
#[instrument]
async fn health_screen(
    req: HttpRequest,
    web::Json(signed_health_screen_unit): web::Json<SignedHealthScreenUnit>,
    executor_conf: SharedExecutorSecurityConf,
    system_mirror: SharedSystemMirror,
) -> impl Responder {
    let guard = executor_conf.get_bind_scheduler_token_ref().await;
    let token = guard.as_ref().map(|s| s.deref());

    let verify_result = signed_health_screen_unit.get_health_screen_unit_after_verify(token);
    if let Ok(health_screen_unit) = verify_result {
        let connection = req.connection_info();
        let ip = connection.realip_remote_addr().unwrap_or_default();
        info!("From: {}, Request-time:{}", ip, health_screen_unit);

        let system_snapshot = system_mirror.refresh_all().await;
        let bind_request = executor_conf
            .get_bind_scheduler_inner_ref()
            .await
            .clone()
            .unwrap_or_default();

        let health_check_package = HealthCheckPackage {
            system_snapshot,
            bind_request,
        };
        return HttpResponse::Ok().json(
            UnifiedResponseMessages::<HealthCheckPackage>::success_with_data(health_check_package),
        );
    }

    HttpResponse::Ok().json(
        UnitUnifiedResponseMessages::error()
            .customized_error_msg(verify_result.expect_err("").to_string()),
    )
}

#[post("/api/executor/bind")]
#[instrument]
// Or set security level, no authentication at level 0, public and private keys required at level 1.
async fn bind_executor(
    web::Json(request_bind_scheduler): web::Json<SignedBindRequest>,
    security_conf: web::Data<ExecutorSecurityConf>,
    shared_delay_timer: SharedDelayTimer,
) -> impl Responder {
    info!("{}", &request_bind_scheduler.bind_request);

    let verify_result = request_bind_scheduler.verify(security_conf.get_ref().get_rsa_public_key());
    if verify_result.is_ok() {
        let SignedBindRequest { bind_request, .. } = request_bind_scheduler;

        let token: Option<String> = security_conf.generate_token();

        // Take 10 bits from executor_machine_id and do machine_id and node_id in two groups.

        let executor_machine_id = bind_request.executor_machine_id;
        let extractor: i16 = 0b00_0001_1111;
        let node_id = executor_machine_id & extractor;
        let machine_id = (executor_machine_id >> 5) & extractor;

        shared_delay_timer.update_id_generator_conf(machine_id as i32, node_id as i32);

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
async fn main() -> AnyResult<()> {
    // Loads environment variables.
    dotenv().ok();

    let log_level: Level =
        FromStr::from_str(&env::var("LOG_LEVEL").unwrap_or_else(|_| String::from("info")))
            .expect("Log level acquired fail.");

    FmtSubscriber::builder()
        // will be written to stdout.
        .with_max_level(log_level)
        .with_thread_names(true)
        // completes the builder.
        .init();

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
            .wrap(MiddlewareLogger::default())
    })
    .bind(
        env::var("EXECUTOR_LISTENING_ADDRESS")
            .expect("Without `EXECUTOR_LISTENING_ADDRESS` set in .env"),
    )?
    .run()
    .await?;

    Ok(())
}

fn launch_status_reporter(
    delay_timer: &mut DelayTimer,
    shared_security_conf: SharedExecutorSecurityConf,
) {
    let status_reporter_option = delay_timer.take_status_reporter();

    if let Some(status_reporter) = status_reporter_option {
        rt_spawn(async move {
            // After taking the lock, get the resource quickly and release the lock.

            let mut token: Option<String> = None;
            let mut scheduler: Option<BindRequest> = None;

            loop {
                let f = async {
                    fresh_scheduler_conf(&shared_security_conf, &mut token, &mut scheduler).await;

                    let events = collect_events(&status_reporter, scheduler.as_ref()).await?;

                    if !events.is_empty() {
                        send_event_collection(
                            scheduler.as_ref(),
                            Into::<ExecutorEventCollection>::into(events).sign(token.as_deref()),
                        )
                        .await;
                    }

                    Ok(())
                };
                let f_result: Result<(), CommonError> = f
                    .instrument(span!(
                        Level::INFO,
                        "status-reporter",
                        log_id = get_unique_id_string().deref()
                    ))
                    .await;

                if let Err(e) = f_result {
                    error!("{}", e);
                    return;
                }
            }
        })
    }
}
async fn fresh_scheduler_conf(
    shared_security_conf: &SharedExecutorSecurityConf,
    token: &mut Option<String>,
    scheduler: &mut Option<BindRequest>,
) {
    {
        let scheduler_token = shared_security_conf.get_bind_scheduler_token_ref().await;
        if scheduler_token.as_ref() != token.as_ref() {
            token.clone_from(&scheduler_token);
        }
    }

    {
        let fresh_scheduler = shared_security_conf.get_bind_scheduler_inner_ref().await;
        let fresh_scheduler_time = fresh_scheduler.as_ref().map(|s| s.time);
        let scheduler_time = scheduler.as_ref().map(|s| s.time);

        if fresh_scheduler_time != scheduler_time {
            scheduler.clone_from(&fresh_scheduler);

            // Adjust the internal host to avoid the need to clone String when calling RequestClient::post.
            //+ "/api/task_logs/event_trigger"
            if let Some(scheduler_mut_ref) = scheduler.as_mut() {
                scheduler_mut_ref.scheduler_host += "/api/task_logs/event_trigger";
            }
        }
    }
}

async fn collect_events(
    status_reporter: &StatusReporter,
    scheduler: Option<&BindRequest>,
) -> Result<Vec<ExecutorEvent>, CommonError> {
    let mut events: Vec<ExecutorEvent> = Vec::new();
    for _i in 0..10 {
        let event_future: RtTimeout<_> = rt_timeout(
            Duration::from_secs(3),
            status_reporter.next_public_event_with_async_wait(),
        );

        match event_future.await {
            // No new events and timeout.
            Err(_) => break,
            // Internal runtime exception.
            Ok(Err(_)) => {
                return Err(CommonError::DisPass(
                    "Internal runtime exception".to_string(),
                ));
            }
            Ok(Ok(event)) => {
                scheduler.map(|conf| convert_event(event, conf).map(|e| events.push(e)));
            }
        }
    }

    Ok(events)
}

async fn send_event_collection(
    scheduler: Option<&BindRequest>,
    executor_event_collection_result: Result<SignedExecutorEventCollection, CommonError>,
) {
    if let Some(scheduler_ref) = scheduler.as_ref() {
        if let Ok(executor_event_collection) = executor_event_collection_result {
            debug!(
                "Event collection - {:?}",
                &executor_event_collection.event_collection
            );

            if let Ok(mut response) = RequestClient::new()
                .post(&scheduler_ref.scheduler_host)
                .send_json(&executor_event_collection)
                .await
                .map_err(|e| {
                    error!(
                        "Failed to send the event collection: {} - {} - {:?}",
                        e, &scheduler_ref, &executor_event_collection.event_collection
                    )
                })
            {
                debug!("delicate-schduler response: {:?}", response.body().await)
            }
        }
    }
}

fn convert_event(public_event: PublicEvent, conf: &BindRequest) -> Option<ExecutorEvent> {
    let mut event = delicate_utils::consensus_message::task_log::ExecutorEvent {
        executor_processor_host: conf.executor_processor_host.clone(),
        executor_processor_id: conf.executor_processor_id,
        executor_processor_name: conf.executor_processor_name.clone(),
        ..Default::default()
    };

    match public_event {
        PublicEvent::FinishTask(mut body) => {
            event.id = body.get_record_id();
            event.task_id = body.get_task_id() as i64;
            event.event_type = EventType::TaskFinish as i16;
            event.output = body.get_finish_output().map(|o| o.into());
        }
        PublicEvent::RemoveTask(_) => {
            return None;
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

    Some(event)
}
