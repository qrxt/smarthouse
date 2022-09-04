use smart_house_http::{
    client::SmartHouseClient,
    device::{DeviceItem, NewDevice, Thermometer},
    house::NewHouse,
    room::NewRoom,
};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reqwest_client = reqwest::Client::new();
    let house_client = SmartHouseClient::new("http://localhost:8000", reqwest_client);

    let new_house = NewHouse {
        name: format!("house_example#{}", Uuid::new_v4()),
    };
    let inserted_house = house_client.add_house(new_house).await?;

    println!("Added new house: {:?}", inserted_house);

    let new_room = NewRoom {
        name: format!("room_example#{}", Uuid::new_v4()),
        device_names: vec![],
    };

    let inserted_room = house_client.add_room(new_room, inserted_house.id).await?;

    println!("Added new room: {:?}", &inserted_room);

    let thermo = Thermometer {
        name: "generated_device#1".to_string(),
        temperature: 7.0,
    };
    let device = NewDevice {
        name: thermo.name.to_string(),
        parent_room: inserted_room.id,
        type_: DeviceItem::Thermometer,
        data: serde_json::to_string(&thermo).unwrap(),
    };

    let inserted_device = house_client.add_device(device).await?;

    println!("Added new device: {:?}", inserted_device);

    let report = house_client.get_report(inserted_house.id).await?;

    println!("House report: {:?}", report);

    Ok(())
}
