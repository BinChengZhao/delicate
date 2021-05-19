use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::r2d2::{Builder, ConnectionManager, Pool, PooledConnection};
use std::env;

pub(crate) use super::prelude;

pub(crate) mod extension;
pub(crate) mod model;
pub(crate) mod schema;

pub(crate) type ConnectionPool = Pool<ConnectionManager<MysqlConnection>>;
pub(crate) type PoolConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

#[allow(dead_code)]
pub(crate) fn establish_connection() -> MysqlConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub(crate) fn get_connection_pool() -> Pool<ConnectionManager<MysqlConnection>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager: ConnectionManager<MysqlConnection> = ConnectionManager::new(database_url);
    Builder::new()
        .max_size(1024)
        .min_idle(Some(128))
        .build(manager)
        .expect("Connection pool initialization failed")
}
