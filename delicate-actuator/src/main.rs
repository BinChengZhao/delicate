mod prelude;
use prelude::*;

// TODO:
// auth example:
// /Users/bincheng_paopao/project/repo/rust/others/libs/tonic/examples/src/
// authentication/server.rs zip example: .send_gzip().accept_gzip()
#[derive(Debug)]
pub struct ActuatorSecurityConf {
    security_level: SecurityLevel,
    #[allow(dead_code)]
    rsa_public_key: Option<SecurityeKey<RSAPublicKey>>,
    bind_scheduler: BindScheduler,
}

impl ActuatorSecurityConf {
    pub fn generate_token(&self) -> Option<String> {
        self.security_level.generate_token()
    }

    pub fn bind_scheduler(&self) -> &BindScheduler {
        &self.bind_scheduler
    }
}

// On async fn health_check(

// TODO:
// Implement a function to cache a list of machines based on cached.
// When executing a task to an actuator, the most resourceful machine is
// retrieved from it by group id.

#[derive(Debug)]
pub struct ActuatorState {
    id_generator: AsyncMutex<SnowflakeIdGenerator>,
    handlers_map: Arc<DashMap<i64, TaskHandlers>>,
    security_conf: Arc<ActuatorSecurityConf>,
    system_mirror: Arc<SystemMirror>,
}

impl ActuatorState {
    pub fn handlers_map_cloned(&self) -> Arc<DashMap<i64, TaskHandlers>> {
        self.handlers_map.clone()
    }

    pub fn handlers_map(&self) -> &DashMap<i64, TaskHandlers> {
        &self.handlers_map
    }
    pub fn security_conf(&self) -> &ActuatorSecurityConf {
        &self.security_conf
    }

    pub fn system_mirror(&self) -> &SystemMirror {
        &self.system_mirror
    }

    pub async fn generate_id(&self) -> i64 {
        self.id_generator.lock().await.real_time_generate()
    }

    pub async fn set_id_generator(&self, id_generator: SnowflakeIdGenerator) {
        let mut id_generator_guard = self.id_generator.lock().await;

        *id_generator_guard = id_generator;
    }

    pub async fn health_response(&self) -> UnifiedResponseMessagesForGrpc {
        let system_snapshot: Option<health_check::proto_health::SystemSnapshot> =
            Some(self.system_mirror().refresh_all().await.into());

        let bind_request: Option<actuator::BindRequest> =
            Some(self.security_conf().bind_scheduler().get_bind().await.unwrap_or_default().into());

        let status_enum: health_check::proto_health::health_check_response::ServingStatus =
            health_check::ServingStatus::Serving.into();
        let status = status_enum as i32;
        let value = HealthCheckResponse { system_snapshot, bind_request, status }.encode_to_vec();

        let type_url = "/delicate.actuator.health.HealthCheckResponse".to_string();
        let any = Any { type_url, value };

        let mut health_response = UnifiedResponseMessagesForGrpc::success();
        health_response.data.push(any);
        health_response
    }
}

#[derive(Debug)]
pub struct TaskHandlers {
    id: i64,
    status: AtomicUsize,
    running_handler: JoinHandle<String>,
    timeout_handler: JoinHandle<()>,
}

impl TaskHandlers {
    pub const INIT: usize = 1;
    pub const COMPLETE: usize = 1;
    pub const TIMEOUT: usize = 1 << 1;

    pub fn new(id: i64,
               running_handler: JoinHandle<String>,
               timeout_handler: JoinHandle<()>)
               -> Self {
        let status = AtomicUsize::new(0);
        Self { id, status, running_handler, timeout_handler }
    }

    pub fn complete(self) {
        if self.status
               .compare_exchange(Self::INIT, Self::COMPLETE, Ordering::AcqRel, Ordering::Acquire)
               .is_ok()
        {
            debug!("cancel running task: {}", self.id);
            self.timeout_handler.abort();
        }
    }

    pub fn timeout(self) {
        if self.status
               .compare_exchange(Self::INIT, Self::TIMEOUT, Ordering::AcqRel, Ordering::Acquire)
               .is_ok()
        {
            debug!("cancel timeout task: {}", self.id);
            self.running_handler.abort();
        }
    }

    pub fn cancel(self) {
        self.timeout_handler.abort();
        self.running_handler.abort();
    }
}

impl Default for ActuatorState {
    fn default() -> Self {
        let handlers_map = Arc::new(DashMap::new());

        let security_conf = Arc::new(ActuatorSecurityConf::default());
        let id_generator = AsyncMutex::new(SnowflakeIdGenerator::new(0, 0));

        let system_mirror = Arc::new(SystemMirror::default());
        Self { handlers_map, security_conf, id_generator, system_mirror }
    }
}

