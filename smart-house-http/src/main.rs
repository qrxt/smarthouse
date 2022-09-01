#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use std::env;

pub mod db_pool;
pub mod device;
pub mod house;
pub mod room;
pub mod schema;
pub mod utils;

fn main() {
    dotenv::dotenv().ok();
    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_url = "postgres://qrx:123@localhost/smart_house"; // TODO! remove! use from .env

    let database_max_size = env::var("DATABASE_MAX_SIZE")
        .unwrap_or_else(|_| String::from("10"))
        .parse::<u32>()
        .unwrap();

    rocket::ignite()
        .manage(db_pool::init_pool(&database_url, database_max_size))
        .mount(
            "/houses",
            routes![
                house::get_all,
                house::get,
                house::create,
                house::delete,
                house::get_all_rooms
            ],
        )
        .mount(
            "/rooms",
            routes![
                room::get,
                room::get_all,
                room::create,
                room::delete,
                room::add_device
            ],
        )
        .mount(
            "/devices",
            routes![device::get, device::get_all, device::create, device::delete,],
        )
        .launch();
}
