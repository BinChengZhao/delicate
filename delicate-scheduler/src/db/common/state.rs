pub mod task {
    pub const NOT_ENABLED: usize = 1 << 0;

    pub const ENABLED: usize = 1 << 1;

    pub const DELETED: usize = 1 << 2;
}

pub mod task_log {
    pub const RUNNING: i16 = 1 << 0;

    pub const NORMAL_ENDING: i16 = 1 << 1;

    pub const ABNORMAL_ENDING: i16 = 1 << 2;

    pub const TIMEOUT_ENDING: i16 = 1 << 3;

    pub const TMANUAL_CANCELLATION: i16 = 1 << 4;
}

pub mod user {
    pub const HEALTH: usize = 1 << 0;

    pub const FORBIDDEN: usize = 1 << 1;
}

pub mod user_auth {
    pub const HEALTH: usize = 1 << 0;

    pub const FROZEN: usize = 1 << 1;
}

pub mod executor_processor {
    pub const HEALTH: usize = 1 << 0;

    pub const DELETED: usize = 1 << 1;
}

pub mod executor_group {
    pub const HEALTH: usize = 1 << 0;

    pub const DELETED: usize = 1 << 1;
}
