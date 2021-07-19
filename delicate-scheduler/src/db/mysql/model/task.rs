use super::prelude::*;
use super::schema::task;
use diesel::sql_types::{Bigint, SmallInt, VarChar};

#[derive(
    Queryable, Insertable, Clone, Identifiable, AsChangeset, Debug, Serialize, Deserialize,
)]
#[table_name = "task"]

pub struct Task {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) command: String,
    pub(crate) frequency: String,
    pub(crate) cron_expression: String,
    pub(crate) timeout: i16,
    pub(crate) retry_times: i16,
    pub(crate) retry_interval: i16,
    pub(crate) maximum_parallel_runnable_num: i16,
    pub(crate) tag: String,
    pub(crate) status: i16,
    pub(crate) created_time: NaiveDateTime,
    pub(crate) deleted_time: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug, Default, Serialize, Deserialize)]
#[table_name = "task"]
pub struct NewTask {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) command: String,
    pub(crate) frequency: String,
    pub(crate) cron_expression: String,
    pub(crate) timeout: i16,
    pub(crate) retry_times: i16,
    pub(crate) retry_interval: i16,
    pub(crate) maximum_parallel_runnable_num: i16,
    pub(crate) tag: String,
}

#[derive(
    Queryable, Insertable, Clone, Identifiable, AsChangeset, Debug, Serialize, Deserialize,
)]
#[table_name = "task"]

pub struct UpdateTask {
    pub(crate) id: i64,
    name: String,
    description: String,
    command: String,
    frequency: String,
    cron_expression: String,
    timeout: i16,
    retry_times: i16,
    retry_interval: i16,
    maximum_parallel_runnable_num: i16,
    tag: String,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]

pub struct TaskId {
    pub(crate) task_id: i64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NewTaskBody {
    pub(crate) task: NewTask,
    pub(crate) binding_ids: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTaskBody {
    pub(crate) task: UpdateTask,
    pub(crate) binding_ids: Vec<i64>,
}

#[derive(Queryable, Identifiable, AsChangeset, Debug, Default, Serialize, Deserialize)]
#[table_name = "task"]
pub struct SupplyTask {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) command: String,
    pub(crate) frequency: String,
    pub(crate) cron_expression: String,
    pub(crate) tag: String,
    pub(crate) maximum_parallel_runnable_num: i16,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct QueryParamsTask {
    id: Option<i64>,
    name: Option<String>,
    description: Option<String>,
    command: Option<String>,
    cron_expression: Option<String>,
    tag: Option<String>,
    status: Option<i16>,
    pub(crate) per_page: i64,
    pub(crate) page: i64,
}

type SupplyTaskType = (
    Bigint,
    VarChar,
    VarChar,
    VarChar,
    VarChar,
    VarChar,
    VarChar,
    SmallInt,
);
pub(crate) struct TaskQueryBuilder;
impl TaskQueryBuilder {
    pub(crate) fn query_all_columns() -> task::BoxedQuery<'static, Mysql> {
        task::table.into_boxed().select(task::all_columns)
    }

    pub(crate) fn query_supply_task_log() -> task::BoxedQuery<'static, Mysql, SupplyTaskType> {
        task::table.into_boxed().select((
            task::id,
            task::name,
            task::description,
            task::command,
            task::frequency,
            task::cron_expression,
            task::tag,
            task::maximum_parallel_runnable_num,
        ))
    }

    pub(crate) fn query_count() -> task::BoxedQuery<'static, Mysql, Bigint> {
        task::table.into_boxed().count()
    }
}

impl QueryParamsTask {
    pub(crate) fn query_filter<ST>(
        self,
        mut statement_builder: task::BoxedQuery<'static, Mysql, ST>,
    ) -> task::BoxedQuery<'static, Mysql, ST> {
        if let Some(task_id) = self.id {
            statement_builder = statement_builder.filter(task::id.eq(task_id));
        }

        if let Some(task_status) = self.status {
            statement_builder = statement_builder.filter(task::status.eq(task_status));
        } else {
            statement_builder =
                statement_builder.filter(task::status.ne(state::task::State::Deleted as i16));
        }

        if let Some(task_name) = self.name {
            statement_builder = statement_builder.filter(task::name.like(task_name));
        }

        if let Some(task_description) = self.description {
            statement_builder = statement_builder.filter(task::description.like(task_description));
        }

        if let Some(task_command) = self.command {
            statement_builder = statement_builder.filter(task::command.like(task_command));
        }

        if let Some(task_cron_expression) = self.cron_expression {
            statement_builder =
                statement_builder.filter(task::cron_expression.like(task_cron_expression));
        }

        if let Some(task_tag) = self.tag {
            statement_builder = statement_builder.filter(task::tag.like(task_tag));
        }

        statement_builder.order(task::id.desc())
    }
}
