use super::prelude::*;
use super::schema::{user, user_auth};

lazy_static! {
    static ref RE_USER_NAME: Regex = Regex::new(r"[a-zA-Z][a-zA-Z0-9_]{5,32}$").unwrap();
}

lazy_static! {
    static ref RE_MOBILE: Regex = Regex::new(r"\d{11}$").unwrap();
}
#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct QueryNewUser {
    #[validate(regex = "RE_USER_NAME")]
    pub(crate) user_name: String,
    #[validate(length(min = 5))]
    pub(crate) nick_name: String,
    #[validate(regex = "RE_MOBILE")]
    pub(crate) mobile: String,
    #[validate(email)]
    pub(crate) email: String,
    #[validate(length(min = 8))]
    pub(crate) certificate: String,
}

#[derive(Queryable, Identifiable, AsChangeset, Debug, Clone, Serialize, Deserialize)]
#[table_name = "user"]

pub struct User {
    pub id: u64,
    pub user_name: String,
    pub nick_name: String,
    pub mobile: String,
    pub email: String,
    pub face: String,
    pub status: i8,
    pub created_time: NaiveDateTime,
    pub updated_time: NaiveDateTime,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[table_name = "user"]
pub struct NewUser {
    user_name: String,
    nick_name: String,
    mobile: String,
    email: String,
}

#[derive(Queryable, Identifiable, AsChangeset, Debug, Clone, Serialize, Deserialize)]
#[table_name = "user"]

pub struct UpdateUser {
    pub(crate) id: u64,
    user_name: String,
    nick_name: String,
    mobile: String,
    email: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct UserChangePassword {
    pub current_password: String,
    pub modified_password: String,
    pub identity_type: u8,
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

        let encrypted_certificate = get_encrypted_certificate_by_raw_certificate(&certificate);

        let mobile_auth = NewUserAuth {
            user_id,
            identity_type: types::IdentityType::Mobile as u8,
            identifier: mobile,
            certificate: encrypted_certificate.clone(),
            status: state::user_auth::State::Health as i8,
        };
        let email_auth = NewUserAuth {
            user_id,
            identity_type: types::IdentityType::Email as u8,
            identifier: email,
            certificate: encrypted_certificate.clone(),
            status: state::user_auth::State::Health as i8,
        };
        let username_auth = NewUserAuth {
            user_id,
            identity_type: types::IdentityType::Username as u8,
            identifier: user_name,
            certificate: encrypted_certificate,
            status: state::user_auth::State::Health as i8,
        };

        let user_auth_arr: [NewUserAuth; 3] = [mobile_auth, email_auth, username_auth];

        NewUserAuths(user_auth_arr)
    }
}

pub fn get_encrypted_certificate_by_raw_certificate(certificate: &str) -> String {
    let encrypted_certificate_digest = digest(&SHA256, certificate.as_bytes());

    hex::encode(encrypted_certificate_digest.as_ref())
}

#[derive(Queryable, Identifiable, AsChangeset, Debug, Clone, Serialize, Deserialize)]
#[table_name = "user_auth"]
pub struct UserAuth {
    pub id: i64,
    pub user_id: u64,
    pub identity_type: u8,
    pub identifier: String,
    pub certificate: String,
    pub status: i8,
    pub created_time: NaiveDateTime,
    pub updated_time: NaiveDateTime,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAuthLogin {
    pub(crate) login_type: u8,
    pub(crate) account: String,
    pub(crate) password: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct QueryParamsUser {
    id: Option<u64>,
    user_name: Option<String>,
    nick_name: Option<String>,
    mobile: Option<String>,
    email: Option<String>,
    status: Option<i8>,
    pub(crate) per_page: i64,
    pub(crate) page: i64,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub(crate) struct UserId {
    pub(crate) user_id: u64,
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


#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct UserName {
    pub user_name: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct UserAndRoles {
    pub user_name: String,
    pub operate_roles: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct UserAndPermissions {
    pub user_name: String,
    pub operate_permissions: Vec<Vec<String>>,
}



