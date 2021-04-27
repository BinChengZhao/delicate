use crate::*;

cfg_mysql_support!(
    pub(crate) mod mysql;
    pub(crate) use mysql::model;
    pub(crate) use mysql::schema;


    pub(crate) use mysql::establish_connection;
    embed_migrations!("./migrations/mysql");

);

cfg_postgres_support!(
    pub(crate) mod postgres;
    pub(crate) use postgres::model;
    pub(crate) use postgres::schema;

    pub(crate) use postgres::establish_connection;
    embed_migrations!("./migrations/postgres");
);

pub fn init() {
    let connection = establish_connection();

    // This will run the necessary migrations.
    embedded_migrations::run(&connection).unwrap();
}
