use smart_house_http::{client::SmartHouseClient, house::NewHouse};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reqwest_client = reqwest::Client::new();
    let house_client = SmartHouseClient::new("http://localhost:8000", reqwest_client);

    let houses = house_client.get_houses().await?;

    println!("All houses: {:?}", houses);

    let new_house = NewHouse {
        name: format!("house_example#{}", Uuid::new_v4()),
    };
    let inserted_house = house_client.add_house(new_house).await?;

    println!("Added new house: {:?}", inserted_house);

    let same_house = house_client.get_house(inserted_house.id).await?;

    println!("Same house: {:?}", same_house);

    Ok(())
}
