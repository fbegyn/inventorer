extern crate actix_web;
extern crate serde;
extern crate diesel;

use super::schema::{users};
use diesel::prelude::*;
use actix_web::{HttpRequest, Json, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub team_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[table_name="users"]
pub struct NewUser {
    pub name: String,
    pub team_id: Option<i32>,
}

fn insert_user(conn: &SqliteConnection, name: &String, team_id: Option<i32>) -> usize {
    let new_user = NewUser{ name: name.clone(), team_id};
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving object")
}

// Fetches all users
pub fn get_users() -> Vec<User> {
    use super::schema::users::dsl::*;
    let conn = super::establish_connection();
    users
        .load::<User>(&conn)
        .expect("error fetching teamss")
}

pub fn handle_user_list(_req: &HttpRequest) -> Result<Json<Vec<User>>> {
    let teams = get_users();
    Ok(Json(teams))
}

pub fn handle_user_create(json: Json<NewUser>) -> Result<String> {
    let conn = super::establish_connection();
    insert_user(&conn, &(json.name), json.team_id);
    Ok(format!("{:?}", json))
}

pub fn handle_user_update(_req: &HttpRequest) -> Result<Json<Vec<User>>> {
    let users = get_users();
    Ok(Json(users))
}

pub fn handle_user_delete(req: &HttpRequest) -> Result<String> {
    use super::schema::users::dsl::*;
    let item_id = req.match_info().get("id").expect("warehouse id not found in url")
        .parse::<i32>().unwrap();
    let conn = super::establish_connection();
    let delete = diesel::delete(users.find(item_id))
        .execute(&conn)
        .expect("troubles finding users");
    Ok(format!("deleted {} items", delete))
}

pub fn handle_warehouse_location(_req: &HttpRequest) -> Result<Json<Vec<User>>> {
    let users = get_users();
    Ok(Json(users))
}

pub fn handle_warehouse_inventory(_req: &HttpRequest) -> Result<Json<Vec<User>>> {
    use super::schema::teams::dsl::*;
    let conn = super::establish_connection();
    let ret = teams
        .load::<User>(&conn)
        .expect("error fetching teams");
    Ok(Json(ret))
}
