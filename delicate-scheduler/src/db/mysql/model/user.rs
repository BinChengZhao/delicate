use super::prelude::*;
use super::schema::{user};

// FIXME: The user's password is encrypted by sha-256 and stored in the database, with low MD5 security.
// Using ring-crate.
#[derive(Queryable, Identifiable, AsChangeset, Debug, Clone, Serialize, Deserialize)]
#[table_name = "user"]

pub struct User {
    id: i64,
    user_name: String,
    nick_name: String,
    mobile: String,
    email: String,
    face: String,
    status: i8,
    created_time: NaiveDateTime,
    updated_time: NaiveDateTime,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "user"]
pub struct NewUser {
    user_name: String,
    nick_name: String,
    mobile: String,
    email: String,
    status: i8,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct QueryParamsUser {
    id: Option<i64>,
    user_name: Option<String>,
    nick_name: Option<String>,
    mobile: Option<String>,
    email: Option<String>,
    face: Option<String>,
    status: Option<i8>,
    pub(crate) per_page : i64,
    pub(crate) page : i64,
}


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct PaginateUser{
    users : Vec<User>,
    per_page : i64,
    total_page : i64
}

impl PaginateUser{
    pub(crate) fn set_users(mut self, users:Vec<User>) ->Self{
        self.users =users;
        self
    }

    pub(crate) fn set_per_page(mut self, per_page:i64) ->Self{
        self.per_page = per_page;
        self
    }

    pub(crate) fn set_total_page(mut self, total:i64) ->Self{
        self.total_page = (total as f64 / self.per_page as f64).ceil() as i64;
        self
    }
}

pub(crate) struct UserQueryBuilder;
impl UserQueryBuilder{
    pub(crate) fn query_all_columns() -> user::BoxedQuery<'static, Mysql>{
        user::table
        .into_boxed()
        .select(user::all_columns)
    }

    pub(crate) fn query_count()-> user::BoxedQuery<'static, Mysql, diesel::sql_types::Bigint>{
        user::table
        .into_boxed()
        .count()
    }
}


impl QueryParamsUser {

    pub(crate) fn query_filter<ST>(self, mut statement_builder : user::BoxedQuery<'static, Mysql, ST>) -> user::BoxedQuery<'static, Mysql, ST> {
        statement_builder = statement_builder
            .filter(user::status.ne(2));
            // Maybe status 2 eq task-deleted status.

        if let Some(user_id) = self.id {
            statement_builder = statement_builder.filter(user::id.eq(user_id));
        }

        if let Some(status) = self.status {
            statement_builder = statement_builder.filter(user::status.eq(status));
        } else {
            //TODO: Addtion state in future.
            statement_builder = statement_builder.filter(user::status.ne(2));
        }

        if let Some(user_name) = self.user_name {
            statement_builder = statement_builder.filter(user::user_name.like(user_name));
        }

        if let Some(mobile) = self.mobile {
            statement_builder = statement_builder.filter(user::mobile.eq(mobile));
        }

        if let Some(email) = self.email {
            statement_builder = statement_builder.filter(user::email.eq(email));
        }

        statement_builder.order(user::id.desc())

    }
}
