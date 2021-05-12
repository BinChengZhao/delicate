use super::prelude::*;
use super::schema::{task_log, task_log_extend};

pub(crate) struct TaskLogQueryBuilder;
impl TaskLogQueryBuilder {
    pub(crate) fn query_all_columns() -> task_log::BoxedQuery<'static, Mysql> {
        task_log::table.into_boxed().select(task_log::all_columns)
    }

    pub(crate) fn query_count() -> task_log::BoxedQuery<'static, Mysql, diesel::sql_types::Bigint> {
        task_log::table.into_boxed().count()
    }
}

#[derive(
    Insertable, Queryable, Identifiable, AsChangeset, Debug, Clone, Serialize, Deserialize,
)]
#[table_name = "task_log_extend"]
pub struct TaskLogExtend {
    id: i64,
    stdout: String,
    stderr: String,
}


#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[table_name = "task_log"]
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
    created_time: NaiveDateTime,
    executor_processor_id: i64,
    executor_processor_name: String,
    executor_processor_host: i64,
}

#[derive(Queryable, Identifiable, AsChangeset, Debug, Clone, Serialize, Deserialize)]
#[table_name = "task_log"]
pub struct TaskLog {
    id: i64,
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
pub struct QueryParamsTaskLog {
    name: Option<String>,
    description: Option<String>,
    command: Option<String>,
    tag: Option<String>,
    task_id: Option<i64>,
    record_id: Option<i64>,
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

        if let Some(record_id) = self.record_id {
            statement_builder = statement_builder.filter(task_log::record_id.eq(record_id));
        }

        if let Some(status) = self.status {
            statement_builder = statement_builder.filter(task_log::status.eq(status));
        } else {
            //TODO: Addtion state in future.
            statement_builder = statement_builder.filter(task_log::status.ne(2));
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
