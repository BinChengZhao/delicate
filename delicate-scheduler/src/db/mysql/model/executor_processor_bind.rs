use super::prelude::*;
use super::schema::{executor_processor_bind};

#[derive(Queryable, AsChangeset, Identifiable, Debug, Clone, Serialize, Deserialize)]
#[table_name = "executor_processor_bind"]

pub struct ExecutorProcessorBind {
    id: i64,
    name: String,
    group_id: i64,
    executor_id: i64,
    weight: i16,
    status: i16,
    created_time: NaiveDateTime,
    deleted_time: Option<NaiveDateTime>,
}

#[derive(Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "executor_processor_bind"]
pub struct NewExecutorProcessorBind {
    name: String,
    group_id: i64,
    executor_id: i64,
    weight: i16,
    status: i16,
    created_time: NaiveDateTime,
    deleted_time: NaiveDateTime,
}


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct QueryParamsExecutorProcessorBind {
    id: Option<i64>,
    group_id: Option<i64>,
    executor_id: Option<i64>,
    name: Option<String>,
    status: Option<i16>,
    pub(crate) per_page: i64,
    pub(crate) page: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct PaginateExecutorProcessorBind {
    executor_processor_binds: Vec<ExecutorProcessorBind>,
    per_page: i64,
    total_page: i64,
}

impl PaginateExecutorProcessorBind {
    pub(crate) fn set_tasks(mut self, executor_processor_binds: Vec<ExecutorProcessorBind>) -> Self {
        self.executor_processor_binds = executor_processor_binds;
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

pub(crate) struct ExecutorProcessorBindQueryBuilder;
impl ExecutorProcessorBindQueryBuilder {
    pub(crate) fn query_all_columns() -> executor_processor_bind::BoxedQuery<'static, Mysql> {
        executor_processor_bind::table
            .into_boxed()
            .select(executor_processor_bind::all_columns)
    }

    pub(crate) fn query_count(
    ) -> executor_processor_bind::BoxedQuery<'static, Mysql, diesel::sql_types::Bigint> {
        executor_processor_bind::table.into_boxed().count()
    }
}

impl QueryParamsExecutorProcessorBind {
    pub(crate) fn query_filter<ST>(
        self,
        mut statement_builder: executor_processor_bind::BoxedQuery<'static, Mysql, ST>,
    ) -> executor_processor_bind::BoxedQuery<'static, Mysql, ST> {
        statement_builder = statement_builder.filter(executor_processor_bind::status.ne(2));
        // Maybe status 2 eq task-deleted status.

        if let Some(executor_processor_bind_id) = self.id {
            statement_builder =
                statement_builder.filter(executor_processor_bind::id.eq(executor_processor_bind_id));
        }

        if let Some(executor_processor_bind_group_id) = self.group_id {
            statement_builder =
                statement_builder.filter(executor_processor_bind::group_id.eq(executor_processor_bind_group_id));
        }

        if let Some(executor_processor_bind_executor_id) = self.executor_id {
            statement_builder =
                statement_builder.filter(executor_processor_bind::executor_id.eq(executor_processor_bind_executor_id));
        }
        
        if let Some(executor_processor_bind_name) = self.name {
            statement_builder =
                statement_builder.filter(executor_processor_bind::name.like(executor_processor_bind_name));
        }
        if let Some(task_status) = self.status {
            statement_builder =
                statement_builder.filter(executor_processor_bind::status.eq(task_status));
        }

        statement_builder.order(executor_processor_bind::id.desc())
    }
}