#[derive(Debug, Default)]
struct DelicateActuator {
    state: Arc<ActuatorState>,
}

impl DelicateActuator {
    pub fn state(&self) -> &ActuatorState {
        &self.state
    }

    pub async fn handle_task(&self, task: Task) -> Result<RecordId, Status> {
        let command = &task.command;
        let timeout = task.timeout;

        // Parse the command line and generate a process linked-list.
        let mut process_linked_list = parse_and_run::<TokioChild, TokioCommand>(command)
            .await
            .map_err(|e| Status::failed_precondition(e.to_string()))?;

        // Take the last process that needs to wait for output and get the output.
        let child_guard = process_linked_list
            .pop_back()
            .ok_or_else(|| Status::failed_precondition("Have no process executed.".to_string()))?;

        let child = child_guard.take_inner().ok_or_else(|| {
            Status::failed_precondition(" No valid process execution .".to_string())
        })?;

        let mut child_stdout =
            child.stdout.ok_or_else(|| {
                             Status::failed_precondition(" No valid process stdout .".to_string())
                         })?;

        let id = self.state().generate_id().await;
        let handlers_map_cloned = self.state().handlers_map_cloned();

        let running_handler = tokio_spawn({
            let handlers_map_cloned = handlers_map_cloned.clone();
            async move {
                let mut output = String::new();
                let result_output = child_stdout.read_to_string(&mut output).await.map(|_| output);

                if let Some((_, h)) = handlers_map_cloned.remove(&id) {
                    debug!("Task: {}, completed", id);
                    h.complete();
                }

                let output = result_output.unwrap_or_else(|e| e.to_string());
                // TODO: There should be callback to scheduler (/api/task_log/event_trigger).
                debug!("Task: {}, Output: {}", id, &output);
                output
            }
        });

        let timeout_handler = tokio_spawn(async move {
            async_sleep(Duration::from_secs(timeout)).await;
            if let Some((_, h)) = handlers_map_cloned.remove(&id) {
                debug!("Task: {}, Timeout", id);
                h.timeout();
            }
        });

        let task_handlers = TaskHandlers::new(id, running_handler, timeout_handler);

        self.state().handlers_map().insert(id, task_handlers);
        Ok(RecordId { id })
    }
}

#[tonic::async_trait]
impl Actuator for DelicateActuator {
    async fn run_task(&self, request: Request<Task>) -> Result<Response<RecordId>, Status> {
        let _recorder = metrics::AutoRecorder::new("run_task");

        let task = request.into_inner();
        debug!("task: {:?}", &task);

        let record_id = self.handle_task(task).await?;
        Ok(Response::new(record_id))
    }

    async fn cancel_task(&self,
                         reqeust: Request<RecordId>)
                         -> Result<Response<UnifiedResponseMessagesForGrpc>, Status> {
        let _recorder = metrics::AutoRecorder::new("cancel_task");

        let record_id = reqeust.into_inner();
        let (_, task_handlers) =
            self.state().handlers_map().remove(&record_id.id).ok_or_else(|| {
                                                                  Status::unavailable(format!(
                    "There have no running task-record: {} instance to cancel.",
                    record_id.id
                ))
                                                              })?;
        task_handlers.cancel();

        Ok(Response::new(UnifiedResponseMessagesForGrpc::success()))
    }

    type KeepRunningTaskStream =
        Pin<Box<dyn Stream<Item = Result<UnifiedResponseMessagesForGrpc, Status>>
                    + Send
                    + Sync
                    + 'static>>;
    async fn keep_running_task(&self,
                               request: Request<Task>)
                               -> Result<Response<Self::KeepRunningTaskStream>, Status> {
        let _recorder = metrics::AutoRecorder::new("keep_running_task");

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

        let child_stdout =
            child.stdout.ok_or_else(|| {
                             Status::failed_precondition(" No valid process stdout .".to_string())
                         })?;

        let buf_reader_lines = LinesStream::new(BufReader::new(child_stdout).lines()).map(|l| {
                                                                                         l.map(|s| {
                let type_url = "/String".to_string();
                let value = s.encode_to_vec();
                let any = Any { type_url, value };
                let data = vec![any];
                UnifiedResponseMessagesForGrpc { data, ..Default::default() }
            })
            .map_err(|e| Status::unknown(e.to_string()))
                                                                                     });

        Ok(Response::new(Box::pin(buf_reader_lines) as Self::KeepRunningTaskStream))
    }

