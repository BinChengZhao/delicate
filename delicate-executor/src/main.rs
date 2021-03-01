use actix_web::web::{self, Data as ShareData};
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

use delay_timer::prelude::*;
//TODO: delay-timer add `tokio_unblock_process_task_fn` to prelude
use delay_timer::utils::convenience::functions::tokio_unblock_process_task_fn;
use serde::{Deserialize, Serialize};

use anyhow::{anyhow, Error as AnyError, Result as AnyResult};

use std::convert::{TryFrom, TryInto};

type SharedDelayTimer = ShareData<DelayTimer>;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct UnifiedResponseMessages {
    code: i8,
    msg: String,
}
impl UnifiedResponseMessages {
    fn success() -> Self {
        UnifiedResponseMessages::default()
    }

    fn error() -> Self {
        UnifiedResponseMessages {
            code: -1,
            ..Default::default()
        }
    }

    fn customized_error_msg(mut self, msg: String) -> Self {
        self.msg = msg;

        self
    }

    #[allow(dead_code)]
    fn customized_error_code(mut self, code: i8) -> Self {
        self.code = code;

        self
    }

    #[allow(dead_code)]
    fn reverse(mut self) -> Self {
        self.code = -1 - self.code;
        self
    }

    fn init_by_result<T>(result: AnyResult<T>) -> Self {
        match result {
            Ok(_) => Self::success(),
            Err(e) => Self::error().customized_error_msg(e.to_string()),
        }
    }
}

//TODO: err.to_string()
#[derive(Debug, Serialize, Deserialize)]
struct TaskConf {
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
    maximun_parallel_runable_num: u64,
    /// Time zone for cron-expression iteration time.
    schedule_iterator_time_zone: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FrequencyRaw<'a> {
    mode: u8,
    count: u32,
    cron_str: &'a str,
}

impl<'a> FrequencyRaw<'a> {
    fn build_by_task_conf_ref(task_conf_ref: &TaskConf) -> FrequencyRaw {
        FrequencyRaw {
            mode: task_conf_ref.frequency_mode,
            count: task_conf_ref.frequency_count,
            cron_str: &task_conf_ref.cron_str,
        }
    }

    fn get_frequency_by_task_conf_ref(
        task_conf_ref: &'a TaskConf,
    ) -> Result<Frequency<'a>, AnyError> {
        FrequencyRaw::build_by_task_conf_ref(task_conf_ref).try_into()
    }
}

impl<'a> TryInto<Frequency<'a>> for FrequencyRaw<'a> {
    type Error = AnyError;
    fn try_into(self) -> Result<Frequency<'a>, Self::Error> {
        let f = match self.mode {
            0 => Frequency::Repeated(self.cron_str),

            1 => Frequency::CountDown(self.count, self.cron_str),

            2 => Frequency::Once(self.cron_str),

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
        let frequency: Frequency = FrequencyRaw::get_frequency_by_task_conf_ref(&task_conf)?;

        let mut task_builder = TaskBuilder::default();
        let task = task_builder
            .set_task_id(task_conf.task_id)
            .set_frequency(frequency)
            .set_maximum_running_time(task_conf.maximum_running_time)
            .set_maximun_parallel_runable_num(task_conf.maximun_parallel_runable_num)
            .spawn(tokio_unblock_process_task_fn(
                task_conf.command_string.clone(),
            ))?;

        Ok(task)
    }
}

/// This handler uses json extractor
#[get("/add_task")]
async fn add_task(
    task_conf: web::Json<TaskConf>,
    shared_delay_timer: SharedDelayTimer,
) -> impl Responder {
    let mut response = UnifiedResponseMessages::error();
    if let Ok(task) = Task::try_from(task_conf.0) {
        response = UnifiedResponseMessages::init_by_result(shared_delay_timer.add_task(task));
    }

    HttpResponse::Ok().json(response)
}

#[get("/remove_task/{id}")]
async fn remove_task(
    web::Path(task_id): web::Path<u64>,
    shared_delay_timer: SharedDelayTimer,
) -> HttpResponse {
    let response = UnifiedResponseMessages::init_by_result(shared_delay_timer.remove_task(task_id));
    HttpResponse::Ok().json(response) // <- send response
}

#[get("/cancel_task/{task_id}/{record_id}")]
async fn cancel_task(
    web::Path((task_id, record_id)): web::Path<(u64, i64)>,
    shared_delay_timer: SharedDelayTimer,
) -> HttpResponse {
    let response =
        UnifiedResponseMessages::init_by_result(shared_delay_timer.cancel_task(task_id, record_id));
    HttpResponse::Ok().json(response) // <- send response
}

#[allow(dead_code)]
async fn maintenance(shared_delay_timer: SharedDelayTimer) -> impl Responder {
    HttpResponse::Ok().json(UnifiedResponseMessages::init_by_result(
        shared_delay_timer.stop_delay_timer(),
    ))
}

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

//Health Screening
#[get("/health_screen")]
async fn health_screen(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

#[get("/bind_executor")]
// who are you.
// callback_address.
// token.
async fn bind_executor(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let delay_timer = DelayTimerBuilder::default()
        .tokio_runtime(None)
        .enable_status_report()
        .build();

    let shared_delay_timer: SharedDelayTimer = ShareData::new(delay_timer);

    HttpServer::new(move || {
        App::new()
            .service(index)
            .app_data(shared_delay_timer.clone())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
