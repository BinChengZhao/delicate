use crate::prelude::*;

#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize)]

pub struct CancelTaskRecord {
    pub task_id: i64,
    pub record_id: i64,
    pub time: u64,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]

pub struct SignedCancelTaskRecord {
    cancel_task_record: CancelTaskRecord,
    pub signature: Vec<u8>,
}

impl CancelTaskRecord {
    pub fn set_record_id(mut self, record_id: i64) -> Self {
        self.record_id = record_id;
        self
    }

    pub fn set_task_id(mut self, task_id: i64) -> Self {
        self.task_id = task_id;
        self
    }

    pub fn set_time(mut self, time: u64) -> Self {
        self.time = time;
        self
    }
    pub fn sign(
        self,
        token: Option<String>,
    ) -> Result<SignedCancelTaskRecord, crate::error::CommonError> {
        let json_str = to_json_string(&self)?;

        let signature = token
            .map(|t| {
                let raw_str = json_str + &t;
                digest(&SHA256, raw_str.as_bytes()).as_ref().to_vec()
            })
            .unwrap_or_default();

        Ok(SignedCancelTaskRecord {
            cancel_task_record: self,
            signature,
        })
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ExecutorEventCollection {
    pub events: Vec<ExecutorEvent>,
    signature: String,
    timestamp: i64,
}

impl ExecutorEventCollection {
    pub fn verify_signature(&self, _token: &str) -> bool {
        todo!();
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ExecutorEvent {
    pub task_id: i64,
    pub id: i64,
    pub event_type: i16,
    pub executor_processor_id: i64,
    pub executor_processor_name: String,
    pub executor_processor_host: String,
    pub output: Option<FinishOutput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FinishOutput {
    ProcessOutput(ChildOutput),
    ExceptionOutput(String),
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ChildOutput {
    pub child_status: i32,
    pub child_stdout: Vec<u8>,
    pub child_stderr: Vec<u8>,
}
