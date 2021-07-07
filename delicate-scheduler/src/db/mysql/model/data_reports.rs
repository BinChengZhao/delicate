use super::prelude::*;

#[derive(Queryable, Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TaskState {
    pub(crate) hour_num: i16,
    pub(crate) status: i16,
    pub(crate) total: i64,
}

#[derive(Queryable, Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct DailyState {
    pub(crate) created: Vec<i64>,
    pub(crate) timeout: Vec<i64>,
    pub(crate) finished: Vec<i64>,
    pub(crate) abnormal: Vec<i64>,
    pub(crate) canceled: Vec<i64>,
    pub(crate) hours_range: Vec<u32>,
}
