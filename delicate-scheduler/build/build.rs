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

    //  Authentication-Model, currently optional value `casbin` （optional feature）.
    env::var("AUTH_MODEL")
        .map(|a| match a.as_str() {
            "casbin" | "CASBIN" => emit("AUTH_CASBIN"),
            _ => (),
        })
        .ok();

    // Detects if it is `APP_DEBUG_MODE`.
    env::var("APP_DEBUG")
        .map(|a| match a.as_str() {
            "true" | "True" | "TRUE" => emit("APP_DEBUG_MODE"),
            _ => (),
        })
        .ok();

    autocfg::rerun_path("build.rs");
}
