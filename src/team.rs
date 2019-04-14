extern crate actix_web;
extern crate serde;
extern crate diesel;

use diesel::prelude::*;
use super::schema::{teams};
use serde::{Serialize, Deserialize};
use actix_web::{HttpRequest, Json, Result};

#[derive(Serialize, Queryable)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub teamlead_id: Option<i32>,
}

#[derive(Deserialize, Insertable, Debug)]
#[table_name="teams"]
pub struct NewTeam {
    pub name: String,
    pub teamlead_id: Option<i32>,
}

fn insert_team(conn: &SqliteConnection, name: &String, teamlead_id: Option<i32>) -> usize {
    let new_team = NewTeam{ name: name.clone(), teamlead_id};
    diesel::insert_into(teams::table)
        .values(&new_team)
        .execute(conn)
        .expect("Error saving object")
}

// Fetches all teams
pub fn get_teams() -> Vec<Team> {
    use super::schema::teams::dsl::*;
    let conn = super::establish_connection();
    teams
        .load::<Team>(&conn)
        .expect("error fetching teamss")
}

pub fn handle_team_list(_req: &HttpRequest) -> Result<Json<Vec<Team>>> {
    let teams = get_teams();
    Ok(Json(teams))
}

pub fn handle_team_create(json: Json<NewTeam>) -> Result<String> {
    let conn = super::establish_connection();
    insert_team(&conn, &(json.name), json.teamlead_id);
    Ok(format!("{:?}", json))
}

pub fn handle_team_update(_req: &HttpRequest) -> Result<Json<Vec<Team>>> {
    let teams = get_teams();
    Ok(Json(teams))
}

pub fn handle_team_delete(req: &HttpRequest) -> Result<String> {
    use super::schema::teams::dsl::*;
    let item_id = req.match_info().get("id").expect("team id not found in url")
        .parse::<i32>().unwrap();
    let conn = super::establish_connection();
    let delete = diesel::delete(teams.find(item_id))
        .execute(&conn)
        .expect("troubles finding teams");
    Ok(format!("deleted {} teams", delete))
}

pub fn handle_warehouse_location(_req: &HttpRequest) -> Result<Json<Vec<Team>>> {
    let teams = get_teams();
    Ok(Json(teams))
}

pub fn handle_warehouse_inventory(_req: &HttpRequest) -> Result<Json<Vec<Team>>> {
    use super::schema::teams::dsl::*;
    let conn = super::establish_connection();
    let ret = teams
        .load::<Team>(&conn)
        .expect("error fetching teams");
    Ok(Json(ret))
}
