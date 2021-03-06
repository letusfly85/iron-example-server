#![recursion_limit="128"]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;

pub mod models;
pub mod schema;

extern crate r2d2;
extern crate r2d2_diesel;

pub mod middlewares;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

extern crate iron;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    MysqlConnection::establish(&database_url).expect(&format!(
        "Error connecting to {}",
        database_url
    ))
}
