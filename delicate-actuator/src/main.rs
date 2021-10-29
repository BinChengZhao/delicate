mod prelude;
use prelude::*;

// auth example: /Users/bincheng_paopao/project/repo/rust/others/libs/tonic/examples/src/authentication/server.rs
// zip example: .send_gzip().accept_gzip()
#[derive(Debug)]
pub struct ActuatorSecurityConf {
    pub security_level: SecurityLevel,
    pub rsa_public_key: Option<SecurityeKey<RSAPublicKey>>,
    pub bind_scheduler: BindScheduler,
    pub id_generator: AsyncMutex<SnowflakeIdGenerator>,
}

impl ActuatorSecurityConf {
    pub fn generate_token(&self) -> Option<String> {
        self.security_level.generate_token()
    }

    pub fn get_bind_scheduler(&self) -> &BindScheduler {
        &self.bind_scheduler
    }

    pub fn get_id_generator(&self) -> &AsyncMutex<SnowflakeIdGenerator> {
        &self.id_generator
    }
}

// On async fn health_check(

// TODO:
//     Implement a function to cache a list of machines based on cached.
// When executing a task to an actuator, the most resourceful machine is retrieved from it by group id.

#[derive(Debug, Clone)]
pub struct ActuatorState {
    pub handlers_map: Arc<DashMap<i64, TaskHandlers>>,
    pub security_conf: Arc<ActuatorSecurityConf>,
}

impl ActuatorState {
    pub fn get_security_conf(&self) -> &ActuatorSecurityConf {
        &self.security_conf
    }
}

#[derive(Debug)]
pub struct TaskHandlers {
    running_handler: JoinHandle<Result<String, std::io::Error>>,
    timeout_handler: JoinHandle<()>,
}

impl TaskHandlers {
    pub fn new(
        running_handler: JoinHandle<Result<String, std::io::Error>>,
        timeout_handler: JoinHandle<()>,
    ) -> Self {
        Self {
            running_handler,
            timeout_handler,
        }
    }
}

impl TaskHandlers {
    pub fn cancel_running(&self) {
        self.running_handler.abort();
    }

    pub fn cancel_timeout(&self) {
        self.timeout_handler.abort();
    }
}

impl Default for ActuatorState {
    fn default() -> Self {
        let handlers_map = Arc::new(DashMap::new());

        let security_conf = Arc::new(ActuatorSecurityConf::default());
        Self {
            handlers_map,
            security_conf,
        }
    }
}

#[derive(Debug, Default, Clone)]
struct DelicateActuator {
    state: ActuatorState,
}

impl DelicateActuator {
    pub fn get_state(&self) -> &ActuatorState {
        &self.state
    }
}

#[tonic::async_trait]
impl Actuator for DelicateActuator {
    async fn run_task(
        &self,
        request: Request<Task>,
    ) -> Result<Response<UnifiedResponseMessagesForGrpc>, Status> {
        let task = request.get_ref();

        let mut process_linked_list = parse_and_run::<TokioChild, TokioCommand>(&task.command)
            .await
            .map_err(|e| Status::failed_precondition(e.to_string()))?;

        let child_guard = process_linked_list
            .pop_back()
            .ok_or_else(|| Status::failed_precondition("Have no process executed.".to_string()))?;

        let child = child_guard.take_inner().ok_or_else(|| {
            Status::failed_precondition(" No valid process execution .".to_string())
        })?;

        let mut child_stdout = child
            .stdout
            .ok_or_else(|| Status::failed_precondition(" No valid process stdout .".to_string()))?;

        let running_handler = tokio_spawn(async move {
            let mut output = String::new();
            child_stdout
                .read_to_string(&mut output)
                .await
                .map(|_| output)
        });

        let timeout_handler = tokio_spawn(async {
            todo!();
        });

        let task_handlers = TaskHandlers::new(running_handler, timeout_handler);

        let task_ref = request.get_ref();

        info!("{:?}", task_ref);

        let mut res = UnifiedResponseMessagesForGrpc {
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

    async fn cancel_task(
        &self,
        reqeust: Request<RecordId>,
    ) -> Result<Response<UnifiedResponseMessagesForGrpc>, Status> {
        todo!();
    }

    type KeepRunningTaskStream = Pin<
        Box<
            dyn Stream<Item = Result<UnifiedResponseMessagesForGrpc, Status>>
                + Send
                + Sync
                + 'static,
        >,
    >;
    async fn keep_running_task(
        &self,
        request: Request<Task>,
    ) -> Result<Response<Self::KeepRunningTaskStream>, Status> {
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
                    UnifiedResponseMessagesForGrpc {
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

        Ok(Response::new(
            Box::pin(stream) as Self::KeepRunningTaskStream
        ))
    }

    async fn bind_actuator(
        &self,
        request: Request<BindRequest>,
    ) -> Result<Response<UnifiedResponseMessagesForGrpc>, Status> {
        let bind_request = request.into_inner();
        let mut id_generator_guard = self
            .get_state()
            .get_security_conf()
            .get_id_generator()
            .lock()
            .await;

        let executor_machine_id = bind_request.executor_machine_id as i16;
        let extractor: i16 = 0b00_0001_1111;
        let node_id = executor_machine_id & extractor;
        let machine_id = (executor_machine_id >> 5) & extractor;
        *id_generator_guard = SnowflakeIdGenerator::new(node_id as i32, machine_id as i32);

        let token = self.get_state().get_security_conf().generate_token();
        let bind_scheduler = self.get_state().get_security_conf().get_bind_scheduler();
        bind_scheduler.set_bind(bind_request.into()).await;
        bind_scheduler.set_token(token).await;
        todo!();
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

    let state = ActuatorState::default();
    Server::builder()
        .add_service(ActuatorServer::new(DelicateActuator { state }))
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

impl Default for ActuatorSecurityConf {
    fn default() -> Self {
        let security_level = SecurityLevel::get_app_security_level();
        let rsa_public_key =
            SecurityeKey::<RSAPublicKey>::get_app_rsa_key("DELICATE_SECURITY_PUBLIC_KEY");

        if matches!(security_level, SecurityLevel::Normal if rsa_public_key.is_err()) {
            error!(
                "{}",
                rsa_public_key.as_ref()
                    .err()
                    .map(|e| "Initialization failed because: ".to_owned() + (e.to_string().as_ref()))
                    .unwrap_or_default()
            );
            unreachable!("When the security level is Normal, the initialization `delicate-executor` must contain the secret key (DELICATE_SECURITY_PUBLIC_KEY)");
        }

        let id_generator = AsyncMutex::new(SnowflakeIdGenerator::new(0, 0));
        Self {
            security_level: SecurityLevel::get_app_security_level(),
            rsa_public_key: rsa_public_key.map(SecurityeKey).ok(),
            id_generator,
            ..Default::default()
        }
    }
}
