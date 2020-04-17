extern crate dotenv;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use std::env;

pub mod error;
pub mod schema;
pub mod user;