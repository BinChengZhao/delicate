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

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "user_login_log"]
pub struct NewUserLoginLog {
    user_id: u64,
    login_type: u8,
    command: u8,
    lastip: String,
    created_time: NaiveDateTime,
    user_name: String,
}


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct QueryParamsUserLoginLog {
    id: Option<i64>,
    user_id: Option<u64>,
    login_type: Option<u8>,
    command: Option<u8>,
    lastip: Option<String>,
    user_name: Option<String>,
    pub(crate) start_time: Option<i64>,
    pub(crate) end_time: Option<i64>,
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

        // TODO: Input time-range is a date-string.
        //  Get NaiveDateTime by called `parse_from_str`.
        if let Some(start_time) = self.start_time {
            let end_time = self.end_time.unwrap_or_else(|| start_time + 86400 * 3);

            let start_time = NaiveDateTime::from_timestamp(start_time, 0);
            let end_time = NaiveDateTime::from_timestamp(end_time, 0);

            statement_builder =
                statement_builder.filter(user_login_log::created_time.between(start_time, end_time));
        }

        statement_builder.order(user_login_log::id.desc())
    }
}
