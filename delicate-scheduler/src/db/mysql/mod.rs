use diesel::mysql::MysqlConnection;
use diesel::r2d2::{Builder, ConnectionManager, Pool, PooledConnection};

pub(crate) use super::prelude::{self, *};

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

    // Supports user configuration via .env.
    let max_size = env::var("CONNECTION_POOL_MAX_SIZE")
        .map(|s| str::parse::<u32>(&s).unwrap_or(64))
        .unwrap_or(64);
    let min_idle = env::var("CONNECTION_POOL_MIN_IDLE")
        .map(|s| str::parse::<u32>(&s).unwrap_or(32))
        .unwrap_or(32);

    Builder::new()
        .max_size(max_size)
        .min_idle(Some(min_idle))
        .connection_customizer(Box::new(Customizer))
        .build(manager)
        .expect("Connection pool initialization failed")
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
struct Customizer;

// diesel The default is UTC time, here we need to configure it to CST time.
impl CustomizeConnection<MysqlConnection, diesel::r2d2::Error> for Customizer {
    fn on_acquire(&self, conn: &mut MysqlConnection) -> Result<(), diesel::r2d2::Error> {
        conn.execute("SET time_zone = SYSTEM;")
            .map_err(diesel::r2d2::Error::QueryError)?;

        Ok(())
    }
}
