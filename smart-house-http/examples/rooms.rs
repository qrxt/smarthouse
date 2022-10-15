use smart_house_http::{client::SmartHouseClient, house::NewHouse, room::NewRoom};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reqwest_client = reqwest::Client::new();
    let house_client = SmartHouseClient::new("http://localhost:8000", reqwest_client);

    let new_house = NewHouse {
        name: format!("house_example#{}", Uuid::new_v4()),
    };
    let house = house_client.add_house(new_house).await?;

    let rooms = house_client.get_rooms().await?;

    println!("All rooms: {:?}", rooms);

    let new_room = NewRoom {
        name: format!("room_example#{}", Uuid::new_v4()),
        device_names: vec![],
    };

    let room = house_client.add_room(new_room, house.id).await?;

    println!("Added new room: {:?}", &room);

    let same_room = house_client.get_room(room.id).await?;

    println!("Same room: {:?}", same_room);

    Ok(())
}
