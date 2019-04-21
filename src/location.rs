extern crate actix_web;
extern crate serde;
extern crate diesel;

use super::schema::{locations};
use diesel::prelude::*;
use actix_web::{HttpRequest, Json, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Queryable, Debug)]
pub struct Location {
    pub id: i32,
    pub name: String,
    pub warehouse_id: Option<i32>,
}
#[derive(Deserialize, Insertable, Debug)]
#[table_name="locations"]
pub struct NewLocation {
    pub name: String,
    pub warehouse_id: Option<i32>,
}

fn insert_location(conn: &SqliteConnection, name: &String, warehouse_id: Option<i32>) -> usize {
    let new_location = NewLocation{ name: name.clone(), warehouse_id};
    diesel::insert_into(locations::table)
        .values(&new_location)
        .execute(conn)
        .expect("Error saving object")
}

// Get all locations in the database
pub fn get_locations() -> Vec<Location> {
    use super::schema::locations::dsl::{locations};
    let conn = super::establish_connection();
    locations
        .load::<Location>(&conn)
        .expect("error fetching location")
}

// Get a location based on the id
pub fn get_location(id: i32) -> Location {
    use super::schema::locations::dsl::{locations};
    let conn = super::establish_connection();
    let mut locs = locations
        .find(id)
        .limit(1)
        .load::<Location>(&conn)
        .expect("error fetching location");
    locs.pop().unwrap()
}

// Handle the request to /api/location
pub fn handle_location_list(_req: &HttpRequest) -> Result<Json<Vec<Location>>> {
    let locations = get_locations();
    Ok(Json(locations))
}

// Create a new location
pub fn handle_location_create(json: Json<NewLocation>) -> Result<String> {
    let conn = super::establish_connection();
    insert_location(&conn, &(json.name), json.warehouse_id);
    Ok(format!("{:?}", json))
}

pub fn handle_location_update(req: &HttpRequest) -> Result<Json<Vec<super::item::Item>>>{
    let loc_id = req.match_info().get("id").expect("location id not found in url")
        .parse::<i32>().unwrap();
    let items = super::item::get_location_items(loc_id);
    Ok(Json(items))
}

pub fn handle_location_delete(req: &HttpRequest) -> Result<String> {
    use super::schema::locations::dsl::*;
    let item_id = req.match_info().get("id").expect("location id not found in url")
        .parse::<i32>().unwrap();
    let conn = super::establish_connection();
    let delete = diesel::delete(locations.find(item_id))
        .execute(&conn)
        .expect("troubles finding locations");
    Ok(format!("deleted {} locations", delete))
}

// Get all the items on the location
pub fn handle_location(req: &HttpRequest) -> Result<Json<Location>>{
    let loc_id = req.match_info().get("id").expect("location id not found in url")
        .parse::<i32>().unwrap();
    Ok(Json(get_location(loc_id)))
}

// Get all the items on the location
pub fn handle_location_inventory(req: &HttpRequest) -> Result<Json<Vec<super::item::Item>>>{
    let loc_id = req.match_info().get("id").expect("location id not found in url")
        .parse::<i32>().unwrap();
    Ok(Json(super::item::get_location_items(loc_id)))
}

