use super::prelude::*;
use super::schema::executor_group;

#[derive(Queryable, Identifiable, AsChangeset, Debug, Clone, Serialize, Deserialize)]
#[table_name = "executor_group"]

pub struct ExecutorGroup {
    id: i64,
    name: String,
    description: String,
    tag: String,
    status: i16,
    created_time: NaiveDateTime,
    deleted_time: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug, Default, Serialize, Deserialize)]
#[table_name = "executor_group"]
pub struct NewExecutorGroup {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) tag: String,
    pub(crate) status: i16,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct QueryParamsExecutorGroup {
    id: Option<i64>,
    name: Option<String>,
    description: Option<String>,
    command: Option<String>,
    frequency: Option<String>,
    cron_expression: Option<String>,
    tag: Option<String>,
    status: Option<i16>,
    pub(crate) per_page: i64,
    pub(crate) page: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct PaginateExecutorGroup {
    executor_groups: Vec<ExecutorGroup>,
    per_page: i64,
    total_page: i64,
}

impl PaginateExecutorGroup {
    pub(crate) fn set_tasks(mut self, executor_groups: Vec<ExecutorGroup>) -> Self {
        self.executor_groups = executor_groups;
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

pub(crate) struct ExecutorGroupQueryBuilder;
impl ExecutorGroupQueryBuilder {
    pub(crate) fn query_all_columns() -> executor_group::BoxedQuery<'static, Mysql> {
        executor_group::table
            .into_boxed()
            .select(executor_group::all_columns)
    }

    pub(crate) fn query_count(
    ) -> executor_group::BoxedQuery<'static, Mysql, diesel::sql_types::Bigint> {
        executor_group::table.into_boxed().count()
    }
}

impl QueryParamsExecutorGroup {
    pub(crate) fn query_filter<ST>(
        self,
        mut statement_builder: executor_group::BoxedQuery<'static, Mysql, ST>,
    ) -> executor_group::BoxedQuery<'static, Mysql, ST> {
        statement_builder = statement_builder.filter(executor_group::status.ne(2));
        // Maybe status 2 eq task-deleted status.

        if let Some(executor_group_id) = self.id {
            statement_builder = statement_builder.filter(executor_group::id.eq(executor_group_id));
        }

        if let Some(task_status) = self.status {
            statement_builder = statement_builder.filter(executor_group::status.eq(task_status));
        } else {
            statement_builder = statement_builder
                .filter(executor_group::status.ne(state::executor_group::State::Forbidden as i16));
        }

        if let Some(executor_group_name) = self.name {
            statement_builder =
                statement_builder.filter(executor_group::name.like(executor_group_name));
        }

        if let Some(executor_group_description) = self.description {
            statement_builder = statement_builder
                .filter(executor_group::description.like(executor_group_description));
        }

        if let Some(executor_group_tag) = self.tag {
            statement_builder =
                statement_builder.filter(executor_group::tag.like(executor_group_tag));
        }

        statement_builder.order(executor_group::id.desc())
    }
}
