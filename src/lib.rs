#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use self::models::{NewItem};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_item<'a>(conn: &SqliteConnection, name: &'a str, location: &'a str, team: Option<&'a str>, amount: Option<i32>, barcode: Option<i32>) -> usize {

    let new_item : NewItem;
    if team.unwrap() == "" {
        new_item = NewItem{ name, location, team: None, amount, barcode};
    } else {
        new_item = NewItem{ name, location, team, amount, barcode};
    }

    diesel::insert_into(schema::items::table)
        .values(&new_item)
        .execute(conn)
        .expect("Error saving object")

}
