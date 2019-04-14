extern crate inventorer;
extern crate diesel;

use std::io;
use self::inventorer::*;

fn main() {
    loop {
        insert_item();
    }
}

fn insert_item() {
    let connection = establish_connection();

    println!("Item:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("failed to read from stdio");
    let name = name.trim();
    println!("Location:");
    let mut location = String::new();
    io::stdin()
        .read_line(&mut location)
        .expect("failed to read from stdio");
    let location = location.trim();
    println!("Team:");
    let mut team = String::new();
    io::stdin()
        .read_line(&mut team)
        .expect("failed to read from stdio");
    let team = team.trim();
    println!("Amount:");
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("failed to read from stdio");
    let amount = amount
        .trim()
        .parse::<i32>().ok();
    println!("Barcode:");
    let mut barcode = String::new();
    io::stdin()
        .read_line(&mut barcode)
        .expect("failed to read from stdio");
    let barcode = barcode
        .trim()
        .parse::<i32>().ok();

    let _ = create_item(&connection, &name, &location, Some(team), amount, barcode);
    println!("\nStored {} {} ({}) at {}", amount.unwrap_or(0), name, team, location);
}
