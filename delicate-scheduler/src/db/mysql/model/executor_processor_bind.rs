use super::prelude::*;
use super::schema::executor_processor_bind;

#[derive(Queryable, AsChangeset, Identifiable, Debug, Clone, Serialize, Deserialize)]
#[table_name = "executor_processor_bind"]

pub struct ExecutorProcessorBind {
    pub(crate) id: i64,
    name: String,
    group_id: i64,
    executor_id: i64,
    weight: i16,
    created_time: NaiveDateTime,
}

#[derive(Queryable, AsChangeset, Identifiable, Debug, Clone, Serialize, Deserialize)]
#[table_name = "executor_processor_bind"]

pub struct UpdateExecutorProcessorBind {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) group_id: i64,
    pub(crate) executor_id: i64,
    pub(crate) weight: i16,
}

#[derive(Queryable, AsChangeset, Identifiable, Debug, Clone, Serialize, Deserialize)]
#[table_name = "executor_processor_bind"]

pub struct BindingSelection {
    id: i64,
    #[serde(rename(serialize = "title"))]
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewExecutorProcessorBinds {
    pub(crate) name: String,
    pub(crate) group_id: i64,
    pub(crate) executor_ids: Vec<i64>,
    pub(crate) weight: i16,
}

#[derive(Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "executor_processor_bind"]
pub struct NewExecutorProcessorBind {
    pub(crate) name: String,
    pub(crate) group_id: i64,
    pub(crate) executor_id: i64,
    pub(crate) weight: i16,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct QueryParamsExecutorProcessorBind {
    id: Option<i64>,
    group_id: Option<i64>,
    executor_id: Option<i64>,
    name: Option<String>,
    pub(crate) per_page: i64,
    pub(crate) page: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutorProcessorBindId {
    pub(crate) executor_processor_bind_id: i64,
}

pub(crate) struct ExecutorProcessorBindQueryBuilder;
impl ExecutorProcessorBindQueryBuilder {
    pub(crate) fn query_all_columns() -> executor_processor_bind::BoxedQuery<'static, Mysql> {
        executor_processor_bind::table
            .into_boxed()
            .select(executor_processor_bind::all_columns)
    }

    pub(crate) fn query_binding_columns(
    ) -> executor_processor_bind::BoxedQuery<'static, Mysql, (sql_types::Bigint, sql_types::VarChar)>
    {
        executor_processor_bind::table
            .into_boxed()
            .select((executor_processor_bind::id, executor_processor_bind::name))
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
        if let Some(executor_processor_bind_id) = self.id {
            statement_builder = statement_builder
                .filter(executor_processor_bind::id.eq(executor_processor_bind_id));
        }

        if let Some(executor_processor_bind_group_id) = self.group_id {
            statement_builder = statement_builder
                .filter(executor_processor_bind::group_id.eq(executor_processor_bind_group_id));
        }

        if let Some(executor_processor_bind_executor_id) = self.executor_id {
            statement_builder = statement_builder.filter(
                executor_processor_bind::executor_id.eq(executor_processor_bind_executor_id),
            );
        }

        if let Some(executor_processor_bind_name) = self.name {
            statement_builder = statement_builder
                .filter(executor_processor_bind::name.like(executor_processor_bind_name));
        }

        statement_builder.order(executor_processor_bind::id.desc())
    }
}
