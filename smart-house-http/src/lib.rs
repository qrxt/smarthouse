#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate dotenv;
#[macro_use]
extern crate diesel;

pub mod client;
pub mod db_pool;
pub mod device;
pub mod house;
pub mod room;
pub mod schema;
pub mod utils;
