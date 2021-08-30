pub(crate) use super::prelude;
use crate::*;

// When introducing different mods based on conditional compilation,
// It affects the formatting of the code within the mod.

// The current interim formatting practice is to remove the conditionally compiled macros (`cfg_mysql_support` or `cfg_postgres_support`),
// Then keep only one class of mods introduced, and then do `cargo fmt`

cfg_mysql_support!(
    pub(crate) mod mysql;
    pub(crate) use mysql::extension;
    pub(crate) use mysql::model;
    pub(crate) use mysql::schema;

    pub(crate) use mysql::{establish_connection, get_connection_pool, ConnectionPool, PoolConnection};

    no_arg_sql_function!(
        last_insert_id,
        sql_types::Unsigned<sql_types::Bigint>
    );

    embed_migrations!("./migrations/mysql");

);

cfg_postgres_support!(
    pub(crate) mod postgres;
    pub(crate) use postgres::model;
    pub(crate) use postgres::schema;

    pub(crate) use postgres::{establish_connection, get_connection_pool, ConnectionPool};
    embed_migrations!("./migrations/postgres");
);

pub(crate) mod common;

joinable!(schema::task_bind -> schema::task (task_id));
joinable!(schema::task_bind -> schema::executor_processor_bind (bind_id));
joinable!(schema::executor_processor_bind -> schema::executor_processor (executor_id));
joinable!(schema::executor_processor_bind -> schema::executor_group (group_id));
joinable!(schema::user_auth -> schema::user (user_id));
joinable!(schema::task_log_extend -> schema::task_log (id));

pub(crate) fn init() {
    let connection = establish_connection();

    // This will run the necessary migrations.
    embedded_migrations::run(&connection).expect("Migration execution failed, please check the database account permission and database service availability.");
    init_admin_account();
}

pub(crate) fn init_admin_account() {
    use model::QueryNewUser;
    use schema::{user, user_auth};

    let user_name = env::var("INITIAL_ADMINISTRATOR_USER_NAME").unwrap_or_else(|e| {
        println!(
            r"`INITIAL_ADMINISTRATOR_USER_NAME` may not set in the environment variable: {}
        The default login user-name will as `admin`  .
        Please ignore this error if you have set or initialized delicate.",
            e.to_string()
        );
        "admin".into()
    });

    let certificate = env::var("INITIAL_ADMINISTRATOR_PASSWORD").unwrap_or_else(|e| {
        println!(
            r"`INITIAL_ADMINISTRATOR_PASSWORD` may not set in the environment variable: {}
        The default login user-password will as `admin`  .
        Please ignore this error if you have set or initialized delicate.",
            e.to_string()
        );

        "admin".into()
    });
    let nick_name = env::var("INITIAL_ADMINISTRATOR_NICK_NAME").unwrap_or_else(|_| "admin".into());
    let mobile = env::var("INITIAL_ADMINISTRATOR_MOBILE").unwrap_or_else(|_| "12345054321".into());
    let email =
        env::var("INITIAL_ADMINISTRATOR_EMAIL").unwrap_or_else(|_| "admin@admin.com".into());

    let conn = establish_connection();

    let admin = QueryNewUser {
        user_name,
        nick_name,
        mobile,
        email,
        certificate,
    };

    let count: i64 = user::table
        .filter(user::user_name.eq(&admin.user_name))
        .count()
        .get_result(&conn)
        .expect("Init admin-account fail.");

    if count != 0 {
        return;
    }

    conn.transaction::<_, diesel::result::Error, _>(|| {
        diesel::insert_into(user::table)
            .values(&(Into::<model::NewUser>::into(&admin)))
            .execute(&conn)?;

        let last_id = diesel::select(db::last_insert_id).get_result::<u64>(&conn)?;

        let user_auths: model::NewUserAuths =
            From::<(model::QueryNewUser, u64)>::from((admin, last_id));

        diesel::insert_into(user_auth::table)
            .values(&user_auths.0[..])
            .execute(&conn)?;
        Ok(())
    })
    .expect("Init admin-account fail.");
}
