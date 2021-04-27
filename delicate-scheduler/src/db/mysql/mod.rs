use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub(crate) mod schema;
pub(crate) mod model;

#[allow(dead_code)]
pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
