use super::prelude::*;
use super::schema::executor_group;

#[derive(Queryable, Identifiable, AsChangeset, Debug, Clone, Serialize, Deserialize)]
#[table_name = "executor_group"]

pub struct ExecutorGroup {
    id: i64,
    name: String,
    description: String,
    tag: String,
    created_time: NaiveDateTime,
    deleted_time: Option<NaiveDateTime>,
}

#[derive(Queryable, Identifiable, AsChangeset, Debug, Clone, Serialize, Deserialize)]
#[table_name = "executor_group"]

pub struct UpdateExecutorGroup {
    id: i64,
    name: String,
    description: String,
    tag: String,
}

#[derive(Insertable, Debug, Default, Serialize, Deserialize)]
#[table_name = "executor_group"]
pub struct NewExecutorGroup {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) tag: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct QueryParamsExecutorGroup {
    id: Option<i64>,
    name: Option<String>,
    description: Option<String>,
    tag: Option<String>,
    pub(crate) per_page: i64,
    pub(crate) page: i64,
}


#[derive(Copy, Clone, Debug, Serialize, Deserialize)]

pub struct ExecutorGroupId {
    pub(crate) executor_group_id: i64,
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
        if let Some(executor_group_id) = self.id {
            statement_builder = statement_builder.filter(executor_group::id.eq(executor_group_id));
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
