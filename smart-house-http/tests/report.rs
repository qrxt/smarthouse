use diesel::RunQueryDsl;
use rocket::local::Client;
use rocket::routes;
use smart_house_http::db_pool;
use smart_house_http::device::Device;
use smart_house_http::device::DeviceItem;
use smart_house_http::device::NewDevice;
use smart_house_http::device::Thermometer;
use smart_house_http::house as house_routes;
use smart_house_http::house::House;
use smart_house_http::house::HouseReport;
use smart_house_http::house::HouseRooms;
use smart_house_http::house::NewHouse;
use smart_house_http::house::NewHouseRoom;
use smart_house_http::room::NewRoom;
use smart_house_http::room::Room;
use smart_house_http::schema::devices::dsl::*;
use smart_house_http::schema::house_rooms::dsl::*;
use smart_house_http::schema::houses::dsl::*;
use smart_house_http::schema::rooms::dsl::*;
use uuid::Uuid;

#[tokio::test]
async fn get_report() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "postgres://qrx:123@localhost/smart_house"; // TODO! remove! use from .env
    let database_max_size = 10;
    let _conn = db_pool::init_pool(database_url, database_max_size)
        .get()
        .unwrap();

    let conn = db_pool::init_pool(database_url, database_max_size)
        .get()
        .unwrap();
    let rocket = rocket::ignite()
        .manage(db_pool::init_pool(database_url, database_max_size))
        .mount("/houses", routes![house_routes::get_report]);
    let client = Client::new(rocket).expect("valid rocket instance");

    let new_house_id = Uuid::new_v4();
    let house: House = diesel::insert_into(houses)
        .values(&NewHouse {
            name: new_house_id.to_string(),
        })
        .get_result(&*conn)
        .expect("Failed to create new house");

    let new_room_id = Uuid::new_v4();
    let room: Room = diesel::insert_into(rooms)
        .values(&NewRoom {
            name: new_room_id.clone().to_string(),
            device_names: vec![],
        })
        .get_result(&*conn)
        .expect("Failed to create new room");

    let _link: HouseRooms = diesel::insert_into(house_rooms)
        .values(&NewHouseRoom {
            house_id: house.id,
            room_id: room.id,
        })
        .get_result(&*conn)
        .expect("Failed to link house with room");

    let thermo = Thermometer {
        name: "generated_device#1".to_string(),
        temperature: 7.0,
    };
    let thermo_data = serde_json::to_string(&thermo).unwrap();
    let new_device = NewDevice {
        name: thermo.name,
        parent_room: room.id,
        type_: DeviceItem::Thermometer,
        data: thermo_data,
    };

    let _device: Device = diesel::insert_into(devices)
        .values(&new_device)
        .get_result(&*conn)
        .expect("Failed to add new device");

    let path = format!("/houses/{}/report", house.id);
    let req = client.get(&path);
    let mut response = req.dispatch();

    let house_report: HouseReport = serde_json::from_str(&response.body_string().unwrap()).unwrap();

    let expected_house_report = format!(
        "Thermometer (generated_device#1):\nLocation: {}\nCurrent temperature is 7",
        new_room_id.clone()
    );

    assert_eq!(house_report.report, expected_house_report);

    Ok(())
}
