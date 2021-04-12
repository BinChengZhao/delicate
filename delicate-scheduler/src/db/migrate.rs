use diesel_migrations::{embed_migrations, EmbedMigrations};

// #[cfg(DB_MYSQL)]
use super::mysql::establish_connection;
#[cfg(DB_POSTGRES)]
use super::postgre::establish_connection;

// #[cfg(DB_MYSQL)]
embed_migrations!("../migrations/mysql");

#[cfg(DB_POSTGRES)]
embed_migrations!("../migrations/postgres");

fn init() {
    let connection = establish_connection();

    // This will run the necessary migrations.
    embedded_migrations::run(&connection);

    // By default the output is thrown out. If you want to redirect it to stdout, you
    // should call embedded_migrations::run_with_output.
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout());
}
