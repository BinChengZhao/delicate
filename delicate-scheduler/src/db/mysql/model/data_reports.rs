use super::prelude::*;

#[derive(Queryable, Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TaskState {
    hour_num: i16,
    pub(crate) status: i16,
    total: i64,
}
