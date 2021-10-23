use delicate_utils::prelude::*;
use prost::Message;
use prost_types::Any;
use std::env;
use std::str::FromStr;
use tonic::{transport::Server, Request, Response, Status};
use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;

use actuator::actuator_server::{Actuator, ActuatorServer};
use actuator::{Task, UnifiedResponseMessages};
pub mod actuator {
    include!("../proto/generated_codes/delicate.actuator.rs");
}

#[derive(Debug, Copy, Clone)]
struct DelicateActuator;

#[tonic::async_trait]
impl Actuator for DelicateActuator {
    async fn add_task(
        &self,
        request: Request<Task>,
    ) -> Result<Response<UnifiedResponseMessages>, Status> {
        let task_ref = request.get_ref();
        let task_message = task_ref.encode_to_vec();

        info!("{:?}", task_ref);

        let type_url = if task_ref.task_id == 1 {
            String::from("/delicate.actuator.Task")
        } else {
            // If type_url is set incorrectly, then the data will not be deserialized properly.
            String::from("/Task")
        };

        let any = Any {
            type_url,
            value: task_message,
        };

        let mut res = UnifiedResponseMessages {
            code: 1,
            msg: String::from("hahahaha"),
            ..Default::default()
        };
        res.data.push(any);

        Ok(Response::new(res))
    }
}

// ./grpcurl -plaintext -import-path ./delicate/delicate-actuator/proto -proto actuator.proto -d '{"task_id":1, "task_name": "Tonic", "command": "sleep" }' "[::1]:8899" delicate.actuator.Actuator/AddTask

// TODO:
// Objectives.

// 2. Implement the actuator, via tonic.
// 3. Prioritize minimalist implementations, supporting only single machines at first, with subsequent support for slicing or various rules.
// 4. advertise on `poem` Readme.
// 5. add/delete/change/check/cancel for standalone tasks Do it first.

// scheduler & actuator interaction, also send/register task,
// return response carrying record-id,
// actuator replies to events after execution is completed.

// The actuator maintains the state of the task internally,
// kills the task directly if it times out, and returns the timeout event.

// The actuator supports cancel of tasks, using task.abort();

// Provide a task test execution, direct transfer between frontend & scheduler & actuator, dynamic rendering of pages, no database storage.
// SSE: (frontend & scheduler)
// GRPC-stream: (scheduler & actuator)
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger();
    Server::builder()
        .add_service(ActuatorServer::new(DelicateActuator))
        .serve("[::1]:8899".parse().expect(""))
        .await?;
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
