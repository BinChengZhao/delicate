use actix_web::web::{self, Data as ShareData};
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

use delay_timer::prelude::*;
use serde::{Deserialize, Serialize};

use anyhow::{anyhow, Error as AnyError, Result as AnyResult};

use std::convert::TryInto;

type SharedDelayTimer = ShareData<DelayTimer>;

#[derive(Debug, Debug, Clone, Serialize, Deserialize)]
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

    fn customized_error_code(mut self, code: i8) -> Self {
        self.code = code;

        self
    }

    fn reverse(mut self) -> Self {
        self.code = -1 - self.code;
    }
}
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

impl TryInto<Task> for TaskConf {
    type Error = AnyError;
    fn try_into(self) -> Result<Task, Self::Error> {
        let frequency: Result<Frequency, Self::Error> =
            FrequencyRaw::get_frequency_by_task_conf_ref(&self);

        let mut task_builder = TaskBuilder::default();
        let task = task_builder
            .set_task_id(self.task_id)
            .set_frequency(frequency)
            .set_maximum_running_time(self.maximum_running_time)
            .set_maximun_parallel_runable_num(self.maximun_parallel_runable_num)
            .spawn(tokio_unblock_process_task_fn(self.command_string));

        Ok(task)
    }
}

/// This handler uses json extractor
async fn addTask(item: web::Json<TaskConf>, shared_delay_timer: SharedDelayTimer) -> HttpResponse {
    // If one fn(f1) have a lot of statment is result-structure, but reture-type is other type such as i32.
    // We should move that statment to another fn(f2) ,in there use '?' ops and reture-type is result-structure.
    // fn(f1) just judgment result of fn(f2) returnd. then dosometing.

    //    shared_delay_timer.add_task(TaskConf.try_into());
    HttpResponse::Ok().json(item.0) // <- send response
}

#[get("/removeTask/{id}")]
async fn removeTask(
    web::Path((task_id)): web::Path<(u64)>,
    shared_delay_timer: SharedDelayTimer,
) -> HttpResponse {
    shared_delay_timer.remove_task(task_id);
    HttpResponse::Ok().json(task_id) // <- send response
}

#[get("/cancelTask/{task_id}/{record_id}")]
async fn cancelTask(
    web::Path((task_id, record_id)): web::Path<(u64, u64)>,
    shared_delay_timer: SharedDelayTimer,
) -> HttpResponse {
    HttpResponse::Ok().json(record_id) // <- send response
}

async fn maintenance(shared_delay_timer: SharedDelayTimer) -> HttpResponse {
    shared_delay_timer.stop_delay_timer();
    HttpResponse::Ok().json(task_id) // <- send response
}

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

//Health Screening
#[get("/healthScreen")]
async fn healthScreen(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

#[get("/bindExecutor")]
// who are you.
// callback_address.
// token.
async fn bindExecutor(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let delay_timer = DelayTimerBuilder::default()
        .tokio_runtime(None)
        .enable_status_report()
        .build();

    let shared_delay_timer: SharedDelayTimer = ShareData::new(delay_timer);

    HttpServer::new(move || App::new().service(index))
        .app_data(shared_delay_timer)
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
