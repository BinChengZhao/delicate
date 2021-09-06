#[allow(unused_imports)]
pub use crate::prelude::*;

pub trait DescribeState: Into<&'static str> {
    fn state_name() -> &'static str;

    fn desc() -> HashMap<usize, &'static str>;
}

macro_rules! impl_state_desc_unify{
    ($($target:ty => $name:expr),+) => {
        $(impl DescribeState for $target {

            fn state_name() -> &'static str
            {
                $name
            }

            fn desc() -> HashMap<usize, &'static str> {
                <$target>::iter()
                    .map(|state| (state as usize, state.into()))
                    .collect()
            }
        }
        )+
    }
}

impl_state_desc_unify!(task::State=>"task", task_log::State=>"task_log", user::State=>"user", user_auth::State=>"user_auth", executor_processor::State=>"executor_processor", executor_group::State=>"executor_group", operation_log::OperationType=>"operation_type", user_login_log::LoginType=>"user_login_type", user_login_log::LoginCommand=>"user_login_command");

pub mod task {

    use super::*;
    #[allow(dead_code)]
    #[derive(Copy, Clone, StrumToString, Debug, EnumIter, AsRefStr, IntoStaticStr)]
    pub enum State {
        NotEnabled = 1,
        Enabled = 2,
        Deleted = 3,
    }
}

pub mod task_log {
    use super::*;

    #[allow(dead_code)]
    #[derive(Copy, Clone, StrumToString, Debug, EnumIter, AsRefStr, IntoStaticStr)]
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
    use super::*;

    #[allow(dead_code)]
    #[derive(Copy, Clone, StrumToString, Debug, EnumIter, AsRefStr, IntoStaticStr)]
    pub enum State {
        Health = 1,
        Forbidden = 2,
    }
}

pub mod user_auth {
    use super::*;

    #[allow(dead_code)]
    #[derive(Copy, Clone, StrumToString, Debug, EnumIter, AsRefStr, IntoStaticStr)]
    pub enum State {
        Health = 1,
        Forbidden = 2,
    }
}

pub mod executor_processor {
    use super::*;

    #[allow(dead_code)]
    #[derive(Copy, Clone, StrumToString, Debug, EnumIter, AsRefStr, IntoStaticStr)]
    pub enum State {
        NotEnabled = 1,
        Enabled = 2,
        Abnormal = 3,
    }
}

pub mod executor_group {
    use super::*;

    #[allow(dead_code)]
    #[derive(Copy, Clone, StrumToString, Debug, EnumIter, AsRefStr, IntoStaticStr)]
    pub enum State {
        Health = 1,
        Forbidden = 2,
    }
}

pub mod operation_log {
    use super::*;

    #[derive(Copy, Clone, StrumToString, Debug, EnumIter, AsRefStr, IntoStaticStr)]

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
    use super::*;

    #[allow(dead_code)]
    #[derive(Copy, Clone, StrumToString, Debug, EnumIter, AsRefStr, IntoStaticStr)]
    pub enum LoginType {
        Mobile = 1,
        Email = 2,
        UserName = 3,
        Ldap = 4,
        OtherOAuth = 5,
        Logout = 81,
    }

    #[allow(dead_code)]
    #[derive(Copy, Clone, StrumToString, Debug, EnumIter, AsRefStr, IntoStaticStr)]
    pub enum LoginCommand {
        LoginSuccess = 1,
        LogoutSuccess = 2,
        Loginfailure = 3,
        Logoutfailure = 4,
    }
}
