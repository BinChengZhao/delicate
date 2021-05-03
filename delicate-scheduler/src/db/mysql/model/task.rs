use super::schema::task;

#[derive(Queryable, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Task {
    id: i64,
    name: String,
    description: String,
    command: String,
    frequency: String,
    cron_expression: String,
    timeout: i16,
    retry_times: i16,
    retry_interval: i16,
    maximun_parallel_runable_num: i16,
    tag: String,
    status: i16,
}

#[derive(Insertable, Debug, Default, Serialize, Deserialize)]
#[table_name = "task"]
pub struct NewTask {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) command: String,
    pub(crate) frequency: String,
    pub(crate) cron_expression: String,
    pub(crate) timeout: i16,
    pub(crate) retry_times: i16,
    pub(crate) retry_interval: i16,
    pub(crate) maximun_parallel_runable_num: i16,
    pub(crate) tag: String,
    pub(crate) status: i16,
}
