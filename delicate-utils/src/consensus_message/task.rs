use crate::prelude::*;
#[derive(Queryable, Clone, Debug, Default, Serialize, Deserialize, Display)]
#[display(
    fmt = "task-id:{} command:{} frequency:{} cron_expression:{} timeout:{} maximun_parallel_runnable_num:{} host:{}",
    id,
    command,
    frequency,
    cron_expression,
    timeout,
    maximun_parallel_runnable_num,
    host
)]

pub struct TaskPackage {
    pub id: i64,
    command: String,
    frequency: String,
    cron_expression: String,
    timeout: i16,
    maximun_parallel_runnable_num: i16,
    pub host: String,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]

pub struct SignedTaskPackage {
    task_package: TaskPackage,
    pub signature: Vec<u8>,
}

impl TaskPackage {
    pub fn sign(self, token: String) -> Result<SignedTaskPackage, crate::error::CommonError> {
        let json_str = to_json_string(&self)?;

        let signature = if token.is_empty() {
            Vec::default()
        } else {
            let raw_str = json_str + &token;
            digest(&SHA256, raw_str.as_bytes()).as_ref().to_vec()
        };

        Ok(SignedTaskPackage {
            task_package: self,
            signature,
        })
    }
}

#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize, Display)]
#[display(fmt = "task-id:{} time:{}", task_id, time)]

pub struct SuspendTaskRecord {
    pub task_id: i64,
    pub time: u64,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]

pub struct SignedSuspendTaskRecord {
    suspend_task_record: SuspendTaskRecord,
    pub signature: Vec<u8>,
}

impl SuspendTaskRecord {
    pub fn set_task_id(mut self, task_id: i64) -> Self {
        self.task_id = task_id;
        self
    }

    pub fn set_time(mut self, time: u64) -> Self {
        self.time = time;
        self
    }
    pub fn sign(self, token: String) -> Result<SignedSuspendTaskRecord, crate::error::CommonError> {
        let json_str = to_json_string(&self)?;

        let raw_str = json_str + &token;
        let signature = digest(&SHA256, raw_str.as_bytes()).as_ref().to_vec();
        Ok(SignedSuspendTaskRecord {
            suspend_task_record: self,
            signature,
        })
    }
}
