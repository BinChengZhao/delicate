pub mod task {
    pub enum State {
        NotEnabled = 1,
         Enabled = 2,
         Deleted = 3,
    }
}

pub mod task_log {

    pub enum State {
        Running = 1,
        NormalEnding = 2,
        AbnormalEnding = 3,
        TimeoutEnding = 4,
        TmanualCancellation = 5,
    }
}

pub mod user {

    pub enum State {
        Health = 1,
        Forbidden = 2,
    }
}

pub mod user_auth {
    pub enum State {
        Health = 1,
        Forbidden = 2,
    }
}

pub mod executor_processor {
    pub enum State {
        Health = 1,
        Forbidden = 2,
    }
}

pub mod executor_group {
    pub enum State {
        Health = 1,
        Forbidden = 2,
    }
}
