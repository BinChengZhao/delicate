use super::prelude::*;

#[derive(Queryable, Debug, Clone, Serialize, Deserialize)]
pub struct TaskState {
    created_time: NaiveDateTime,
    status: i16,
    count: i64,
}