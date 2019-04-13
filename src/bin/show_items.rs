extern crate inventorer;
extern crate diesel;

use self::inventorer::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use inventorer::schema::items::dsl::*;

    let connection = establish_connection();
    let results = items
        .limit(5)
        .load::<Item>(&connection)
        .expect("Error loading items");

    println!("Displaying {} items", results.len());
    for item in results {
        println!("ID: {}\nItem: {}\nLocation: {}\nTeam: {}\nAmount: {}\nBarcode: {}\n", item.id,
                 item.name, item.location, item.team,
                 item.amount.unwrap_or(-1), item.barcode.unwrap_or(-1));
    }
}
