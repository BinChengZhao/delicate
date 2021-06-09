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
    /// Task_id should unique.
    pub id: i64,
    /// Command string.
    pub command: String,
    /// Repeat type and count.
    pub frequency: String,
    /// Cron-expression str.
    pub cron_expression: String,
    /// timeout.
    pub timeout: i16,
    /// Maximum parallel runable num (optional).
    pub maximun_parallel_runnable_num: i16,
    /// Target executor host.
    pub host: String,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum FrequencyModelType {
    Once = 1,
    CountDown = 2,
    Repeat = 3,
}

impl Default for FrequencyModelType {
    fn default() -> Self {
        FrequencyModelType::Repeat
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FrequencyModel<'a> {
    pub metadata: FrequencyObject,
    pub cron_expression: &'a str,
}

impl<'a> TryInto<Frequency<'a>> for FrequencyModel<'a> {
    type Error = CommonError;
    fn try_into(self) -> Result<Frequency<'a>, Self::Error> {
        match self.metadata.mode {
            1 => {}
            // FrequencyModelType::Once as i8 => {}
            // FrequencyModelType::Once as i8 => {}
            _ => {}
        }
        todo!();
    }
}
#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct FrequencyObject {
    pub mode: i8,
    pub extend: FrequencyExtend,
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct FrequencyExtend {
    pub count: u64,
    pub time_zone: u8,
}
#[derive(Clone, Default, Debug, Serialize, Deserialize)]

pub struct SignedTaskPackage {
    pub task_package: TaskPackage,
    pub signature: Vec<u8>,
}

impl TaskPackage {
    pub fn sign(self, token: Option<&str>) -> Result<SignedTaskPackage, crate::error::CommonError> {
        let signature = make_signature(&self, token)?;

        Ok(SignedTaskPackage {
            task_package: self,
            signature,
        })
    }
}

impl SignedTaskPackage {
    pub fn verify(&self, token: Option<&str>) -> Result<(), crate::error::CommonError> {
        let SignedTaskPackage {
            ref task_package,
            ref signature,
        } = self;

        verify_signature_by_raw_data(task_package, token, signature)
    }

    pub fn get_task_package_after_verify(
        self,
        token: Option<&str>,
    ) -> Result<TaskPackage, crate::error::CommonError> {
        self.verify(token)?;
        let SignedTaskPackage { task_package, .. } = self;

        Ok(task_package)
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

impl TryFrom<TaskPackage> for Task {
    type Error = CommonError;
    fn try_from(task_package: TaskPackage) -> Result<Self, Self::Error> {
        // let TaskPackage {
        //     id,
        //     command,
        //     frequency,
        //     cron_expression,
        //     timeout,
        //     maximun_parallel_runnable_num,
        //     ..
        // } = task_package;

        // let frequency: FrequencyObject = json_from_slice(frequency.as_bytes())?;

        // let mut task_builder = TaskBuilder::default();
        // let task = task_builder
        //     .set_task_id(id as u64)
        //     .set_frequency(frequency)
        //     .set_maximum_running_time(timeout as u64)
        //     .set_maximun_parallel_runable_num(maximun_parallel_runnable_num as u64)
        //     .spawn(unblock_process_task_fn(command))?;

        // Ok(task)
        todo!();
    }
}
