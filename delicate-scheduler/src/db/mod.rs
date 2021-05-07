pub(crate) use super::prelude;
use crate::*;

// When introducing different mods based on conditional compilation,
// It affects the formatting of the code within the mod.

// The current interim formatting practice is to remove the conditionally compiled macros (`cfg_mysql_support` or `cfg_postgres_support`),
// Then keep only one class of mods introduced, and then do `cargo fmt`

cfg_mysql_support!(
    pub(crate) mod mysql;
    pub(crate) use mysql::model;
    pub(crate) use mysql::schema;
    pub(crate) use mysql::extension;


    pub(crate) use mysql::{establish_connection, get_connection_pool, ConnectionPool};
    embed_migrations!("./migrations/mysql");

);

cfg_postgres_support!(
    pub(crate) mod postgres;
    pub(crate) use postgres::model;
    pub(crate) use postgres::schema;

    pub(crate) use postgres::{establish_connection, get_connection_pool, ConnectionPool};
    embed_migrations!("./migrations/postgres");
);

pub(crate) fn init() {
    let connection = establish_connection();

    // This will run the necessary migrations.
    embedded_migrations::run(&connection).unwrap();
}
