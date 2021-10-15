#![allow(clippy::async_yields_async)]
mod component;
mod prelude;
use prelude::*;

#[handler]
#[instrument(skip(executor_conf, shared_delay_timer, signed_task_package), fields(task_package = signed_task_package.task_package.id))]
async fn create_task(
    Json(signed_task_package): Json<SignedTaskPackage>,
    shared_delay_timer: Data<&DelayTimer>,
    executor_conf: Data<&ExecutorSecurityConf>,
) -> Json<UnitUnifiedResponseMessages> {
    let response: UnitUnifiedResponseMessages =
        Into::into(pre_create_task(signed_task_package, shared_delay_timer, executor_conf).await);

    Json(response)
}

pub async fn pre_create_task(
    signed_task_package: SignedTaskPackage,
    shared_delay_timer: Data<&DelayTimer>,
    executor_conf: Data<&ExecutorSecurityConf>,
) -> Result<(), CommonError> {
    info!("pre_create_task: {}", &signed_task_package.task_package);
    let guard = executor_conf.get_bind_scheduler_token_ref().await;
    let token = guard.as_ref().map(|s| s.deref());
    let task = signed_task_package
        .get_task_package_after_verify(token)
        .map(TryInto::<Task>::try_into)??;

    Ok(shared_delay_timer.add_task(task)?)
}

#[handler]
#[instrument(skip(executor_conf, shared_delay_timer, signed_task_package), fields(task_package = signed_task_package.task_package.id))]
async fn update_task(
    Json(signed_task_package): Json<SignedTaskPackage>,
    shared_delay_timer: Data<&DelayTimer>,
    executor_conf: Data<&ExecutorSecurityConf>,
) -> Json<UnitUnifiedResponseMessages> {
    let response: UnitUnifiedResponseMessages =
        Into::into(pre_update_task(signed_task_package, shared_delay_timer, executor_conf).await);

    Json(response)
}

pub async fn pre_update_task(
    signed_task_package: SignedTaskPackage,
    shared_delay_timer: Data<&DelayTimer>,
    executor_conf: Data<&ExecutorSecurityConf>,
) -> Result<(), CommonError> {
    info!("pre_update_task: {}", &signed_task_package.task_package);
    let guard = executor_conf.get_bind_scheduler_token_ref().await;
    let token = guard.as_ref().map(|s| s.deref());
    let task = signed_task_package
        .get_task_package_after_verify(token)
        .map(TryInto::<Task>::try_into)??;

    Ok(shared_delay_timer.update_task(task)?)
}

#[handler]
#[instrument(skip(executor_conf, shared_delay_timer, signed_task_unit), fields(task_id = signed_task_unit.task_unit.task_id))]
async fn remove_task(
    Json(signed_task_unit): Json<SignedTaskUnit>,
    shared_delay_timer: Data<&DelayTimer>,
    executor_conf: Data<&ExecutorSecurityConf>,
) -> Json<UnitUnifiedResponseMessages> {
    let response: UnitUnifiedResponseMessages =
        pre_remove_task(signed_task_unit, shared_delay_timer, executor_conf)
            .await
            .into();
    Json(response)
}

pub async fn pre_remove_task(
    signed_task_unit: SignedTaskUnit,
    shared_delay_timer: Data<&DelayTimer>,
    executor_conf: Data<&ExecutorSecurityConf>,
) -> Result<(), CommonError> {
    info!("pre_remove_task: {}", &signed_task_unit);

    let guard = executor_conf.get_bind_scheduler_token_ref().await;
    let token = guard.as_ref().map(|s| s.deref());
    let task_unit = signed_task_unit.get_task_unit_after_verify(token)?;
    Ok(shared_delay_timer.remove_task(task_unit.task_id as u64)?)
}

#[handler]
#[instrument(skip(executor_conf, shared_delay_timer, signed_task_unit), fields(task_id = signed_task_unit.task_unit.task_id))]
async fn advance_task(
    Json(signed_task_unit): Json<SignedTaskUnit>,
    shared_delay_timer: Data<&DelayTimer>,
    executor_conf: Data<&ExecutorSecurityConf>,
) -> Json<UnitUnifiedResponseMessages> {
    let response: UnitUnifiedResponseMessages =
        pre_advance_task(signed_task_unit, shared_delay_timer, executor_conf)
            .await
            .into();
    Json(response)
}

