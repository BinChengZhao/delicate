use super::state::task_log::State;
use crate::prelude::*;
use delicate_utils_task_log::EventType;

pub(crate) enum IdentityType {
    Mobile = 1,
    Email = 2,
    Username = 3,
}

impl From<EventType> for State {
    fn from(value: EventType) -> Self {
        match value {
            EventType::TaskPerform => State::Running,
            EventType::TaskFinish => State::NormalEnding,
            EventType::TaskTimeout => State::TimeoutEnding,
            EventType::Unknown => State::Unknown,
        }
    }
}
