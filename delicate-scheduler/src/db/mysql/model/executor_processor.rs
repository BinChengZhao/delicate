use super::prelude::*;
use super::schema::{executor_processor};


#[derive(Queryable, Identifiable, AsChangeset, Debug, Clone, Serialize, Deserialize)]
#[table_name = "executor_processor"]

pub struct ExecutorProcessor {
    id: i64,
    name: String,
    host: String,
    port: i16,
    description: String,
    tag: String,
    status: i16,
    created_time: NaiveDateTime,
    deleted_time: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "executor_processor"]
pub struct NewExecutorProcessor {
    name: String,
    host: String,
    port: i16,
    description: String,
    tag: String,
    status: i16,
    created_time: NaiveDateTime,
    deleted_time: Option<NaiveDateTime>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct QueryParamsExecutorProcessor {
    id: Option<i64>,
    name: Option<String>,
    description: Option<String>,
    command: Option<String>,
    frequency: Option<String>,
    cron_expression: Option<String>,
    tag: Option<String>,
    status: Option<i16>,
    pub(crate) per_page : i64,
    pub(crate) page : i64,
}


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct PaginateExecutorProcessor{
    executor_processors : Vec<ExecutorProcessor>,
    per_page : i64,
    total_page : i64
}

impl PaginateExecutorProcessor{
    pub(crate) fn set_tasks(mut self, executor_processors:Vec<ExecutorProcessor>) ->Self{
        self.executor_processors =executor_processors;
        self
    }

    pub(crate) fn set_per_page(mut self, per_page:i64) ->Self{
        self.per_page = per_page;
        self
    }

    pub(crate) fn set_total_page(mut self, total:i64) ->Self{
        self.total_page = (total as f64 / self.per_page as f64).ceil() as i64;
        self
    }
}

pub(crate) struct ExecutorProcessorQueryBuilder;
impl ExecutorProcessorQueryBuilder{
    pub(crate) fn query_all_columns() -> executor_processor::BoxedQuery<'static, Mysql>{
        executor_processor::table
        .into_boxed()
        .select(executor_processor::all_columns)
    }

    pub(crate) fn query_count()-> executor_processor::BoxedQuery<'static, Mysql, diesel::sql_types::Bigint>{
        executor_processor::table
        .into_boxed()
        .count()
    }
}


impl QueryParamsExecutorProcessor {

    pub(crate) fn query_filter<ST>(self, mut statement_builder : executor_processor::BoxedQuery<'static, Mysql, ST>) -> executor_processor::BoxedQuery<'static, Mysql, ST> {
        statement_builder = statement_builder
            .filter(executor_processor::status.ne(2));
            // Maybe status 2 eq task-deleted status.

        if let Some(executor_processor_id) = self.id {
            statement_builder = statement_builder.filter(executor_processor::id.eq(executor_processor_id));
        }

        if let Some(task_status) = self.status {
            statement_builder = statement_builder.filter(executor_processor::status.eq(task_status));
        } else {
            //TODO: Addtion state in future.
            statement_builder = statement_builder.filter(executor_processor::status.ne(2));
        }

        if let Some(executor_processor_name) = self.name {
            statement_builder = statement_builder.filter(executor_processor::name.like(executor_processor_name));
        }

        if let Some(executor_processor_description) = self.description {
            statement_builder = statement_builder.filter(executor_processor::description.like(executor_processor_description));
        }

        if let Some(executor_processor_tag) = self.tag {
            statement_builder = statement_builder.filter(executor_processor::tag.like(executor_processor_tag));
        }

        statement_builder.order(executor_processor::id.desc())

    }
}
