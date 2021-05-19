use super::prelude::*;
use super::schema::{user, user_auth};
use ring::digest::{digest, SHA256};
use validator::Validate;

// FIXME: The user's password is encrypted by sha-256 and stored in the database, with low MD5 security.
// Using ring-crate.

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct QueryNewUser {
    #[validate(length(min = 8))]
    user_name: String,
    #[validate(length(min = 1))]
    nick_name: String,
    #[validate(phone)]
    mobile: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 8))]
    certificate: String,
}

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
}

impl From<&QueryNewUser> for NewUser {
    fn from(value: &QueryNewUser) -> NewUser {
        NewUser {
            user_name: value.user_name.clone(),
            nick_name: value.nick_name.clone(),
            mobile: value.mobile.clone(),
            email: value.email.clone(),
        }
    }
}

pub struct NewUserAuths(pub [NewUserAuth; 3]);
//QueryNewUser

impl From<(QueryNewUser, u64)> for NewUserAuths {
    fn from(
        (
            QueryNewUser {
                user_name,
                mobile,
                email,
                certificate,
                ..
            },
            user_id,
        ): (QueryNewUser, u64),
    ) -> NewUserAuths {
        let user_auth_arr: [NewUserAuth; 3];
        let encrypted_certificate: String;
        let encrypted_certificate_digest = digest(&SHA256, certificate.as_bytes());

        unsafe {
            encrypted_certificate =
                std::str::from_utf8_unchecked(encrypted_certificate_digest.as_ref()).to_string();
        }

        let mobile_auth = NewUserAuth {
            user_id,
            identity_type: 1,
            identifier: mobile,
            certificate: encrypted_certificate.clone(),
            status: 1,
        };
        let email_auth = NewUserAuth {
            user_id,
            identity_type: 2,
            identifier: email,
            certificate: encrypted_certificate.clone(),
            status: 1,
        };
        let username_auth = NewUserAuth {
            user_id,
            identity_type: 3,
            identifier: user_name,
            certificate: encrypted_certificate,
            status: 1,
        };

        user_auth_arr = [mobile_auth, email_auth, username_auth];

        NewUserAuths(user_auth_arr)
    }
}

#[derive(Queryable, Identifiable, AsChangeset, Debug, Clone, Serialize, Deserialize)]
#[table_name = "user_auth"]
pub struct UserAuth {
    id: i64,
    user_id: u64,
    identity_type: u8,
    identifier: String,
    certificate: String,
    status: i8,
    created_time: NaiveDateTime,
    updated_time: NaiveDateTime,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "user_auth"]
pub struct NewUserAuth {
    user_id: u64,
    identity_type: u8,
    identifier: String,
    certificate: String,
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
    pub(crate) per_page: i64,
    pub(crate) page: i64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct PaginateUser {
    users: Vec<User>,
    per_page: i64,
    total_page: i64,
}

impl PaginateUser {
    pub(crate) fn set_users(mut self, users: Vec<User>) -> Self {
        self.users = users;
        self
    }

    pub(crate) fn set_per_page(mut self, per_page: i64) -> Self {
        self.per_page = per_page;
        self
    }

    pub(crate) fn set_total_page(mut self, total: i64) -> Self {
        self.total_page = (total as f64 / self.per_page as f64).ceil() as i64;
        self
    }
}

pub(crate) struct UserQueryBuilder;
impl UserQueryBuilder {
    pub(crate) fn query_all_columns() -> user::BoxedQuery<'static, Mysql> {
        user::table.into_boxed().select(user::all_columns)
    }

    pub(crate) fn query_count() -> user::BoxedQuery<'static, Mysql, diesel::sql_types::Bigint> {
        user::table.into_boxed().count()
    }
}

impl QueryParamsUser {
    pub(crate) fn query_filter<ST>(
        self,
        mut statement_builder: user::BoxedQuery<'static, Mysql, ST>,
    ) -> user::BoxedQuery<'static, Mysql, ST> {
        statement_builder = statement_builder.filter(user::status.ne(2));
        // Maybe status 2 eq task-deleted status.

        if let Some(user_id) = self.id {
            statement_builder = statement_builder.filter(user::id.eq(user_id));
        }

        if let Some(status) = self.status {
            statement_builder = statement_builder.filter(user::status.eq(status));
        } else {
            statement_builder =
                statement_builder.filter(user::status.ne(state::user::State::Forbidden as i8));
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
