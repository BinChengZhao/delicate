pub mod task {
    #[allow(dead_code)]
    pub enum State {
        NotEnabled = 1,
        Enabled = 2,
        Deleted = 3,
    }
}

pub mod task_log {

    #[allow(dead_code)]
    pub enum State {
        Running = 1,
        NormalEnding = 2,
        AbnormalEnding = 3,
        TimeoutEnding = 4,
        TmanualCancellation = 5,
        Unknown = 81,
    }

    impl From<i16> for State {
        fn from(v: i16) -> State {
            match v {
                1 => State::Running,
                2 => State::NormalEnding,
                3 => State::AbnormalEnding,
                4 => State::TimeoutEnding,
                5 => State::TmanualCancellation,
                _ => State::Unknown,
            }
        }
    }
}

pub mod user {

    #[allow(dead_code)]
    pub enum State {
        Health = 1,
        Forbidden = 2,
    }
}

pub mod user_auth {
    #[allow(dead_code)]
    pub enum State {
        Health = 1,
        Forbidden = 2,
    }
}

pub mod executor_processor {
    #[allow(dead_code)]
    pub enum State {
        NotEnabled = 1,
        Enabled = 2,
        Abnormal = 3,
    }
}

pub mod executor_group {
    #[allow(dead_code)]
    pub enum State {
        Health = 1,
        Forbidden = 2,
    }
}
