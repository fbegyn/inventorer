#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod warehouse;
pub mod location;
pub mod item;
pub mod team;
pub mod user;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

