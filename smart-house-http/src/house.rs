use crate::db_pool;
use crate::device::Device;
use crate::diesel::RunQueryDsl;
use crate::room::Room;
use crate::schema::{house_rooms, houses};
use diesel::associations::HasTable;
use diesel::result::Error::NotFound;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::QueryResult;
use rocket::http::Status;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Identifiable, Queryable, Debug, Insertable, PartialEq, Eq)]
#[table_name = "houses"]
pub struct House {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Identifiable, Queryable, Debug, Insertable, PartialEq, Eq)]
#[table_name = "house_rooms"]
pub struct HouseRooms {
    pub id: i32,
    pub house_id: i32,
    pub room_id: i32,
}

#[get("/")]
pub fn get_all(conn: db_pool::DbConn) -> QueryResult<Json<Vec<House>>> {
    use super::schema::houses::dsl::*;

    houses.load::<House>(&*conn).map(Json)
}

#[get("/<fid>")]
pub fn get(fid: i32, conn: db_pool::DbConn) -> Json<House> {
    use super::schema::houses::dsl::*;

    Json(
        houses
            .find(fid)
            .first::<House>(&*conn)
            .expect("Error loading house"),
    )
}

#[post("/", data = "<house>")]
pub fn create(house: Json<House>, conn: db_pool::DbConn) -> Result<Json<House>, Status> {
    use super::schema::houses::dsl::*;

    let house = house.0;
    let is_room_exist = houses.find(&house.id).first::<House>(&*conn);

    match is_room_exist {
        Err(NotFound) => Ok(Json(
            diesel::insert_into(houses)
                .values(&house)
                .get_result(&*conn)
                .expect("Failed to add new house"),
        )),
        Ok(_) => Err(Status::Conflict),
        _ => panic!("Failed to add new house"),
    }
}

#[delete("/<fid>")]
pub fn delete(fid: i32, conn: db_pool::DbConn) -> Result<(), Status> {
    use super::schema::houses::dsl::*;

    diesel::delete(houses.find(fid))
        .execute(&*conn)
        .expect("Failed to delete house");

    Ok(())
}

//

#[get("/<fid>/rooms")]
pub fn get_all_rooms(fid: i32, conn: db_pool::DbConn) -> QueryResult<Json<Vec<Room>>> {
    use super::schema::house_rooms::dsl::*;
    use super::schema::houses::columns::id as houses_id;
    use super::schema::houses::dsl::*;
    use super::schema::rooms;

    houses::table()
        .inner_join(house_rooms::table().inner_join(rooms::table))
        .filter(houses_id.eq(fid))
        .select(rooms::all_columns)
        .load::<Room>(&*conn)
        .map(Json)
}

//

#[derive(Serialize, Deserialize)]
pub struct HouseReport {
    house: String,
    report: String,
}

#[get("/<fid>/report")]
pub fn get_report(fid: i32, conn: db_pool::DbConn) -> Result<Json<HouseReport>, Status> {
    use super::schema::devices;
    use super::schema::house_rooms::dsl::*;
    use super::schema::houses;
    use super::schema::houses::columns::id as houses_id;
    use super::schema::rooms;

    let house = houses::table
        .find(fid)
        .first::<House>(&*conn)
        .expect("Failed to get house name");

    let house_devices: Vec<(String, Device)> = houses::table
        .inner_join(house_rooms::table().inner_join(rooms::table.inner_join(devices::table)))
        .filter(houses_id.eq(fid))
        .select((rooms::name, devices::all_columns))
        .load::<(String, Device)>(&*conn)
        .expect("Failed to find house devices");

    let report_data: Vec<String> = house_devices
        .into_iter()
        .map(|(room_name, device)| device.get_status(room_name))
        .collect();

    let report = HouseReport {
        house: house.name,
        report: report_data.join("\n"),
    };

    Ok(Json(report))
}