    async fn bind_actuator(&self,
                           request: Request<BindRequest>)
                           -> Result<Response<UnifiedResponseMessagesForGrpc>, Status> {
        let _recorder = metrics::AutoRecorder::new("bind_actuator");

        let bind_request = request.into_inner();

        let executor_machine_id = bind_request.executor_machine_id as i16;
        let extractor: i16 = 0b00_0001_1111;
        let node_id = executor_machine_id & extractor;
        let machine_id = (executor_machine_id >> 5) & extractor;
        self.state()
            .set_id_generator(SnowflakeIdGenerator::new(node_id as i32, machine_id as i32))
            .await;

        let token = self.state().security_conf().generate_token();
        let bind_scheduler = self.state().security_conf().bind_scheduler();
        bind_scheduler.set_bind(bind_request.into()).await;
        bind_scheduler.set_token(token).await;
        Ok(Response::new(UnifiedResponseMessagesForGrpc::success()))
    }

    async fn health_check(&self,
                          request: Request<HealthCheckUnit>)
                          -> Result<Response<UnifiedResponseMessagesForGrpc>, Status> {
        let _recorder = metrics::AutoRecorder::new("health_check");

        let addr = request.remote_addr();
        let check_unit = request.into_inner();
        info!("Health-check From: {:?}, unit: {:?}", addr, check_unit);
        Ok(Response::new(self.state().health_response().await))
    }

    type HealthWatchStream = Pin<Box<dyn Stream<Item = Result<UnifiedResponseMessagesForGrpc,
                                                              Status>>
                                         + Send
                                         + Sync
                                         + 'static>>;
    async fn health_watch(&self,
                          request: Request<HealthWatchUnit>)
                          -> Result<Response<Self::HealthWatchStream>, Status> {
        let _recorder = metrics::AutoRecorder::new("health_watch");

        let addr = request.remote_addr();
        let check_unit = request.into_inner();
        info!("Health-check From: {:?}, unit: {:?}", addr, check_unit);
        let state = self.state.clone();
        let stream = async_stream::stream! {

         loop{
             yield Ok(state.health_response().await);
         }
        };

        Ok(Response::new(Box::pin(stream)))
    }
}

// ./grpcurl -plaintext -import-path ./delicate/delicate-utils/proto -proto
// actuator.proto -d '{"id":1, "name": "Tonic", "command": "sleep" }'
// "[::1]:8899" delicate.actuator.Actuator/AddTask TODO:
// Objectives.

// 2. Implement the actuator, via tonic.
// 3. Prioritize minimalist implementations, supporting only single machines at
// first, with subsequent support for slicing or various rules. 4. advertise on
// `poem` Readme. 5. add/delete/change/check/cancel for standalone tasks Do it
// first.

// scheduler & actuator interaction, also send/register task,
// return response carrying record-id,
// actuator replies to events after execution is completed.

// The actuator maintains the state of the task internally,
// kills the task directly if it times out, and returns the timeout event.

// The actuator supports cancel of tasks, using task.abort();

// Provide a task test execution, direct transfer between frontend & scheduler &
// actuator, dynamic rendering of pages, no database storage. SSE: (frontend &
// scheduler) GRPC-stream: (scheduler & actuator)
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Loads environment variables.
    dotenv().ok();

    init_logger();

    // TODO:
    // auth example:
    // /Users/bincheng_paopao/project/repo/rust/others/libs/tonic/examples/src/
    // authentication/ server.rs zip example: .send_gzip().accept_gzip()

    Server::builder().add_service(ActuatorServer::new(DelicateActuator::default()))
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
        let security_level = SecurityLevel::app_security_level();
        let rsa_public_key =
            SecurityeKey::<RSAPublicKey>::app_rsa_key("DELICATE_SECURITY_PUBLIC_KEY");

        if matches!(security_level, SecurityLevel::Normal if rsa_public_key.is_err()) {
            error!("{}",
                   rsa_public_key.as_ref()
                                 .err()
                                 .map(|e| "Initialization failed because: ".to_owned()
                                          + (e.to_string().as_ref()))
                                 .unwrap_or_default());
            unreachable!("When the security level is Normal, the initialization `delicate-executor` must contain the secret key (DELICATE_SECURITY_PUBLIC_KEY)");
        }

        let bind_scheduler = BindScheduler::default();
        Self { security_level: SecurityLevel::app_security_level(),
               rsa_public_key: rsa_public_key.map(SecurityeKey).ok(),
               bind_scheduler }
    }
}
