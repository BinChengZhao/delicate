extern crate autocfg;

use autocfg::emit;
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let database = env::var("DATABASE").expect("Without `DATABASE` set in .env");
    match database.as_str() {
        "mysql" | "MYSQL" => emit("DB_MYSQL"),
        "postgres" | "POSTGRES" => emit("DB_POSTGRES"),
        _ => panic!(
            "No reasonable `DATABASE` is set in .env, optional value is ('mysql' or 'postgres')"
        ),
    }

    autocfg::rerun_path("build.rs");
}
