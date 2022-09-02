use core::panic;

use crate::diesel::RunQueryDsl;
use crate::utils::has_duplicates::has_duplicates;
use crate::{db_pool, schema::rooms};
use diesel::result::Error::NotFound;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::QueryResult;
use rocket::http::Status;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Identifiable, Queryable, Debug, Insertable)]
#[table_name = "rooms"]
pub struct Room {
    pub id: i32,
    pub name: String,
    pub device_names: Vec<String>,
}

#[get("/<fid>")]
pub fn get(fid: i32, conn: db_pool::DbConn) -> Json<Room> {
    use super::schema::rooms::dsl::*;

    Json(
        rooms
            .find(fid)
            .first::<Room>(&*conn)
            .expect("Error loading room"),
    )
}

#[get("/")]
pub fn get_all(conn: db_pool::DbConn) -> QueryResult<Json<Vec<Room>>> {
    use super::schema::rooms::dsl::*;

    rooms.load::<Room>(&*conn).map(Json)
}

#[post("/", data = "<room>")]
pub fn create(room: Json<Room>, conn: db_pool::DbConn) -> Result<Json<Room>, Status> {
    use super::schema::rooms::dsl::*;

    let room = room.0;

    if has_duplicates(&room.device_names) {
        return Err(Status::Conflict);
    }

    let is_room_exist = rooms.find(&room.id).first::<Room>(&*conn);
    match is_room_exist {
        Err(NotFound) => Ok(Json(
            diesel::insert_into(rooms)
                .values(&room)
                .get_result(&*conn)
                .expect("Failed to add new room"),
        )),
        Ok(_) => Err(Status::Conflict),
        _ => panic!("Failed to add new room"),
    }
}

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct Device {
    name: String,
}

#[put("/<fid>/devices", data = "<device>")]
pub fn add_device(
    fid: i32,
    device: Json<Device>,
    conn: db_pool::DbConn,
) -> Result<Json<()>, Status> {
    use super::schema::rooms;
    use super::schema::rooms::dsl::*;

    let device = device.0;
    let room = rooms
        .find(fid)
        .first::<Room>(&*conn)
        .expect("Failed to find room");
    let mut room_devices: Vec<String> = room.device_names;

    if room_devices.contains(&device.name) {
        return Err(Status::Conflict);
    }

    room_devices.push(device.name);

    diesel::update(rooms)
        .filter(rooms::id.eq(fid))
        .set(rooms::device_names.eq(room_devices))
        .execute(&*conn)
        .expect("Failed to add new device");

    Ok(Json(()))
}

#[delete("/<fid>")]
pub fn delete(fid: i32, conn: db_pool::DbConn) -> Result<(), Status> {
    use super::schema::rooms::dsl::*;

    diesel::delete(rooms.find(fid))
        .execute(&*conn)
        .expect("Failed to delete room");

    Ok(())
}
