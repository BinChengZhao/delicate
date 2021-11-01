use crate::prelude::*;

#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize, Display)]
#[display(fmt = "task-id:{} record-id:{} time:{}", task_id, record_id, time)]

pub struct CancelTaskRecord {
    pub task_id: i64,
    pub record_id: i64,
    pub time: u64,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, Display)]
#[display(fmt = "cancel-task-record:{} ", cancel_task_record)]

pub struct SignedCancelTaskRecord {
    pub cancel_task_record: CancelTaskRecord,
    #[serde(with = "hex")]
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
    pub fn sign(self,
                token: Option<&str>)
                -> Result<SignedCancelTaskRecord, crate::error::CommonError> {
        let signature = make_signature(&self, token)?;

        Ok(SignedCancelTaskRecord { cancel_task_record: self, signature })
    }
}

impl SignedCancelTaskRecord {
    pub fn verify(&self, token: Option<&str>) -> Result<(), crate::error::CommonError> {
        let SignedCancelTaskRecord { ref cancel_task_record, ref signature } = self;

        verify_signature_by_raw_data(cancel_task_record, token, signature)
    }

    pub fn get_cancel_task_record_after_verify(
        self,
        token: Option<&str>)
        -> Result<CancelTaskRecord, crate::error::CommonError> {
        self.verify(token)?;
        let SignedCancelTaskRecord { cancel_task_record, .. } = self;

        Ok(cancel_task_record)
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ExecutorEventCollection {
    pub events: Vec<ExecutorEvent>,
    timestamp: i64,
}

impl From<Vec<ExecutorEvent>> for ExecutorEventCollection {
    fn from(events: Vec<ExecutorEvent>) -> Self {
        let timestamp = get_timestamp() as i64;
        ExecutorEventCollection { events, timestamp }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SignedExecutorEventCollection {
    pub event_collection: ExecutorEventCollection,
    #[serde(with = "hex")]
    signature: Vec<u8>,
}

impl ExecutorEventCollection {
    pub fn sign(self,
                token: Option<&str>)
                -> Result<SignedExecutorEventCollection, crate::error::CommonError> {
        let signature = make_signature(&self, token)?;

        Ok(SignedExecutorEventCollection { event_collection: self, signature })
    }
}

impl SignedExecutorEventCollection {
    pub fn verify(&self, token: Option<&str>) -> Result<(), crate::error::CommonError> {
        let SignedExecutorEventCollection { ref event_collection, ref signature } = self;

        verify_signature_by_raw_data(event_collection, token, signature)
    }

    pub fn get_executor_event_collection_after_verify(
        self,
        token: Option<&str>)
        -> Result<ExecutorEventCollection, crate::error::CommonError> {
        self.verify(token)?;
        let SignedExecutorEventCollection { event_collection, .. } = self;

        Ok(event_collection)
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

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventType {
    TaskPerform = 1,
    TaskFinish = 2,
    TaskTimeout = 3,
    Unknown    = 81,
}

impl From<i16> for EventType {
    fn from(value: i16) -> Self {
        match value {
            1 => EventType::TaskPerform,
            2 => EventType::TaskFinish,
            3 => EventType::TaskTimeout,
            _ => EventType::Unknown,
        }
    }
}

impl From<PublicFinishOutput> for FinishOutput {
    fn from(value: PublicFinishOutput) -> Self {
        match value {
            PublicFinishOutput::ExceptionOutput(str) => FinishOutput::ExceptionOutput(str),
            PublicFinishOutput::ProcessOutput(child_output) => {
                FinishOutput::ProcessOutput(child_output.into())
            },
        }
    }
}

impl From<StdOutput> for ChildOutput {
    fn from(value: StdOutput) -> Self {
        let StdOutput { status, stdout, stderr } = value;

        let child_status = status.code().unwrap_or(81);
        let child_stdout = String::from_utf8_lossy(&stdout).into_owned();
        let child_stderr = String::from_utf8_lossy(&stderr).into_owned();

        ChildOutput { child_status, child_stdout, child_stderr }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FinishOutput {
    ProcessOutput(ChildOutput),
    ExceptionOutput(String),
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ChildOutput {
    pub child_status: i32,
    pub child_stdout: String,
    pub child_stderr: String,
}