pub async fn pre_advance_task(
    signed_task_unit: SignedTaskUnit,
    shared_delay_timer: Data<&DelayTimer>,
    executor_conf: Data<&ExecutorSecurityConf>,
) -> Result<(), CommonError> {
    info!("pre_advance_task: {}", &signed_task_unit);
    let guard = executor_conf.get_bind_scheduler_token_ref().await;
    let token = guard.as_ref().map(|s| s.deref());
    let task_unit = signed_task_unit.get_task_unit_after_verify(token)?;
    Ok(shared_delay_timer.advance_task(task_unit.task_id as u64)?)
}

#[handler]
#[instrument(skip(executor_conf, shared_delay_timer, signed_cancel_task_record), fields(cancel_task_record = signed_cancel_task_record.cancel_task_record.to_string().deref()))]
async fn cancel_task(
    Json(signed_cancel_task_record): Json<SignedCancelTaskRecord>,
    shared_delay_timer: Data<&DelayTimer>,
    executor_conf: Data<&ExecutorSecurityConf>,
) -> Json<UnitUnifiedResponseMessages> {
    let response: UnitUnifiedResponseMessages =
        pre_cancel_task(signed_cancel_task_record, shared_delay_timer, executor_conf)
            .await
            .into();
    Json(response)
}

pub async fn pre_cancel_task(
    signed_cancel_task_record: SignedCancelTaskRecord,
    shared_delay_timer: Data<&DelayTimer>,
    executor_conf: Data<&ExecutorSecurityConf>,
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
async fn maintenance(shared_delay_timer: Data<&DelayTimer>) -> Json<UnitUnifiedResponseMessages> {
    Json(Into::<UnitUnifiedResponseMessages>::into(
        shared_delay_timer.stop_delay_timer(),
    ))
}

// Health Screening
#[handler]
#[instrument(skip(req, signed_health_screen_unit, executor_conf, system_mirror), fields(time = signed_health_screen_unit.health_screen_unit.time))]
async fn health_screen(
    req: &Request,
    Json(signed_health_screen_unit): Json<SignedHealthScreenUnit>,
    executor_conf: Data<&ExecutorSecurityConf>,
    system_mirror: Data<&SystemMirror>,
) -> Json<UnifiedResponseMessages<HealthCheckPackage>> {
    let guard = executor_conf.get_bind_scheduler_token_ref().await;
    let token = guard.as_ref().map(|s| s.deref());

    let verify_result = signed_health_screen_unit.get_health_screen_unit_after_verify(token);
    if let Ok(health_screen_unit) = verify_result {
        let ip = req.remote_addr();
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
        return Json(
            UnifiedResponseMessages::<HealthCheckPackage>::success_with_data(health_check_package),
        );
    }

    Json(
        UnifiedResponseMessages::<HealthCheckPackage>::error()
            .customized_error_msg(verify_result.expect_err("").to_string()),
    )
}

#[handler]
#[instrument(skip(request_bind_scheduler, security_conf, shared_delay_timer), fields(bind_scheduler = request_bind_scheduler.bind_request.to_string().deref()))]
// Or set security level, no authentication at level 0, public and private keys required at level 1.
async fn bind_executor(
    Json(request_bind_scheduler): Json<SignedBindRequest>,
    security_conf: Data<&ExecutorSecurityConf>,
    shared_delay_timer: Data<&DelayTimer>,
) -> Json<UnifiedResponseMessages<EncryptedBindResponse>> {
    info!("{}", &request_bind_scheduler.bind_request);

    let verify_result = request_bind_scheduler.verify(security_conf.get_rsa_public_key());
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
        return Json(response);
    }

    Json(
        UnifiedResponseMessages::<EncryptedBindResponse>::error()
            .customized_error_msg(verify_result.expect_err("").to_string()),
    )
}

fn main() -> AnyResult<()> {
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

    let raw_runtime = Builder::new_multi_thread()
        .thread_name_fn(|| {
            static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
            let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
            format!("executor-{}", id)
        })
        .thread_stack_size(4 * 1024 * 1024)
        .build()
        .expect("Init Tokio runtime failed.");

    let arc_runtime = Arc::new(raw_runtime);
    let arc_runtime_cloned = arc_runtime.clone();

    let block_result: AnyResult<()> = arc_runtime.block_on(async move {
        let route = Route::new()
            .at("/api/task/update", post(update_task))
            .at("/api/task/create", post(create_task))
            .at("/api/task/remove", post(remove_task))
            .at("/api/task/advance", post(advance_task))
            .at("/api/task_instance/kill", post(cancel_task))
            .at("/api/executor/health_screen", post(health_screen))
            .at("/api/executor/bind", post(bind_executor));

        let app = init_executor(route, arc_runtime_cloned).await;
        let address = env::var("EXECUTOR_LISTENING_ADDRESS")
            .expect("Without `EXECUTOR_LISTENING_ADDRESS` set in .env");
        let listener = TcpListener::bind(address);
        let server = Server::new(listener).await?;
        Ok(server.run(app).await?)
    });

    block_result
}

