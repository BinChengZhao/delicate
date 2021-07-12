use super::prelude::*;
use super::schema::{task_log, task_log_extend};
use delicate_utils_task_log::{ChildOutput, ExecutorEvent, ExecutorEventCollection, FinishOutput};

pub(crate) struct TaskLogQueryBuilder;
impl TaskLogQueryBuilder {
    pub(crate) fn query_all_columns() -> task_log::BoxedQuery<'static, Mysql> {
        task_log::table.into_boxed().select(task_log::all_columns)
    }

    pub(crate) fn query_count() -> task_log::BoxedQuery<'static, Mysql, diesel::sql_types::Bigint> {
        task_log::table.into_boxed().count()
    }
}

pub struct NewTaskLogs(pub Vec<NewTaskLog>);

impl From<ExecutorEventCollection> for NewTaskLogs {
    fn from(value: ExecutorEventCollection) -> Self {
        let ExecutorEventCollection { events, .. } = value;
        NewTaskLogs(events.into_iter().map(|e| e.into()).collect())
    }
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
        let state: state::task_log::State =
            Into::<delicate_utils_task_log::EventType>::into(event_type).into();

        let status = state as i16;

        NewTaskLog {
            id,
            task_id,
            status,
            executor_processor_id,
            executor_processor_name,
            executor_processor_host,
            ..Default::default()
        }
    }
}

pub struct SupplyTaskLogTuple(pub SupplyTaskLog, pub SupplyTaskLogExtend);
impl From<ExecutorEvent> for SupplyTaskLogTuple {
    fn from(
        ExecutorEvent {
            event_type,
            id,
            task_id,
            output,
            ..
        }: ExecutorEvent,
    ) -> Self {
        let mut stdout: String = String::new();
        let mut stderr: String = String::new();
        let mut state: state::task_log::State =
            Into::<delicate_utils_task_log::EventType>::into(event_type).into();

        if let Some(output) = output {
            match output {
                FinishOutput::ProcessOutput(ChildOutput {
                    child_status,
                    child_stdout,
                    child_stderr,
                }) => {
                    stdout = child_stdout;
                    stderr = child_stderr;

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

        SupplyTaskLogTuple(
            SupplyTaskLog { id, status },
            SupplyTaskLogExtend {
                id,
                task_id,
                stdout,
                stderr,
            },
        )
    }
}

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
    maximun_parallel_runnable_num: i16,
    tag: String,
    status: i16,
    created_time: NaiveDateTime,
    updated_time: NaiveDateTime,
    executor_processor_id: i64,
    executor_processor_name: String,
    executor_processor_host: String,
}

#[derive(
    Insertable, Queryable, Identifiable, AsChangeset, Debug, Default, Clone, Serialize, Deserialize,
)]
#[table_name = "task_log_extend"]
pub struct TaskLogExtend {
    id: i64,
    task_id: i64,
    stdout: String,
    stderr: String,
}

#[derive(
    Insertable, Queryable, Identifiable, AsChangeset, Debug, Clone, Serialize, Deserialize, Default,
)]
#[table_name = "task_log"]
pub struct NewTaskLog {
    pub(crate) id: i64,
    pub(crate) task_id: i64,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) command: String,
    pub(crate) frequency: String,
    pub(crate) cron_expression: String,
    pub(crate) maximun_parallel_runnable_num: i16,
    pub(crate) tag: String,
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
}

#[derive(
    Insertable, Queryable, Identifiable, AsChangeset, Debug, Clone, Serialize, Deserialize,
)]
#[table_name = "task_log_extend"]
pub struct SupplyTaskLogExtend {
    id: i64,
    task_id: i64,
    stdout: String,
    stderr: String,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]

pub struct RecordId {
    pub(crate) record_id: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct QueryParamsTaskLog {
    id: Option<i64>,
    task_id: Option<i64>,
    name: Option<String>,
    description: Option<String>,
    command: Option<String>,
    tag: Option<String>,
    status: Option<i16>,
    executor_processor_id: Option<i64>,
    pub(crate) start_time: Option<i64>,
    pub(crate) end_time: Option<i64>,
    pub(crate) per_page: i64,
    pub(crate) page: i64,
}

#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize)]

pub struct TaskRecord {
    pub(crate) task_id: i64,
    pub(crate) record_id: i64,
    pub(crate) executor_processor_id: i64,
}

impl QueryParamsTaskLog {
    pub(crate) fn query_filter<ST>(
        self,
        mut statement_builder: task_log::BoxedQuery<'static, Mysql, ST>,
    ) -> task_log::BoxedQuery<'static, Mysql, ST> {

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

        if let Some(start_time) = self.start_time {
            let end_time = self.end_time.unwrap_or_else(|| start_time + 86400 * 3);

            let start_time = NaiveDateTime::from_timestamp(start_time, 0);
            let end_time = NaiveDateTime::from_timestamp(end_time, 0);

            statement_builder =
                statement_builder.filter(task_log::created_time.between(start_time, end_time));
        }

        statement_builder.order(task_log::id.desc())
    }
}
