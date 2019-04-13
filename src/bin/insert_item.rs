extern crate inventorer;
extern crate diesel;

use std::io;
use self::inventorer::*;

fn main() {
    let connection = establish_connection();

    println!("Which item do you want to add?");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("failed to read from stdio");
    let name = name.trim();
    println!("Where is {} stored?", name);
    let mut location = String::new();
    io::stdin()
        .read_line(&mut location)
        .expect("failed to read from stdio");
    let location = location.trim();
    println!("To which team does {} belong?", name);
    let mut team = String::new();
    io::stdin()
        .read_line(&mut team)
        .expect("failed to read from stdio");
    let team = team.trim();
    println!("How many {} are there?", name);
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("failed to read from stdio");
    let amount = amount
        .trim()
        .parse::<i32>().ok();
    println!("Barcode of {}?", name);
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
