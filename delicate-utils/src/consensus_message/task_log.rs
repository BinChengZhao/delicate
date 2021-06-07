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
