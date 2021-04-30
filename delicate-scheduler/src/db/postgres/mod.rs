use std::env;
use dotenv::dotenv;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{Builder, ConnectionManager, Pool};

pub(crate) type ConnectionPool = Pool<ConnectionManager<PgConnection>>;

#[allow(dead_code)]
pub(crate) fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[allow(dead_code)]
pub(crate) fn get_connection_pool()->Pool<ConnectionManager<PgConnection>>{

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager:ConnectionManager<PgConnection> = ConnectionManager::new(database_url);
    Builder::new().max_size(1024).min_idle(Some(256)).build(manager).expect("Connection pool initialization failed")
}