#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate dotenv_codegen;

#[macro_use]
extern crate rocket;
extern crate dotenv;
#[macro_use]
extern crate diesel;

use dotenv::dotenv;
use smart_house_http::db_pool;
use smart_house_http::device;
use smart_house_http::house;
use smart_house_http::room;
use std::env;

fn main() {
    dotenv().ok();
    dotenv::from_filename("../.env").ok();

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
                house::get_all_rooms,
                house::get_report,
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
