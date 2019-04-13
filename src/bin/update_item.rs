extern crate inventorer;
extern crate diesel;

use self::inventorer::*;
use self::diesel::prelude::*;
use std::env::args;

fn main() {
    use inventorer::schema::items::dsl::{items, for_rent};

    let id = args()
        .nth(1)
        .expect("enable rental requires an item id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = establish_connection();

    let _ = diesel::update(items.find(id))
        .set(for_rent.eq(true))
        .execute(&connection)
        .unwrap_or_else(|_| panic!("Unable to find post {}", id));
}
