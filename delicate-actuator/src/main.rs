mod prelude;
use prelude::*;

#[derive(Debug, Copy, Clone)]
struct DelicateActuator;

#[tonic::async_trait]
impl Actuator for DelicateActuator {
    async fn add_task(
        &self,
        request: Request<Task>,
    ) -> Result<Response<UnifiedResponseMessagesForGPRC>, Status> {
        let task_ref = request.get_ref();

        info!("{:?}", task_ref);

        let mut res = UnifiedResponseMessagesForGPRC {
            code: 1,
            msg: String::from("hahahaha"),
            ..Default::default()
        };

        {
            let value = task_ref.encode_to_vec();
            let type_url = String::from("/delicate.actuator.Task");
            res.data.push(Any { type_url, value });
        }

        {
            let value = String::from("I'm string").into_bytes();
            let type_url = String::from("/String");
            res.data.push(Any { type_url, value });
        }

        {
            let value = String::from("I'm Fake , have no exist.").into_bytes();
            let type_url = String::from("/Fake");
            res.data.push(Any { type_url, value });
        }

        Ok(Response::new(res))
    }

    type KeepRunningStream = Pin<
        Box<
            dyn Stream<Item = Result<UnifiedResponseMessagesForGPRC, Status>>
                + Send
                + Sync
                + 'static,
        >,
    >;
    async fn keep_running(
        &self,
        request: Request<Task>,
    ) -> Result<Response<Self::KeepRunningStream>, Status> {
        let task = request.into_inner();
        let mut process_linked_list = parse_and_run::<TokioChild, TokioCommand>(&task.command)
            .await
            .map_err(|e| Status::failed_precondition(e.to_string()))?;

        let child_guard = process_linked_list
            .pop_back()
            .ok_or_else(|| Status::failed_precondition("Have no process executed.".to_string()))?;

        let child = child_guard.take_inner().ok_or_else(|| {
            Status::failed_precondition(" No valid process execution .".to_string())
        })?;

        let child_stdout = child
            .stdout
            .ok_or_else(|| Status::failed_precondition(" No valid process stdout .".to_string()))?;

        let mut buf_reader_lines =
            LinesStream::new(BufReader::new(child_stdout).lines()).map(|l| {
                l.map(|s| {
                    let type_url = "/String".to_string();
                    let value = s.encode_to_vec();
                    let any = Any { type_url, value };
                    let data = vec![any];
                    UnifiedResponseMessagesForGPRC {
                        data,
                        ..Default::default()
                    }
                })
                .map_err(|e| Status::unknown(e.to_string()))
            });

        let stream = async_stream::stream! {

            while let Some(resp) = buf_reader_lines.next().await{
                yield resp;
            }
        };

        Ok(Response::new(Box::pin(stream) as Self::KeepRunningStream))
    }
}

// ./grpcurl -plaintext -import-path ./delicate/delicate-utils/proto -proto actuator.proto -d '{"id":1, "name": "Tonic", "command": "sleep" }' "[::1]:8899" delicate.actuator.Actuator/AddTask
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
    // Loads environment variables.
    dotenv().ok();

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
