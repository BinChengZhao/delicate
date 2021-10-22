use tonic::{transport::Server, Request, Response, Status};

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
        _request: Request<Task>,
    ) -> Result<Response<UnifiedResponseMessages>, Status> {
        Err(Status::data_loss("sleep."))
    }
}

// ./grpcurl -plaintext -import-path ./delicate/delicate-actuator/proto -proto actuator.proto -d '{"task_id":1, "task_name": "Tonic", "command": "sleep" }' "[::1]:8899" delicate.actuator.Actuator/AddTask

// TODO:
// Objectives.

// 2. Implement the actuator, via tonic.
// 3. Prioritize minimalist implementations, supporting only single machines at first, with subsequent support for slicing or various rules.
// 4. advertise on `poem` Readme.
// 5. add/delete/change/check/cancel for standalone tasks Do it first.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Server::builder()
        .add_service(ActuatorServer::new(DelicateActuator))
        .serve("[::1]:8899".parse().expect(""))
        .await?;
    Ok(())
}
