use thiserror::Error;

use crate::{devices::device::DeviceInfoProvider, room::Room};

#[derive(Debug)]
pub struct House {
    name: String,
    rooms: Vec<Room>,
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum HouseError {
    #[error("Room with name {:?} already exists", .0)]
    TryingToAddAnExistingRoom(String),
    #[error("Device with name {:?} already exists", .0)]
    TryingToAddAnExistingDevice(String),
}

impl House {
    pub fn new(name: &str) -> Self {
        Self {
            rooms: Vec::new(),
            name: name.to_string(),
        }
    }

    pub fn get_rooms(&self) -> &Vec<Room> {
        &self.rooms
    }

    pub fn is_room_exist(&self, room_name: &str) -> bool {
        self.rooms.iter().any(|r| r.get_name() == room_name)
    }

    pub fn add_room(&mut self, room: Room) -> Result<(), HouseError> {
        if Self::is_room_exist(self, &room.name) {
            Err(HouseError::TryingToAddAnExistingRoom(room.name))
        } else {
            self.rooms.push(room);

            Ok(())
        }
    }

    pub fn add_rooms(&mut self, rooms: Vec<Room>) -> Result<(), HouseError> {
        rooms.into_iter().try_for_each(|r| Self::add_room(self, r))
    }

    pub fn remove_room(&mut self, room_name: &str) {
        let idx = self
            .get_rooms()
            .iter()
            .position(|r| r.get_name() == room_name)
            .unwrap();

        self.rooms.remove(idx);
    }

    pub fn devices(&self, room_name: &str) -> &Vec<String> {
        let fitting_room = self
            .get_rooms()
            .iter()
            .find(|r| r.name == room_name)
            .unwrap();

        &fitting_room.device_names
    }

    pub fn create_report_lines<T: DeviceInfoProvider>(&self, provider: &T) -> Vec<String> {
        let mut report = Vec::new();

        report.push(format!("House: {}", self.name));
        for room in self.get_rooms() {
            report.push(format!("{}:", room.name));

            for device_name in self.devices(&room.name) {
                report.push(provider.get_info(&room.name, device_name))
            }
        }

        report
    }

    pub fn create_report<T: DeviceInfoProvider>(&self, provider: &T) -> String {
        self.create_report_lines(provider).join("\n")
    }
}

#[cfg(test)]
mod test_house {
    use crate::utils::are_vecs_equal;

    use super::*;

    #[test]
    fn test_get_rooms() {
        let room_1 = Room::new("Kitchen", Vec::new());
        let room_2 = Room::new("Living room", Vec::new());

        let expected_rooms = vec![room_1.clone(), room_2.clone()];
        let mut house = House::new("My house");
        let _r1 = house.add_room(room_1);
        let _r2 = house.add_room(room_2);

        assert_eq!(house.get_rooms().len(), expected_rooms.len());
        assert!(are_vecs_equal(house.get_rooms(), &expected_rooms));
    }

    #[test]
    fn test_add_room_success() {
        let mut house = House::new("My house");

        assert_eq!(house.get_rooms().len(), 0);
        let room_1 = Room::new("Kitchen", Vec::new());

        let result = house.add_room(room_1.clone());

        assert_eq!(result, Ok(()));

        let expected_rooms = vec![room_1];

        assert!(are_vecs_equal(house.get_rooms(), &expected_rooms))
    }

    #[test]
    fn test_add_room_error() -> Result<(), HouseError> {
        let mut house = House::new("My house");

        house.add_room(Room::new("Kitchen", Vec::new()))?;
        let result = house.add_room(Room::new("Kitchen", Vec::new()));

        assert_eq!(
            result,
            Err(HouseError::TryingToAddAnExistingRoom("Kitchen".to_string()))
        );

        assert_eq!(
            result.unwrap_err().to_string(),
            "Room with name \"Kitchen\" already exists",
        );

        Ok(())
    }

    #[test]
    fn test_remove_room() {
        let mut house = House::new("My house");

        let room_1 = Room::new("Kitchen", Vec::new());

        let _r = house.add_room(room_1);

        assert_eq!(house.get_rooms().len(), 1);

        house.remove_room("Kitchen");

        assert_eq!(house.get_rooms().len(), 0);
    }

    #[test]
    fn test_devices() {
        let mut house = House::new("My house");
        let room = Room::new("Kitchen", vec!["socket".to_string()]);

        let _r1 = house.add_room(room.clone());

        let expected_devices = vec!["socket".to_string()];

        assert_eq!(
            house.devices(&room.get_name()).len(),
            expected_devices.len()
        );

        assert!(are_vecs_equal(
            house.devices(&room.get_name()),
            &expected_devices
        ));
    }
}
