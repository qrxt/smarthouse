use crate::db_pool;
use crate::diesel::RunQueryDsl;
use crate::schema::devices;
use diesel;
use diesel::QueryDsl;
use diesel::QueryResult;
use diesel_derive_enum::DbEnum;
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
    name: String,
    temperature: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Socket {
    name: String,
    status: bool,
}

#[derive(Serialize, Deserialize, Identifiable, Queryable, Debug, Insertable)]
#[table_name = "devices"]
pub struct Device {
    pub id: i32,
    pub name: String,
    pub parent_room: String,
    pub type_: DeviceItem,
    pub data: String,
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
    pub parent_room: String,
    pub type_: DeviceItem,
    pub data: String,
}

#[post("/", data = "<new_device>")]
pub fn create(new_device: Json<NewDevice>, conn: db_pool::DbConn) -> Json<Device> {
    use super::schema::devices::dsl::*;

    let new_device = new_device.0;

    let device_item = match &new_device.type_ {
        DeviceItem::Socket => {
            let socket: Result<Socket, serde_json::Error> = serde_json::from_str(&new_device.data);

            DeviceWrapper::Socket(socket.unwrap())
        }
        DeviceItem::Thermometer => {
            let thermo: Result<Thermometer, serde_json::Error> =
                serde_json::from_str(&new_device.data);

            DeviceWrapper::Thermometer(thermo.unwrap())
        }
    };

    let new_data = serde_json::to_string(&device_item).unwrap();

    let new_device = NewDevice {
        data: new_data,
        ..new_device
    };

    Json(
        diesel::insert_into(devices)
            .values(&new_device)
            .get_result(&*conn)
            .expect("Failed to add new device"),
    )
}

#[delete("/<fid>")]
pub fn delete(fid: i32, conn: db_pool::DbConn) {
    use super::schema::devices::dsl::*;

    diesel::delete(devices.find(fid))
        .execute(&*conn)
        .expect("Failed to delete device");
}
