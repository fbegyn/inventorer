extern crate actix_web;
extern crate serde;
extern crate diesel;

use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use actix_web::{HttpRequest, Json, Result};

// Struct used to handle the incoming API requests
#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    id: Option<i32>,
    name: String,
    warehouse_id: Option<i32>,
}

// Get all locations in the database
pub fn get_locations() -> Vec<Location> {
    use super::schema::locations::dsl::{locations};
    let conn = super::establish_connection();
    let locs = locations
        .load::<super::models::Location>(&conn)
        .expect("error fetching warehouses");
    locs.into_iter()
        .map(|w| Location{id: Some(w.id), name: w.name, warehouse_id: w.warehouse_id})
        .collect()
}

// Get a location based on the id
pub fn get_location(id: i32) -> Location {
    use super::schema::locations::dsl::{warehouse_id,locations};
    let conn = super::establish_connection();
    let mut locs = locations
        .find(id)
        .limit(1)
        .load::<super::models::Location>(&conn)
        .expect("error fetching warehouses");
    let loc = locs.pop()
        .expect("faild to pop warehouse from serach results, is there one that matches?");
    Location{id: Some(loc.id), name: loc.name, warehouse_id: loc.warehouse_id}
}

// Handle the request to /api/location
pub fn handle_location_list(_req: &HttpRequest) -> Result<Json<Vec<Location>>> {
    let locations = get_locations();
    Ok(Json(locations))
}

// Create a new location
pub fn handle_location_create(json: Json<Location>) -> Result<String> {
    let conn = super::establish_connection();
    super::create_location(&conn, &(json.name), json.warehouse_id);
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
    Ok(format!("deleted {} items", delete))
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

