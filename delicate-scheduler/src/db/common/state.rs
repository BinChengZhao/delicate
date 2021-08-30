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

pub mod operation_log {

    pub enum OperationType {
        Addition = 1,
        Modify = 2,
        Delete = 3,
        Unknown = 81,
    }

    impl From<i16> for OperationType {
        fn from(v: i16) -> OperationType {
            match v {
                1 => OperationType::Addition,
                2 => OperationType::Modify,
                3 => OperationType::Delete,
                _ => OperationType::Unknown,
            }
        }
    }
}

pub mod user_login_log {

    #[allow(dead_code)]
    pub enum LoginType {
        Mobile = 1,
        Email = 2,
        UserName = 3,
        Ldap = 4,
        OtherOAuth = 5,
        Logout = 81,
    }

    #[allow(dead_code)]
    pub enum LoginCommand {
        LoginSuccess = 1,
        LogoutSuccess = 2,
        Loginfailure = 3,
        Logoutfailure = 4,
    }
}
