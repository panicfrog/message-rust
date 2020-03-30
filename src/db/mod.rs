extern crate dotenv;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use std::env;

pub mod error;
pub mod schema;
pub mod user;

pub fn establish_connection() -> MysqlConnection {
    let _ = dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
