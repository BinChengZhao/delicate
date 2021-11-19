pub use std::{env, str::FromStr};
use std::{
    io::{self, BufRead, BufReader},
    ops::Deref,
};

pub use actuator::{actuator_client::ActuatorClient, RecordId, Task};
pub use delicate_utils::prelude::*;
pub use prost::Message;
pub use prost_types::Any;
pub use tokio_stream::StreamExt;
pub use tonic::{transport::Server, Request, Response, Status};
pub use tracing::{debug, info, Level};
pub use tracing_subscriber::FmtSubscriber;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Loads environment variables.
    dotenv().ok();

    init_logger();

    let mut action: String;
    let mut id: i64;
    let mut name: String;
    let mut command: String;
    let timeout = 60;
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
        info!("Please operate task-id or record-id :");
        id = lines.next().expect("").expect("").parse().expect("");

        info!("Please operate task-name:");
        name = lines.next().expect("").expect("");

        info!("Please operate task-command:");
        command = lines.next().expect("").expect("");

        match action.deref() {
            "create" => {
                let task = Task { id, name, command, timeout };

                let response = client.run_task(Request::new(task)).await?;

                debug!("{:?}", response.get_ref());
            },

            "cancel" => {
                let response = client.cancel_task(Request::new(RecordId { id })).await?;

                debug!("{:?}", response.get_ref());
            },

            "keep" => {
                let task = Task { id, name, command, timeout };

                let response = client.keep_running_task(Request::new(task)).await?;

                let mut stream = response.into_inner();
                while let Some(u) = stream.next().await {
                    info!("{:?}", u.as_ref().expect(""));
                }
            },

            _ => {
                break;
            },
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
