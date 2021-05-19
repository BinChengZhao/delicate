use super::prelude::*;
use super::schema::task_log;

pub(crate) struct TaskLogQueryBuilder;
impl TaskLogQueryBuilder {
    pub(crate) fn query_all_columns() -> task_log::BoxedQuery<'static, Mysql> {
        task_log::table.into_boxed().select(task_log::all_columns)
    }

    pub(crate) fn query_count() -> task_log::BoxedQuery<'static, Mysql, diesel::sql_types::Bigint> {
        task_log::table.into_boxed().count()
    }
}

impl From<ExecutorEventCollection> for Vec<NewTaskLog> {
    fn from(value: ExecutorEventCollection) -> Self {
        let ExecutorEventCollection { events, .. } = value;
        events.into_iter().map(|e| e.into()).collect()
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ExecutorEventCollection {
    pub(crate) events: Vec<ExecutorEvent>,
    signature: String,
    timestamp: i64,
}

impl ExecutorEventCollection {
    pub(crate) fn verify_signature(&self, _token: &str) -> bool {
        todo!();
    }
}

// TODO:  `delay_timer::utils::status_report::PublicEvent::FinishTask` without task_id and id.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ExecutorEvent {
    task_id: i64,
    id: i64,
    pub(crate) event_type: i16,
    executor_processor_id: i64,
    executor_processor_name: String,
    executor_processor_host: String,
    output: Option<FinishOutput>,
}

impl From<ExecutorEvent> for NewTaskLog {
    fn from(
        ExecutorEvent {
            id,
            task_id,
            event_type,
            executor_processor_id,
            executor_processor_name,
            executor_processor_host,
            ..
        }: ExecutorEvent,
    ) -> Self {
        let state: state::task_log::State = Into::<types::EventType>::into(event_type).into();

        let status = state as i16;

        NewTaskLog {
            task_id,
            id,
            executor_processor_id,
            executor_processor_name,
            executor_processor_host,
            status,
            ..Default::default()
        }
    }
}

impl From<ExecutorEvent> for SupplyTaskLog {
    fn from(
        ExecutorEvent {
            event_type,
            id,
            output,
            ..
        }: ExecutorEvent,
    ) -> Self {
        let mut stdout: String = String::new();
        let mut stderr: String = String::new();
        let mut state: state::task_log::State = Into::<types::EventType>::into(event_type).into();

        if let Some(output) = output {
            match output {
                FinishOutput::ProcessOutput(ChildOutput {
                    child_status,
                    child_stdout,
                    child_stderr,
                }) => {
                    unsafe {
                        stdout = String::from_utf8_unchecked(child_stdout);
                        stderr = String::from_utf8_unchecked(child_stderr);
                    }
                    if child_status != 0 {
                        state = state::task_log::State::AbnormalEnding;
                    }
                }
                FinishOutput::ExceptionOutput(exception_output) => {
                    stderr = exception_output;
                    state = state::task_log::State::AbnormalEnding;
                }
            };
        }

        let status = state as i16;

        SupplyTaskLog {
            id,
            stdout,
            stderr,
            status,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum FinishOutput {
    ProcessOutput(ChildOutput),
    ExceptionOutput(String),
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ChildOutput {
    pub(crate) child_status: i32,
    pub(crate) child_stdout: Vec<u8>,
    pub(crate) child_stderr: Vec<u8>,
}

// TODO: Use `replace_into` instead of `insert_into` to process data.

// delay_timer Add api to support modifying machine node id of `SnowflakeIdGenerator`.
// Then i send executor_processor_id to there.
#[derive(Queryable, Identifiable, AsChangeset, Debug, Clone, Serialize, Deserialize)]
#[table_name = "task_log"]
pub struct TaskLog {
    id: i64,
    task_id: i64,
    name: String,
    description: String,
    command: String,
    frequency: String,
    cron_expression: String,
    maximun_parallel_runable_num: i16,
    tag: String,
    status: i16,
    created_time: NaiveDateTime,
    updated_time: NaiveDateTime,
    executor_processor_id: i64,
    executor_processor_name: String,
    executor_processor_host: String,
    stdout: String,
    stderr: String,
}

#[derive(
    Insertable, Queryable, Identifiable, AsChangeset, Debug, Clone, Serialize, Deserialize, Default,
)]
#[table_name = "task_log"]
pub struct NewTaskLog {
    id: i64,
    pub(crate) task_id: i64,
    name: String,
    description: String,
    command: String,
    frequency: String,
    cron_expression: String,
    maximun_parallel_runable_num: i16,
    tag: String,
    status: i16,
    executor_processor_id: i64,
    executor_processor_name: String,
    executor_processor_host: String,
}

#[derive(Queryable, Identifiable, AsChangeset, Debug, Clone, Serialize, Deserialize)]
#[table_name = "task_log"]
pub struct SupplyTaskLog {
    id: i64,
    status: i16,
    stdout: String,
    stderr: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct QueryParamsTaskLog {
    name: Option<String>,
    description: Option<String>,
    command: Option<String>,
    tag: Option<String>,
    task_id: Option<i64>,
    id: Option<i64>,
    status: Option<i16>,
    executor_processor_id: Option<i64>,
    pub(crate) per_page: i64,
    pub(crate) page: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct PaginateTaskLogs {
    task_logs: Vec<TaskLog>,
    per_page: i64,
    total_page: i64,
}

impl PaginateTaskLogs {
    pub(crate) fn set_task_logs(mut self, task_logs: Vec<TaskLog>) -> Self {
        self.task_logs = task_logs;
        self
    }

    pub(crate) fn set_per_page(mut self, per_page: i64) -> Self {
        self.per_page = per_page;
        self
    }

    pub(crate) fn set_total_page(mut self, total: i64) -> Self {
        self.total_page = (total as f64 / self.per_page as f64).ceil() as i64;
        self
    }
}

impl QueryParamsTaskLog {
    pub(crate) fn query_filter<ST>(
        self,
        mut statement_builder: task_log::BoxedQuery<'static, Mysql, ST>,
    ) -> task_log::BoxedQuery<'static, Mysql, ST> {
        statement_builder = statement_builder.filter(task_log::status.ne(2));
        // Maybe status 2 eq task_log-deleted status.

        if let Some(task_id) = self.task_id {
            statement_builder = statement_builder.filter(task_log::task_id.eq(task_id));
        }

        if let Some(id) = self.id {
            statement_builder = statement_builder.filter(task_log::id.eq(id));
        }

        if let Some(status) = self.status {
            statement_builder = statement_builder.filter(task_log::status.eq(status));
        }

        if let Some(task_name) = self.name {
            statement_builder = statement_builder.filter(task_log::name.like(task_name));
        }

        if let Some(task_description) = self.description {
            statement_builder =
                statement_builder.filter(task_log::description.like(task_description));
        }

        if let Some(task_command) = self.command {
            statement_builder = statement_builder.filter(task_log::command.like(task_command));
        }

        if let Some(task_tag) = self.tag {
            statement_builder = statement_builder.filter(task_log::tag.like(task_tag));
        }

        statement_builder.order(task_log::id.desc())
    }
}