async fn init_executor(app: Route, arc_runtime: Arc<Runtime>) -> impl Endpoint {
    let arc_security_conf = Arc::new(ExecutorSecurityConf::default());
    let shared_security_conf: AddData<Arc<ExecutorSecurityConf>> =
        AddData::new(arc_security_conf.clone());

    let shared_system_mirror: AddData<Arc<SystemMirror>> =
        AddData::new(Arc::new(SystemMirror::default()));

    let mut delay_timer = DelayTimerBuilder::default()
        .tokio_runtime_shared_by_custom(arc_runtime)
        .enable_status_report()
        .build();
    let request_client = RequestClient::new();
    let shared_request_client = AddData::new(request_client.clone());
    launch_status_reporter(&mut delay_timer, arc_security_conf, request_client);
    let shared_delay_timer: AddData<Arc<DelayTimer>> = AddData::new(Arc::new(delay_timer));

    app.with(shared_delay_timer)
        .with(shared_security_conf)
        .with(shared_system_mirror)
        .with(shared_request_client)
}
fn launch_status_reporter(
    delay_timer: &mut DelayTimer,
    shared_security_conf: Arc<ExecutorSecurityConf>,
    client: RequestClient,
) {
    let status_reporter_option = delay_timer.take_status_reporter();

    if let Some(status_reporter) = status_reporter_option {
        tokio_spawn(async move {
            // After taking the lock, get the resource quickly and release the lock.

            let mut token: Option<String> = None;
            let mut scheduler: Option<BindRequest> = None;

            loop {
                let f = async {
                    fresh_scheduler_conf(&shared_security_conf, &mut token, &mut scheduler).await;

                    let events = collect_events(&status_reporter, scheduler.as_ref()).await?;

                    if events.is_empty() {
                        return Ok(());
                    }

                    if let Ok(executor_event_collection) =
                        Into::<ExecutorEventCollection>::into(events).sign(token.as_deref())
                    {
                        send_event_collection(
                            scheduler.as_ref(),
                            executor_event_collection,
                            &client,
                        )
                        .await;
                    }

                    Ok(())
                };
                let f_result: Result<(), NewCommonError> = f
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
        });
    }
}
async fn fresh_scheduler_conf(
    shared_security_conf: &ExecutorSecurityConf,
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
            //+ "/api/task_log/event_trigger"
            if let Some(scheduler_mut_ref) = scheduler.as_mut() {
                scheduler_mut_ref.scheduler_host += "/api/task_log/event_trigger";
            }
        }
    }
}

async fn collect_events(
    status_reporter: &StatusReporter,
    scheduler: Option<&BindRequest>,
) -> Result<Vec<ExecutorEvent>, NewCommonError> {
    let mut events: Vec<ExecutorEvent> = Vec::new();
    for _i in 0..10 {
        let event_future: TokioTimeout<_> = tokio_timeout(
            Duration::from_secs(3),
            status_reporter.next_public_event_with_async_wait(),
        );

        match event_future.await {
            // No new events and timeout.
            Err(_) => break,
            // Internal runtime exception.
            Ok(Err(_)) => {
                return Err(NewCommonError::DisPass(
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
    executor_event_collection: SignedExecutorEventCollection,
    client: &RequestClient,
) {
    if let Some(scheduler_ref) = scheduler.as_ref() {
        debug!(
            "Event collection - {:?}",
            &executor_event_collection.event_collection
        );

        if let Ok(response) = client
            .post(&scheduler_ref.scheduler_host)
            .json(&executor_event_collection)
            .send()
            .await
            .map_err(|e| {
                error!(
                    "Failed to send the event collection: {} - {} - {:?}",
                    e, &scheduler_ref, &executor_event_collection.event_collection
                )
            })
        {
            response
                .bytes()
                .await
                .map(|b| debug!("delicate-schduler response: {:?}", b))
                .ok();
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
