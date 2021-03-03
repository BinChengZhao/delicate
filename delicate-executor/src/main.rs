use actix_web::web::{self, Data as ShareData};
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

use delay_timer::prelude::*;
//TODO: delay-timer add `tokio_unblock_process_task_fn` to prelude
use delay_timer::utils::convenience::functions::tokio_unblock_process_task_fn;
use serde::{Deserialize, Serialize};

use anyhow::{anyhow, Error as AnyError};

use std::convert::{From, Into, TryFrom, TryInto};
use std::net::IpAddr;

mod component;
use component::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Scheduler {
    name: String,
    ip: IpAddr,
    port: u16,
    domin: String,
    callback_url: String,
    // private_key.decrypt(raw_token) = "ip:port:token" when security_level = SecurityLevel::Normal.
    // raw_token = "ip:port:token" when security_level = SecurityLevel::ZeroRestriction.
    raw_token: String,
}

impl Scheduler {
    fn verify(&self, security_level: SecurityLevel) -> bool {
        todo!()
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

impl<'a> From<&'a TaskConf> for FrequencyRaw<'a> {
    fn from(value: &'a TaskConf) -> Self {
        FrequencyRaw {
            mode: value.frequency_mode,
            count: value.frequency_count,
            cron_str: &value.cron_str,
        }
    }
}

impl<'a> TryFrom<FrequencyRaw<'a>> for Frequency<'a> {
    type Error = AnyError;
    fn try_from(value: FrequencyRaw<'a>) -> Result<Self, Self::Error> {
        let f = match value.mode {
            0 => Frequency::Repeated(value.cron_str),

            1 => Frequency::CountDown(value.count, value.cron_str),

            2 => Frequency::Once(value.cron_str),

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
        response = shared_delay_timer.add_task(task).into();
    }

    HttpResponse::Ok().json(response)
}

#[get("/remove_task/{id}")]
async fn remove_task(
    web::Path(task_id): web::Path<u64>,
    shared_delay_timer: SharedDelayTimer,
) -> HttpResponse {
    let response: UnifiedResponseMessages = shared_delay_timer.remove_task(task_id).into();
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
    HttpResponse::Ok().json(Into::<UnifiedResponseMessages>::into(
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

// Or use middleware to reach consensus.
// Register token at executor startup, check token when scheduler bind executor.

// Or set security level, no authentication at level 0, public and private keys required at level 1.
async fn bind_executor(
    scheduler: web::Json<Scheduler>,
    delicate_conf: web::Data<DelicateConf>,
) -> impl Responder {
    scheduler
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
            .service(bind_executor)
            .service(add_task)
            .service(remove_task)
            .service(cancel_task)
            .service(health_screen)
            .app_data(shared_delay_timer.clone())
            .data(|| DelicateConf::default())
    })
    .bind("127.0.0.1:8090")?
    .run()
    .await
}
