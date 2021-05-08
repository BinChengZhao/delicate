use super::prelude::*;
use super::schema::{task, task_log};
use super::PoolMysqlConnection;
use diesel::result::Error as DieselError;

#[derive(Queryable, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Task {
    id: i64,
    name: String,
    description: String,
    command: String,
    frequency: String,
    cron_expression: String,
    timeout: i16,
    retry_times: i16,
    retry_interval: i16,
    maximun_parallel_runable_num: i16,
    tag: String,
    status: i16,
}

#[derive(Insertable, Identifiable, AsChangeset, Debug, Default, Serialize, Deserialize)]
#[table_name = "task"]
pub struct NewTask {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) command: String,
    pub(crate) frequency: String,
    pub(crate) cron_expression: String,
    pub(crate) timeout: i16,
    pub(crate) retry_times: i16,
    pub(crate) retry_interval: i16,
    pub(crate) maximun_parallel_runable_num: i16,
    pub(crate) tag: String,
    pub(crate) status: i16,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct QueryParamsTask {
    id: Option<i64>,
    name: Option<String>,
    description: Option<String>,
    command: Option<String>,
    frequency: Option<String>,
    cron_expression: Option<String>,
    tag: Option<String>,
    status: Option<i16>,
}

// TODO: Constructing query fragments.
impl QueryParamsTask {
    pub(crate) fn query_all_columns(self) -> task::BoxedQuery<'static, Mysql>{
        task::table
        .into_boxed()
        .select(task::all_columns)
    }

    pub(crate) fn query_count(self)-> task::BoxedQuery<'static, Mysql, diesel::sql_types::Bigint>{
        task::table
        .into_boxed()
        .count().filter(task::status.ne(2))
    }

    pub(crate) fn query(self) -> task::BoxedQuery<'static, Mysql> {
        let mut statement_builder = task::table
            .into_boxed()
            .select(task::all_columns)
            .filter(task::status.ne(2));
            // Maybe status 2 eq task-deleted status.

        if let Some(task_id) = self.id {
            statement_builder = statement_builder.filter(task::id.eq(task_id));
        }

        if let Some(task_status) = self.status {
            statement_builder = statement_builder.filter(task::status.eq(task_status));
        } else {
            statement_builder = statement_builder.filter(task::status.ne(2));
        }

        if let Some(task_name) = self.name {
            statement_builder = statement_builder.filter(task::name.eq(task_name));
        }

        if let Some(task_description) = self.description {
            statement_builder = statement_builder.filter(task::description.eq(task_description));
        }

        if let Some(task_command) = self.command {
            statement_builder = statement_builder.filter(task::command.eq(task_command));
        }

        if let Some(task_frequency) = self.frequency {
            statement_builder = statement_builder.filter(task::frequency.eq(task_frequency));
        }

        if let Some(task_cron_expression) = self.cron_expression {
            statement_builder =
                statement_builder.filter(task::cron_expression.eq(task_cron_expression));
        }

        if let Some(task_tag) = self.tag {
            statement_builder = statement_builder.filter(task::tag.eq(task_tag));
        }

        statement_builder.order(task::id.desc())

    }
}

#[derive(Queryable, Identifiable, AsChangeset, Debug, Clone, Serialize, Deserialize)]
#[table_name = "task_log"]
pub struct NaiveTaskLog {
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
}

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[table_name = "task_log"]
pub struct NaiveNewTaskLog {
    task_id: i64,
    record_id: i64,
    name: String,
    description: String,
    command: String,
    frequency: String,
    cron_expression: String,
    maximun_parallel_runable_num: i16,
    tag: String,
    status: i16,
    created_time: NaiveDateTime,
    executor_processor_id: i64,
    executor_processor_name: String,
    executor_processor_host: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
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
    created_time: MysqlTimestamp,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct NewTaskLog {
    task_id: i64,
    record_id: i64,
    name: String,
    description: String,
    command: String,
    frequency: String,
    cron_expression: String,
    maximun_parallel_runable_num: i16,
    tag: String,
    status: i16,
    created_time: MysqlTimestamp,
    executor_processor_id: i64,
    executor_processor_name: String,
    executor_processor_host: i64,
}

impl From<NaiveNewTaskLog> for NewTaskLog {
    fn from(value: NaiveNewTaskLog) -> NewTaskLog {
        let created_time = MysqlTimestamp(value.created_time);
        NewTaskLog {
            task_id: value.task_id,
            record_id: value.record_id,
            name: value.name,
            description: value.description,
            command: value.command,
            frequency: value.frequency,
            cron_expression: value.cron_expression,
            maximun_parallel_runable_num: value.maximun_parallel_runable_num,
            tag: value.tag,
            status: value.status,
            executor_processor_id: value.executor_processor_id,
            executor_processor_name: value.executor_processor_name,
            executor_processor_host: value.executor_processor_host,
            created_time,
        }
    }
}

impl From<NaiveTaskLog> for TaskLog {
    fn from(value: NaiveTaskLog) -> TaskLog {
        let created_time = MysqlTimestamp(value.created_time);
        TaskLog {
            id: value.id,
            task_id: value.task_id,
            name: value.name,
            description: value.description,
            command: value.command,
            frequency: value.frequency,
            cron_expression: value.cron_expression,
            maximun_parallel_runable_num: value.maximun_parallel_runable_num,
            tag: value.tag,
            status: value.status,
            created_time,
        }
    }
}


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct QueryParamsTaskLog {
    name: Option<String>,
    description: Option<String>,
    command: Option<String>,
    tag: Option<String>,
    task_id: Option<i64>,
    record_id: Option<i64>,
    status: Option<i16>,
    executor_processor_id: Option<i64>,
}


#[derive(PartialEq, Debug, Eq, PartialOrd, Ord, Copy, Clone, Serialize, Deserialize)]
pub struct MysqlTimestamp(NaiveDateTime);

impl Default for MysqlTimestamp {
    fn default() -> Self {
        MysqlTimestamp(NaiveDateTime::new(
            NaiveDate::from_ymd(0, 0, 0),
            NaiveTime::from_hms(0, 0, 0),
        ))
    }
}
