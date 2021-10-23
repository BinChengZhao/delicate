pub use delicate_utils::prelude::*;
pub use prost::Message;
pub use prost_types::Any;
pub use std::env;
use std::io::{self, BufRead, BufReader, Read};
pub use std::str::FromStr;
pub use tonic::{transport::Server, Request, Response, Status};
pub use tracing::{debug, info, Level};
pub use tracing_subscriber::FmtSubscriber;

pub use actuator::actuator_client::ActuatorClient;
pub use actuator::{Task, UnifiedResponseMessages};
pub mod actuator {
    include!("../../proto/generated_codes/delicate.actuator.rs");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut action: String;
    let mut task_id: i64;
    let mut task_name: String;
    let mut command: String;
    let mut client = ActuatorClient::connect("http://[::1]:8899").await?;
    let stdin = io::stdin();
    let mut buffer = BufReader::new(stdin);
    let mut lines = buffer.lines();

    init_logger();

    loop {
        println!("Please operate action:");
        let action = lines.next().expect("").expect("");
        println!("Please operate task-id:");
        task_id = lines.next().expect("").expect("").parse().expect("");

        println!("Please operate task-name:");
        task_name = lines.next().expect("").expect("");

        println!("Please operate task-command:");
        command = lines.next().expect("").expect("");
    }
    Ok(())
}

fn init_logger() {
    let log_level: Level =
        FromStr::from_str(&env::var("LOG_LEVEL").unwrap_or_else(|_| String::from("info")))
            .expect("Log level acquired fail.");

    FmtSubscriber::builder()
        // will be written to stdout.
        .with_max_level(log_level)
        .with_thread_names(true)
        // completes the builder.
        .init();
}
