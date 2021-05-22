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
    deleted_time: NaiveDateTime,
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