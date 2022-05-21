use crate::prelude::*;
#[derive(Queryable, Clone, Debug, Default, Serialize, Deserialize, Display)]
#[display(
    fmt = "task-id:{} command:{} frequency:{} cron_expression:{} timeout:{} maximum_parallel_runnable_num:{}",
    id,
    command,
    frequency,
    cron_expression,
    timeout,
    maximum_parallel_runnable_num
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
    /// Maximum parallel runnable num (optional).
    pub maximum_parallel_runnable_num: i16,
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

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct FrequencyObject {
    pub mode: i8,
    pub extend: FrequencyExtend,
    pub time_zone: u8,
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct FrequencyExtend {
    pub count: u64,
}
#[derive(Clone, Default, Debug, Serialize, Deserialize)]

pub struct SignedTaskPackage {
    pub task_package: TaskPackage,
    #[serde(with = "hex")]
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

pub struct TaskUnit {
    pub task_id: i64,
    pub time: u64,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, Display)]
#[display(fmt = "task-unit:{} ", task_unit)]

pub struct SignedTaskUnit {
    pub task_unit: TaskUnit,
    #[serde(with = "hex")]
    pub signature: Vec<u8>,
}

impl TaskUnit {
    pub fn set_task_id(mut self, task_id: i64) -> Self {
        self.task_id = task_id;
        self
    }

    pub fn set_time(mut self, time: u64) -> Self {
        self.time = time;
        self
    }
    pub fn sign(self, token: Option<&str>) -> Result<SignedTaskUnit, crate::error::CommonError> {
        let signature = make_signature(&self, token)?;
        Ok(SignedTaskUnit {
            task_unit: self,
            signature,
        })
    }
}

impl SignedTaskUnit {
    pub fn verify(&self, token: Option<&str>) -> Result<(), crate::error::CommonError> {
        let SignedTaskUnit {
            ref task_unit,
            ref signature,
        } = self;

        verify_signature_by_raw_data(task_unit, token, signature)
    }

    pub fn get_task_unit_after_verify(
        self,
        token: Option<&str>,
    ) -> Result<TaskUnit, crate::error::CommonError> {
        self.verify(token)?;
        let SignedTaskUnit { task_unit, .. } = self;

        Ok(task_unit)
    }
}

impl TryFrom<TaskPackage> for Task {
    type Error = CommonError;
    fn try_from(task_package: TaskPackage) -> Result<Self, Self::Error> {
        let TaskPackage {
            id,
            command,
            frequency,
            cron_expression,
            timeout,
            maximum_parallel_runnable_num,
            ..
        } = task_package;

        let metadata: FrequencyObject = json_from_slice(frequency.as_bytes())?;
        let cron_expression = &cron_expression;

        let time_zone: ScheduleIteratorTimeZone = match metadata.time_zone {
            1 => ScheduleIteratorTimeZone::Utc,

            2 => ScheduleIteratorTimeZone::Local,

            _ => {
                return Err(CommonError::DisPass(String::from(
                    "Ineffective time-zone mode.",
                )))
            }
        };

        let mut task_builder = TaskBuilder::default();

        task_builder.set_task_id(id as u64);

        match metadata.mode {
            1 => {
                task_builder.set_frequency_once_by_cron_str(cron_expression);
            }
            2 => {
                task_builder
                    .set_frequency_count_down_by_cron_str(cron_expression, metadata.extend.count);
            }
            3 => {
                task_builder.set_frequency_repeated_by_cron_str(cron_expression);
            }
            _ => {
                return Err(CommonError::DisPass(String::from(
                    "Ineffective frequency mode.",
                )));
            }
        }

        let task = task_builder
            .set_schedule_iterator_time_zone(time_zone)
            .set_maximum_running_time(timeout as u64)
            .set_maximum_parallel_runnable_num(maximum_parallel_runnable_num as u64)
            .spawn_async_routine({
                move || tokio_unblock_process_task_fn(command.clone(), id as u64)
            })?;

        Ok(task)
    }
}
