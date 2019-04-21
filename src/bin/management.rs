extern crate inventorer;
extern crate diesel;
extern crate actix_web;
extern crate serde;
extern crate log;
extern crate env_logger;


use inventorer::{warehouse, location, item};
use log::{info};
use actix_web::{server, App, http};

fn main() {
    info!("Starting inventorer application");
    env_logger::init();
    server::new(|| {
        App::new()
            // warehouse api
            .resource("/api/warehouse/list", |r| r.method(http::Method::GET).f(warehouse::handle_warehouse_list))
            .resource("/api/warehouse/create", |r| r.method(http::Method::POST).with(warehouse::handle_warehouse_create))
            .resource("/api/warehouse/{id}/update", |r| r.method(http::Method::PUT).f(warehouse::handle_warehouse_update))
            .resource("/api/warehouse/{id}/delete", |r| r.method(http::Method::DELETE).f(warehouse::handle_warehouse_delete))
            .resource("/api/warehouse/{id}", |r| r.method(http::Method::GET).f(warehouse::handle_warehouse_location))
            .resource("/api/warehouse/{id}/inventory", |r| r.method(http::Method::GET).f(warehouse::handle_warehouse_inventory))
            .resource("/api/warehouse/{id}/inventory/{loc_id}", |r| r.method(http::Method::GET).f(warehouse::handle_warehouse_location_inventory))

            // location api
            .resource("/api/location/list", |r| r.method(http::Method::GET).f(location::handle_location_list))
            .resource("/api/location/create", |r| r.method(http::Method::POST).with(location::handle_location_create))
            .resource("/api/location/{id}/update", |r| r.method(http::Method::PUT).f(location::handle_location_update))
            .resource("/api/location/{id}/delete", |r| r.method(http::Method::DELETE).f(location::handle_location_delete))
            .resource("/api/location/{id}", |r| r.method(http::Method::GET).f(location::handle_location))
            .resource("/api/location/{id}/inventory", |r| r.method(http::Method::GET).f(location::handle_location_inventory))

            // location api
            .resource("/api/item/list", |r| r.method(http::Method::GET).f(item::handle_item_list))
            .resource("/api/item/create", |r| r.method(http::Method::POST).with(item::handle_item_create))
            .resource("/api/item/{id}/update", |r| r.method(http::Method::PUT).f(item::handle_item_update))
            .resource("/api/item/{id}/delete", |r| r.method(http::Method::DELETE).f(item::handle_item_delete))
        })
        .bind("127.0.0.1:8080")
        .expect("cannot bind to 8080")
        .run();
}
