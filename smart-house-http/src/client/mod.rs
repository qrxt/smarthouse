use reqwest::Client;

use crate::{
    device::{Device, NewDevice},
    house::{House, HouseReport, NewHouse},
    room::{NewRoom, Room},
};

pub struct SmartHouseClient {
    url: String,
    client: Client,
}

pub type SmartHouseClientResult<T> = Result<T, Box<dyn std::error::Error>>;

impl SmartHouseClient {
    pub fn new(url: &str, client: Client) -> Self {
        Self {
            url: url.to_string(),
            client,
        }
    }

    // House
    pub async fn get_houses(&self) -> SmartHouseClientResult<Vec<House>> {
        let url = format!("{}/houses", &self.url);

        let response = self.client.get(url).send().await?;
        let houses = response.json::<Vec<House>>().await?;

        Ok(houses)
    }

    pub async fn get_house(&self, id: i32) -> SmartHouseClientResult<House> {
        let url = format!("{}/houses/{}", &self.url, id);

        let response = self.client.get(url).send().await?;
        let house = response.json::<House>().await?;

        Ok(house)
    }

    pub async fn add_house(&self, house: NewHouse) -> SmartHouseClientResult<House> {
        let url = format!("{}/houses", &self.url);

        let response = self.client.post(url).json(&house).send().await?;
        let inserted_house = response.json::<House>().await?;

        Ok(inserted_house)
    }

    pub async fn delete_house(&self, id: i32) -> SmartHouseClientResult<()> {
        let url = format!("{}/houses/{}", &self.url, id);

        let response = self.client.delete(url).send().await?;
        response.json::<()>().await?;

        // TODO: delete linked rooms

        Ok(())
    }

    // Rooms
    pub async fn get_rooms(&self) -> SmartHouseClientResult<Vec<Room>> {
        let url = format!("{}/rooms", &self.url);

        let response = self.client.get(url).send().await?;
        let rooms = response.json::<Vec<Room>>().await?;

        Ok(rooms)
    }

    pub async fn get_room(&self, id: i32) -> SmartHouseClientResult<Room> {
        let url = format!("{}/rooms/{}", &self.url, id);

        let response = self.client.get(url).send().await?;
        let room = response.json::<Room>().await?;

        Ok(room)
    }

    pub async fn add_room(&self, room: NewRoom, house_id: i32) -> SmartHouseClientResult<Room> {
        let url = format!("{}/rooms/{}", &self.url, house_id);

        let response = self.client.post(url).json(&room).send().await?;
        let inserted_room = response.json::<Room>().await?;

        Ok(inserted_room)
    }

    pub async fn delete_room(&self, id: i32) -> SmartHouseClientResult<()> {
        let url = format!("{}/rooms/{}", &self.url, id);

        let response = self.client.delete(url).send().await?;
        response.json::<()>().await?;

        Ok(())
    }

    // Devices
    pub async fn get_devices(&self) -> SmartHouseClientResult<Vec<Device>> {
        let url = format!("{}/devices", &self.url);

        let response = self.client.get(url).send().await?;
        let devices = response.json::<Vec<Device>>().await?;

        Ok(devices)
    }

    pub async fn get_device(&self, id: i32) -> SmartHouseClientResult<Device> {
        let url = format!("{}/devices/{}", &self.url, id);

        let response = self.client.get(url).send().await?;
        let device = response.json::<Device>().await?;

        Ok(device)
    }

    pub async fn add_device(&self, device: NewDevice) -> SmartHouseClientResult<Device> {
        let url = format!("{}/devices", &self.url);

        let response = self.client.post(url).json(&device).send().await?;
        let inserted_device = response.json::<Device>().await?;

        Ok(inserted_device)
    }

    pub async fn delete_device(&self, id: i32) -> SmartHouseClientResult<()> {
        let url = format!("{}/devices/{}", &self.url, id);

        let response = self.client.delete(url).send().await?;
        response.json::<()>().await?;

        Ok(())
    }

    //

    pub async fn get_report(&self, house_id: i32) -> SmartHouseClientResult<HouseReport> {
        let url = format!("{}/houses/{}/report", &self.url, house_id);

        let response = self.client.get(url).send().await?;
        let report = response.json::<HouseReport>().await?;

        Ok(report)
    }
}
