use super::schema::task;

#[derive(Queryable, Debug, Serialize, Deserialize)]
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

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "task"]
pub struct NewTask {
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
