use super::schema::{items, users, teams, locations, warehouses};

#[derive(Queryable)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub location_id: i32,
    pub team_id: Option<i32>,
    pub amount: Option<i32>,
    pub barcode: Option<String>,
    pub for_rent: bool,
}

#[derive(Queryable)]
pub struct Location {
    pub id: i32,
    pub name: String,
    pub warehouse_id: i32,
}

#[derive(Queryable)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub teamlead_id: Option<i32>,
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub team_id: Option<i32>,
}

#[derive(Queryable)]
pub struct Warehouse {
    pub id: i32,
    pub name: String,
    pub address: Option<String>,
    pub capacity: Option<String>
}

#[derive(Insertable)]
#[table_name="items"]
pub struct NewItem<'a> {
    pub name: &'a str,
    pub location_id: &'a i32,
    pub team_id: Option<&'a i32>,
    pub amount: Option<&'a i32>,
    pub barcode: Option<&'a str>,
}

#[derive(Insertable)]
#[table_name="locations"]
pub struct NewLocation<'a> {
    pub name: &'a str,
    pub warehouse_id: &'a i32,
}

#[derive(Insertable)]
#[table_name="teams"]
pub struct NewTeam<'a> {
    pub name: &'a str,
    pub teamlead_id: Option<&'a i32>,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub team_id: Option<&'a i32>,
}

#[derive(Insertable)]
#[table_name="warehouses"]
pub struct NewWarehouse<'a> {
    pub name: &'a str,
    pub address: Option<&'a str>,
    pub capacity: Option<&'a str>,
}

