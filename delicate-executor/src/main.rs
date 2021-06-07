mod component;
mod prelude;
use prelude::*;

#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct TaskConf {
    /// Task_id should unique.
    task_id: u64,
    /// Command string.
    command_string: String,
    /// Cron-expression str.
    cron_str: String,
    /// Repeat type.
    frequency_mode: u8,
    /// Repeat count.
    frequency_count: u32,
    /// Maximum execution time (optional).
    /// it can be use to deadline (excution-time + maximum_running_time).
    maximum_running_time: u64,
    /// Maximum parallel runable num (optional).
    maximun_parallel_runnable_num: u64,
    /// Time zone for cron-expression iteration time.
    schedule_iterator_time_zone: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FrequencyRaw<'a> {
    mode: u8,
    count: u32,
    cron_str: &'a str,
}

impl<'a> From<&'a TaskConf> for FrequencyRaw<'a> {
    fn from(value: &'a TaskConf) -> Self {
        FrequencyRaw {
            mode: value.frequency_mode,
            count: value.frequency_count,
            cron_str: &value.cron_str,
        }
    }
}

#[allow(dead_code)]
pub(crate) enum FrequencyModelType {
    Once = 1,
    CountDown = 2,
    Repeat = 3,
}

impl<'a> TryFrom<FrequencyRaw<'a>> for Frequency<'a> {
    type Error = AnyError;
    fn try_from(value: FrequencyRaw<'a>) -> Result<Self, Self::Error> {
        let f = match value.mode {
            1 => Frequency::Once(value.cron_str),
            2 => Frequency::CountDown(value.count, value.cron_str),
            3 => Frequency::Repeated(value.cron_str),

            _ => {
                return Err(anyhow!("Frequency-mode missed."));
            }
        };

        Ok(f)
    }
}

impl TryFrom<TaskConf> for Task {
    type Error = AnyError;
    fn try_from(task_conf: TaskConf) -> Result<Self, Self::Error> {
        let frequency: Frequency = FrequencyRaw::from(&task_conf).try_into()?;

        let mut task_builder = TaskBuilder::default();
        let task = task_builder
            .set_task_id(task_conf.task_id)
            .set_frequency(frequency)
            .set_maximum_running_time(task_conf.maximum_running_time)
            .set_maximun_parallel_runable_num(task_conf.maximun_parallel_runnable_num)
            .spawn(unblock_process_task_fn(task_conf.command_string.clone()))?;

        Ok(task)
    }
}

/// This handler uses json extractor
#[get("/add_task")]
async fn add_task(
    task_conf: web::Json<TaskConf>,
    shared_delay_timer: SharedDelayTimer,
) -> impl Responder {
    let mut response = UnitUnifiedResponseMessages::error();
    if let Ok(task) = Task::try_from(task_conf.0) {
        response = shared_delay_timer.add_task(task).into();
    }

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
    delicate_shared_scheduler: SharedBindScheduler,
    security_conf: web::Data<ExecutorSecurityConf>,
) -> impl Responder {
    let verify_result = request_bind_scheduler.verify(security_conf.get_ref().get_rsa_public_key());
    if verify_result.is_ok() {
        let SignedBindRequest { bind_request, .. } = request_bind_scheduler;

        let token: String = repeat_with(fastrand::alphanumeric).take(32).collect();
        delicate_shared_scheduler
            .inner
            .write()
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
    let shared_scheduler: SharedBindScheduler = ShareData::new(BindScheduler::default());
    let shared_security_conf: SharedExecutorSecurityConf =
        ShareData::new(ExecutorSecurityConf::default());
    let shared_system_mirror: SharedSystemMirror = ShareData::new(SystemMirror::default());

    HttpServer::new(move || {
        App::new()
            .service(bind_executor)
            .service(add_task)
            .service(remove_task)
            .service(cancel_task)
            .service(health_screen)
            .app_data(shared_delay_timer.clone())
            .app_data(shared_scheduler.clone())
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
