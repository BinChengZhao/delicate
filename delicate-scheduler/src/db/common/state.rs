#[allow(unused_imports)]
pub use crate::prelude::*;

pub mod task {

    use super::*;
    #[allow(dead_code)]
    #[derive(Copy, Clone, StrumToString, Debug, EnumIter, AsRefStr, IntoStaticStr)]
    pub enum State {
        NotEnabled = 1,
        Enabled = 2,
        Deleted = 3,
        Unknown = 81,
    }

    impl From<i16> for State {
        fn from(v: i16) -> State {
            match v {
                1 => State::NotEnabled,
                2 => State::Enabled,
                3 => State::Deleted,
                _ => State::Unknown,
            }
        }
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
        Unknown = 81,
    }

    impl From<i16> for State {
        fn from(v: i16) -> State {
            match v {
                1 => State::Health,
                2 => State::Forbidden,
                _ => State::Unknown,
            }
        }
    }
}

pub mod user_auth {
    use super::*;

    #[allow(dead_code)]
    #[derive(Copy, Clone, StrumToString, Debug, EnumIter, AsRefStr, IntoStaticStr)]
    pub enum State {
        Health = 1,
        Forbidden = 2,
        Unknown = 81,
    }

    impl From<i16> for State {
        fn from(v: i16) -> State {
            match v {
                1 => State::Health,
                2 => State::Forbidden,
                _ => State::Unknown,
            }
        }
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
        Unknown = 81,
    }

    impl From<i16> for State {
        fn from(v: i16) -> State {
            match v {
                1 => State::NotEnabled,
                2 => State::Enabled,
                3 => State::Abnormal,
                _ => State::Unknown,
            }
        }
    }
}

pub mod executor_group {
    use super::*;

    #[allow(dead_code)]
    #[derive(Copy, Clone, StrumToString, Debug, EnumIter, AsRefStr, IntoStaticStr)]
    pub enum State {
        Health = 1,
        Forbidden = 2,
        Unknown = 81,
    }

    impl From<i16> for State {
        fn from(v: i16) -> State {
            match v {
                1 => State::Health,
                2 => State::Forbidden,
                _ => State::Unknown,
            }
        }
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
        Unknown = 88,
    }

    impl From<i16> for LoginType {
        fn from(v: i16) -> LoginType {
            match v {
                1 => LoginType::Mobile,
                2 => LoginType::Email,
                3 => LoginType::UserName,
                4 => LoginType::Ldap,
                5 => LoginType::OtherOAuth,
                81 => LoginType::Logout,
                _ => LoginType::Unknown,
            }
        }
    }

    #[allow(dead_code)]
    #[derive(Copy, Clone, StrumToString, Debug, EnumIter, AsRefStr, IntoStaticStr)]
    pub enum LoginCommand {
        LoginSuccess = 1,
        LogoutSuccess = 2,
        Loginfailure = 3,
        Logoutfailure = 4,
        Unknown = 81,
    }

    impl From<i16> for LoginCommand {
        fn from(v: i16) -> LoginCommand {
            match v {
                1 => LoginCommand::LoginSuccess,
                2 => LoginCommand::LogoutSuccess,
                3 => LoginCommand::Loginfailure,
                4 => LoginCommand::Logoutfailure,
                _ => LoginCommand::Unknown,
            }
        }
    }
}

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

impl_state_desc_unify!(task::State=>"task", task_log::State=>"taskLog", user::State=>"user", user_auth::State=>"userAuth", executor_processor::State=>"executorProcessor", executor_group::State=>"executorGroup", operation_log::OperationType=>"operationType", user_login_log::LoginType=>"userLoginType", user_login_log::LoginCommand=>"userLoginCommand");
