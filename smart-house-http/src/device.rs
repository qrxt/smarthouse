use crate::db_pool;
use crate::schema::devices;
use diesel;
use diesel::QueryDsl;
use diesel::QueryResult;
use diesel::RunQueryDsl;
use diesel_derive_enum::DbEnum;
use rocket::http::Status;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug, DbEnum)]
#[PgType = "device_item"]
#[DieselType = "Device_item"]
pub enum DeviceItem {
    Thermometer,
    Socket,
}

#[derive(Serialize, Deserialize)]
pub struct Thermometer {
    pub name: String,
    pub temperature: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Socket {
    pub name: String,
    pub status: bool,
}

#[derive(Serialize, Deserialize, Identifiable, Queryable, Debug, Insertable)]
#[table_name = "devices"]
pub struct Device {
    pub id: i32,
    pub name: String,
    pub parent_room: i32,
    pub type_: DeviceItem,
    pub data: String,
}

impl Device {
    pub fn get_status(&self, room_name: String) -> String {
        match self.type_ {
            DeviceItem::Thermometer => {
                let thermo: Thermometer = serde_json::from_str(&self.data).unwrap();

                format!(
                    "Thermometer ({}):\nLocation: {}\nCurrent temperature is {}",
                    &thermo.name, room_name, &thermo.temperature
                )
            }
            DeviceItem::Socket => {
                let socket: Socket = serde_json::from_str(&self.data).unwrap();

                format!(
                    "Socket ({}):\nLocation: {}\nStatus is {}",
                    &socket.name, room_name, &socket.status
                )
            }
        }
    }
}

#[get("/<fid>")]
pub fn get(fid: i32, conn: db_pool::DbConn) -> Json<Device> {
    use super::schema::devices::dsl::*;

    Json(
        devices
            .find(fid)
            .first::<Device>(&*conn)
            .expect("Failed to load device"),
    )
}

#[get("/")]
pub fn get_all(conn: db_pool::DbConn) -> QueryResult<Json<Vec<Device>>> {
    use super::schema::devices::dsl::*;

    devices.load::<Device>(&*conn).map(Json)
}

#[derive(Serialize, Deserialize)]
enum DeviceWrapper {
    Socket(Socket),
    Thermometer(Thermometer),
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "devices"]
pub struct NewDevice {
    pub name: String,
    pub parent_room: i32,
    pub type_: DeviceItem,
    pub data: String,
}

#[post("/", data = "<new_device>")]
pub fn create(new_device: Json<NewDevice>, conn: db_pool::DbConn) -> Json<Device> {
    use super::schema::devices::dsl::*;

    let new_device = new_device.0;

    Json(
        diesel::insert_into(devices)
            .values(&new_device)
            .get_result(&*conn)
            .expect("Failed to add new device"),
    )
}

#[delete("/<fid>")]
pub fn delete(fid: i32, conn: db_pool::DbConn) -> Result<(), Status> {
    use super::schema::devices::dsl::*;

    diesel::delete(devices.find(fid))
        .execute(&*conn)
        .expect("Failed to delete device");

    Ok(())
}
