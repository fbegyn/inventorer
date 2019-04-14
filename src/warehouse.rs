extern crate actix_web;
extern crate serde;
extern crate diesel;

use diesel::prelude::*;
use super::schema::{warehouses};
use serde::{Serialize, Deserialize};
use actix_web::{HttpRequest, Json, Result};

#[derive(Serialize, Queryable, Debug)]
pub struct Warehouse {
    pub id: i32,
    pub name: String,
    pub address: Option<String>,
    pub capacity: Option<i32>
}
#[derive(Deserialize, Insertable, Debug)]
#[table_name="warehouses"]
pub struct NewWarehouse {
    pub name: String,
    pub address: Option<String>,
    pub capacity: Option<i32>,
}

fn insert_warehouse(conn: &SqliteConnection, name: &String, address: &Option<String>, capacity: Option<i32>) -> usize {
    let new_warehouse = NewWarehouse{ name: name.clone(), address: address.clone(), capacity};
    diesel::insert_into(warehouses::table)
        .values(&new_warehouse)
        .execute(conn)
        .expect("Error saving object")
}

// Fetches all warehouses
pub fn get_warehouses() -> Vec<Warehouse> {
    use super::schema::warehouses::dsl::*;
    let conn = super::establish_connection();
    warehouses
        .load::<Warehouse>(&conn)
        .expect("error fetching warehouses")
}

pub fn handle_warehouse_list(_req: &HttpRequest) -> Result<Json<Vec<Warehouse>>> {
    let warehouses = get_warehouses();
    Ok(Json(warehouses))
}

pub fn handle_warehouse_create(json: Json<NewWarehouse>) -> Result<String> {
    let conn = super::establish_connection();
    insert_warehouse(&conn, &(json.name), &(json.address), json.capacity);
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
    Ok(format!("deleted {} warehouses", delete))
}

pub fn handle_warehouse_location(_req: &HttpRequest) -> Result<Json<Vec<Warehouse>>> {
    let warehouses = get_warehouses();
    Ok(Json(warehouses))
}

pub fn handle_warehouse_inventory(_req: &HttpRequest) -> Result<Json<Vec<Warehouse>>> {
    use super::schema::warehouses::dsl::*;
    let conn = super::establish_connection();
    let results = warehouses
        .load::<Warehouse>(&conn)
        .expect("error fetching warehouses");
    Ok(Json(results))
}

pub fn handle_warehouse_location_inventory(_req: &HttpRequest) -> Result<Json<Vec<Warehouse>>> {
    use super::schema::warehouses::dsl::*;
    let conn = super::establish_connection();
    let results = warehouses
        .load::<Warehouse>(&conn)
        .expect("error fetching warehouses");
    Ok(Json(results))
}
