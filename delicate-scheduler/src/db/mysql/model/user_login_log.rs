use super::prelude::*;
use super::schema::{user_login_log};

#[derive(Queryable, Identifiable, AsChangeset, Debug, Clone, Serialize, Deserialize)]
#[table_name = "user_login_log"]

pub struct UserLoginLog {
    id: i64,
    user_id: u64,
    login_type: u8,
    command: u8,
    lastip: String,
    created_time: NaiveDateTime,
    user_name: String,
}

#[derive(Debug, Clone, Serialize)]

pub struct FrontEndUserLoginLog {
    id: i64,
    user_id: u64,
    login_type: u8,
    login_type_desc: &'static str,
    command: u8,
    command_desc: &'static str,
    lastip: String,
    created_time: NaiveDateTime,
    user_name: String,
}

impl From<UserLoginLog> for FrontEndUserLoginLog{
    fn from(log: UserLoginLog) -> Self {
        let UserLoginLog{id, user_id, login_type, command, lastip, created_time, user_name} = log;
        let login_type_desc = Into::<state::user_login_log::LoginType>::into(login_type as i16).into();
        let command_desc = Into::<state::user_login_log::LoginCommand>::into(command as i16).into();

        FrontEndUserLoginLog {
            id,
            user_id,
            login_type,
            login_type_desc,
            command,
            command_desc,
            lastip,
            created_time,
            user_name,
        }
    }
}


#[derive(Insertable, Debug, Default, Serialize, Deserialize)]
#[table_name = "user_login_log"]
pub struct NewUserLoginLog {
    user_id: u64,
    login_type: u8,
    command: u8,
    lastip: String,
    user_name: String,
}

impl NewUserLoginLog {
    pub fn set_user_id(&mut self, user_id:u64) -> &mut Self{
        self.user_id = user_id;
        self
    }

    pub fn set_login_type(&mut self, login_type:u8) -> &mut Self{
        self.login_type = login_type;
        self
    }

    pub fn set_command(&mut self, command:u8) -> &mut Self{
        self.command = command;
        self
    }

    pub fn set_lastip(&mut self, lastip:Option<&str>) -> &mut Self{
        if let Some(ip) = lastip{
            self.lastip = ip.to_string();
        }
        self
    }

    pub fn set_user_name(&mut self, user_name:String) -> &mut Self{
        self.user_name = user_name;
        self
    }

}


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct QueryParamsUserLoginLog {
    id: Option<i64>,
    user_id: Option<u64>,
    login_type: Option<u8>,
    command: Option<u8>,
    lastip: Option<String>,
    user_name: Option<String>,
    pub(crate) start_time: Option<String>,
    pub(crate) end_time: Option<String>,
    pub(crate) per_page: i64,
    pub(crate) page: i64,
}


pub(crate) struct UserLoginLogQueryBuilder;
impl UserLoginLogQueryBuilder {
    pub(crate) fn query_all_columns() -> user_login_log::BoxedQuery<'static, Mysql> {
        user_login_log::table
            .into_boxed()
            .select(user_login_log::all_columns)
    }

    pub(crate) fn query_count(
    ) -> user_login_log::BoxedQuery<'static, Mysql, diesel::sql_types::Bigint> {
        user_login_log::table.into_boxed().count()
    }
}

impl QueryParamsUserLoginLog {
    pub(crate) fn query_filter<ST>(
        self,
        mut statement_builder: user_login_log::BoxedQuery<'static, Mysql, ST>,
    ) -> user_login_log::BoxedQuery<'static, Mysql, ST> {

        if let Some(id) = self.id {
            statement_builder = statement_builder.filter(user_login_log::id.eq(id));
        }

        if let Some(user_id) = self.user_id {
            statement_builder = statement_builder.filter(user_login_log::user_id.eq(user_id));
        }


        if let Some(login_type) = self.login_type {
            statement_builder = statement_builder.filter(user_login_log::login_type.eq(login_type));
        }

        if let Some(command) = self.command {
            statement_builder = statement_builder.filter(user_login_log::command.eq(command));
        }

        if let Some(lastip) = self.lastip {
            statement_builder = statement_builder
                .filter(user_login_log::user_name.like(lastip));
        }


        if let Some(user_name) = self.user_name {
            statement_builder = statement_builder
                .filter(user_login_log::user_name.like(user_name));
        }

        if let Some(Ok(start_time)) = self.start_time.map(|s|NaiveDateTime::parse_from_str(&s,  "%Y-%m-%d %H:%M:%S")) {
            let end_time = self.end_time.map(|s|NaiveDateTime::parse_from_str(&s,  "%Y-%m-%d %H:%M:%S").unwrap_or_else(|_| start_time + ChronoDuration::days(3))).unwrap_or_else(|| start_time + ChronoDuration::days(3));

            statement_builder =
                statement_builder.filter(user_login_log::created_time.between(start_time, end_time));
        }

        statement_builder.order(user_login_log::id.desc())
    }
}
