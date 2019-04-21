extern crate actix_web;
extern crate serde;
extern crate diesel;

use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use actix_web::{HttpRequest, Json, Result};

// Struct used to handle the incoming API requests
#[derive(Serialize, Debug)]
pub struct Item {
    id: i32,
    name: String,
    location_id: i32,
    team_id: Option<i32>,
    amount: Option<i32>,
    barcode: Option<String>,
    for_rent: bool,
}

// Struct used to handle the incoming API requests
#[derive(Deserialize, Debug)]
pub struct NewItem {
    name: String,
    location_id: i32,
    team_id: Option<i32>,
    amount: Option<i32>,
    barcode: Option<String>,
}

// Fetches all warehouses
pub fn get_items() -> Vec<Item> {
    use super::schema::items::dsl::*;
    let conn = super::establish_connection();
    let results = items
        .load::<super::models::Item>(&conn)
        .expect("error fetching warehouses");
    results.into_iter()
        .map(|w| Item{id: w.id, name: w.name, location_id: w.location_id, team_id: w.team_id, amount: w.amount, barcode: w.barcode, for_rent: w.for_rent})
        .collect()
}

pub fn get_location_items(loc_id: i32) -> Vec<Item> {
    use super::schema::items::dsl::*;
    let conn = super::establish_connection();
    let results = items
        .filter(location_id.eq(loc_id))
        .load::<super::models::Item>(&conn)
        .expect("error fetching warehouses");
    results.into_iter()
        .map(|w| Item{id: w.id, name: w.name, location_id: w.location_id, team_id: w.team_id, amount: w.amount, barcode: w.barcode, for_rent: w.for_rent})
        .collect()
}

pub fn handle_item_list(_req: &HttpRequest) -> Result<Json<Vec<Item>>> {
    let items = get_items();
    Ok(Json(items))
}

// Create a new item
pub fn handle_item_create(json: Json<NewItem>) -> Result<String> {
    let conn = super::establish_connection();
    super::create_item(&conn, &(json.name), json.location_id, json.team_id, json.amount, json.barcode.as_ref().map(|x| &**x));
    Ok(format!("{:?}", json))
}

pub fn handle_item_update(_req: &HttpRequest) -> Result<Json<Vec<Item>>> {
    let items = get_items();
    Ok(Json(items))
}

pub fn handle_item_delete(req: &HttpRequest) -> Result<String> {
    use super::schema::items::dsl::*;
    let item_id = req.match_info().get("id").expect("location id not found in url")
        .parse::<i32>().unwrap();
    let conn = super::establish_connection();
    let delete = diesel::delete(items.find(item_id))
        .execute(&conn)
        .expect("troubles finding items");
    Ok(format!("deleted {} items", delete))
}
