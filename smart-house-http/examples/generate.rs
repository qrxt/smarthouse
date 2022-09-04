use diesel::RunQueryDsl;
use dotenv::dotenv;
use smart_house_http::db_pool::{self, DbConn};
use smart_house_http::device::{Device, DeviceItem, NewDevice, Thermometer};
use smart_house_http::house::{House, HouseRooms, NewHouse, NewHouseRoom};
use smart_house_http::room::{NewRoom, Room};
use smart_house_http::schema::devices::dsl::*;
use smart_house_http::schema::house_rooms::dsl::*;
use smart_house_http::schema::houses::dsl::*;
use smart_house_http::schema::rooms::dsl::*;
use std::env;

fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = db_pool::init_pool(&database_url, 10);
    let dbconn = DbConn(connection.get().unwrap());

    // Add new house
    let house = NewHouse {
        name: "generated_house#1".to_string(),
    };

    let inserted_house: House = diesel::insert_into(houses)
        .values(&house)
        .get_result::<House>(&*dbconn)
        .expect("Failed to add new house");

    println!("Inserted house: {:?}", inserted_house);

    // Add new room
    let room = NewRoom {
        name: "generated_room#1".to_string(),
        device_names: Vec::new(),
    };

    let inserted_room: Room = diesel::insert_into(rooms)
        .values(&room)
        .get_result::<Room>(&*dbconn)
        .expect("Failed to add new room");

    println!("Inserted room: {:?}", inserted_room);

    // Link house with room
    let house_room = NewHouseRoom {
        house_id: inserted_house.id,
        room_id: inserted_room.id,
    };

    diesel::insert_into(house_rooms)
        .values(&house_room)
        .get_result::<HouseRooms>(&*dbconn)
        .expect("Failed to link house with room");

    println!(
        "Room ({}) linked with house ({})",
        inserted_room.id, inserted_house.id
    );

    // Add device
    let thermo = Thermometer {
        name: "generated_device#1".to_string(),
        temperature: 8.0,
    };
    let device = NewDevice {
        name: thermo.name.to_string(),
        parent_room: inserted_room.id,
        type_: DeviceItem::Thermometer,
        data: serde_json::to_string(&thermo).unwrap(),
    };

    let inserted_device = diesel::insert_into(devices)
        .values(&device)
        .get_result::<Device>(&*dbconn)
        .expect("Failed to add new device");

    println!("Inserted device: {:?}", inserted_device);

    Ok(())
}
