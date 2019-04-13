extern crate inventorer;
extern crate diesel;

use self::inventorer::*;
use self::diesel::prelude::*;
use std::env::args;

fn main() {
    use inventorer::schema::items::dsl::{items};

    let id = args()
        .nth(1)
        .expect("remove item requires ID")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = establish_connection();

    let _ = diesel::delete(items.find(id))
        .execute(&connection)
        .unwrap_or_else(|_| panic!("Unable to find item {}", id));
}
