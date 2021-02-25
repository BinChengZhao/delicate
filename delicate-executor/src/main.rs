use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use delay_timer::prelude::*;
use serde::{Deserialize, Serialize};

use std::convert::TryInto;

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
    frequency_count: u64,
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

impl<'a> TryInto<Frequency<'a>> for FrequencyRaw<'a> {
    type Error = &'static str;
    fn try_into(self) -> Result<Frequency<'a>, Self::Error> {
        let f = match self.mode {
            0 => Frequency::Repeated(self.cron_str),

            1 => Frequency::CountDown(self.count, self.cron_str),

            2 => Frequency::Once(self.cron_str),

            _ => {
                return Err("Frequency-mode missed.");
            }
        };

        Ok(f)
    }
}

impl TryInto<Task> for TaskConf {
    type Error = ();
    fn try_into(self) -> Result<Task, Self::Error> {
        todo!()
    }
}

/// This handler uses json extractor
async fn addTask(item: web::Json<TaskConf>) -> HttpResponse {
    println!("model: {:?}", &item);
    HttpResponse::Ok().json(item.0) // <- send response
}

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
