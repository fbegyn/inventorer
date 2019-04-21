extern crate actix_web;
extern crate serde;
extern crate diesel;

use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use actix_web::{HttpRequest, Json, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct Warehouse {
    id: Option<i32>,
    name: String,
    address: Option<String>,
    capacity: Option<i32>,
}

// Fetches all warehouses
pub fn get_warehouses() -> Vec<Warehouse> {
    use super::schema::warehouses::dsl::*;
    let conn = super::establish_connection();
    let results = warehouses
        .load::<super::models::Warehouse>(&conn)
        .expect("error fetching warehouses");
    results.into_iter()
        .map(|w| Warehouse{id: Some(w.id), name: w.name, address: w.address, capacity: w.capacity})
        .collect()
}

pub fn handle_warehouse_list(_req: &HttpRequest) -> Result<Json<Vec<Warehouse>>> {
    let warehouses = get_warehouses();
    Ok(Json(warehouses))
}

pub fn handle_warehouse_create(json: Json<Warehouse>) -> Result<String> {
    let conn = super::establish_connection();
    super::create_warehouse(&conn, &(json.name), json.address.as_ref().map(|x| &**x), json.capacity);
    Ok(format!("{:?}", json))
}

pub fn handle_warehouse_update(_req: &HttpRequest) -> Result<Json<Vec<Warehouse>>> {
    let warehouses = get_warehouses();
    Ok(Json(warehouses))
}

pub fn handle_warehouse_delete(req: &HttpRequest) -> Result<String> {
    use super::schema::warehouses::dsl::*;
    let item_id = req.match_info().get("id").expect("warehouse id not found in url")
        .parse::<i32>().unwrap();
    let conn = super::establish_connection();
    let delete = diesel::delete(warehouses.find(item_id))
        .execute(&conn)
        .expect("troubles finding warehouses");
    Ok(format!("deleted {} items", delete))
}

pub fn handle_warehouse_location(_req: &HttpRequest) -> Result<Json<Vec<Warehouse>>> {
    let warehouses = get_warehouses();
    Ok(Json(warehouses))
}

pub fn handle_warehouse_inventory(_req: &HttpRequest) -> Result<Json<Vec<Warehouse>>> {
    use super::schema::warehouses::dsl::*;
    use super::models;
    let conn = super::establish_connection();
    let results = warehouses
        .load::<models::Warehouse>(&conn)
        .expect("error fetching warehouses");
    let ret: Vec<Warehouse> = results
        .into_iter()
        .map(|w| Warehouse{id: Some(w.id), name: w.name, address: w.address, capacity: w.capacity})
        .collect();
    Ok(Json(ret))
}

pub fn handle_warehouse_location_inventory(_req: &HttpRequest) -> Result<Json<Vec<Warehouse>>> {
    use super::schema::warehouses::dsl::*;
    use super::models;
    let conn = super::establish_connection();
    let results = warehouses
        .load::<models::Warehouse>(&conn)
        .expect("error fetching warehouses");
    let ret: Vec<Warehouse> = results
        .into_iter()
        .map(|w| Warehouse{id: Some(w.id), name: w.name, address: w.address, capacity: w.capacity})
        .collect();
    Ok(Json(ret))
}
