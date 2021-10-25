pub use delicate_utils::prelude::*;
pub use prost::Message;
pub use prost_types::Any;
pub use std::env;
use std::io::{self, BufRead, BufReader};
use std::ops::Deref;
pub use std::str::FromStr;
pub use tonic::{transport::Server, Request, Response, Status};
pub use tracing::{debug, info, Level};
pub use tracing_subscriber::FmtSubscriber;

pub use actuator::{actuator_client::ActuatorClient, Task};
pub use delicate_utils::prelude::*;

use tokio_stream::StreamExt;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Loads environment variables.
    dotenv().ok();

    init_logger();

    let mut action: String;
    let mut id: u64;
    let mut name: String;
    let mut command: String;
    let mut client = ActuatorClient::connect("http://[::1]:8899").await?;
    let stdin = io::stdin();
    let buffer = BufReader::new(stdin);
    let mut lines = buffer.lines();

    loop {
        info!("|------------------------Welcom Friends----------------------------|");
        info!("");
        info!("");
        info!("Please operate action:");
        action = lines.next().expect("").expect("");
        info!("Please operate task-id:");
        id = lines.next().expect("").expect("").parse().expect("");

        info!("Please operate task-name:");
        name = lines.next().expect("").expect("");

        info!("Please operate task-command:");
        command = lines.next().expect("").expect("");

        match action.deref() {
            "create" => {
                let task = Task { id, name, command };

                let response = client.add_task(Request::new(task)).await?;

                debug!("{:?}", response.get_ref());
            }

            "cancel" => {}

            "keep_running" => {
                let task = Task { id, name, command };

                let response = client.keep_running(Request::new(task)).await?;

                let mut stream = response.into_inner();
                while let Some(u) = stream.next().await {
                    info!("{:?}", u.as_ref().expect(""));
                }
            }

            _ => {}
        }
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
        // completes the builder.
        .init();
}
