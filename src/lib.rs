#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use self::models::{NewItem, NewLocation, NewTeam, NewUser, NewWarehouse};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_item<'a>(conn: &SqliteConnection, name: &'a str, location_id: &'a i32, team_id: Option<&'a i32>,
                       amount: Option<&'a i32>, barcode: Option<&'a str>) -> usize {
    let new_item = NewItem{ name, location_id, team_id, amount, barcode};
    diesel::insert_into(schema::items::table)
        .values(&new_item)
        .execute(conn)
        .expect("Error saving object")
}

pub fn create_location<'a>(conn: &SqliteConnection, name: &'a str, warehouse_id: &'a i32) -> usize {
    let new_location = NewLocation{ name, warehouse_id};
    diesel::insert_into(schema::locations::table)
        .values(&new_location)
        .execute(conn)
        .expect("Error saving object")
}

pub fn create_team<'a>(conn: &SqliteConnection, name: &'a str, teamlead_id: Option<&'a i32>) -> usize {
    let new_team = NewTeam{ name, teamlead_id};
    diesel::insert_into(schema::teams::table)
        .values(&new_team)
        .execute(conn)
        .expect("Error saving object")
}

pub fn create_user<'a>(conn: &SqliteConnection, name: &'a str, team_id: Option<&'a i32>) -> usize {
    let new_user = NewUser{ name, team_id};
    diesel::insert_into(schema::users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving object")
}

pub fn create_warehouse<'a>(conn: &SqliteConnection, name: &'a str, address: Option<&'a str>,
                            capacity: Option<&'a str>) -> usize {
    let new_warehouse = NewWarehouse{ name, address, capacity};
    diesel::insert_into(schema::warehouses::table)
        .values(&new_warehouse)
        .execute(conn)
        .expect("Error saving object")
}

