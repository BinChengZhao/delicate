use diesel::pg::PgConnection;
use diesel::r2d2::{Builder, ConnectionManager, Pool};

pub(crate) use super::prelude::{self, *};

pub(crate) type ConnectionPool = Pool<ConnectionManager<PgConnection>>;

#[allow(dead_code)]
pub(crate) fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[allow(dead_code)]
pub(crate) fn get_connection_pool()->Pool<ConnectionManager<PgConnection>>{

    // Supports user configuration via .env.
    let max_size = env::var("CONNECTION_POOL_MAX_SIZE").map(|s|str::parse::<u32>(&s).unwrap_or(64)).unwrap_or(64);
    let min_idle = env::var("CONNECTION_POOL_MIN_IDLE").map(|s|str::parse::<u32>(&s).unwrap_or(32)).unwrap_or(32);

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager:ConnectionManager<PgConnection> = ConnectionManager::new(database_url);
    Builder::new().max_size(max_size).min_idle(Some(min_idle)).build(manager).expect("Connection pool initialization failed")
}