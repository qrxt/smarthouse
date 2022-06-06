use crate::{devices::device::DeviceInfoProvider, room::Room};

#[derive(Debug)]
pub struct House {
    name: String,
    rooms: Vec<Room>,
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

    pub fn add_room(&mut self, room: Room) {
        self.rooms.push(room)
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
    use super::*;

    #[test]
    fn test_get_rooms() {
        let room_1 = Room::new("Kitchen", Vec::new());
        let room_2 = Room::new("Living room", Vec::new());

        let expected_rooms = vec![room_1.clone(), room_2.clone()];
        let mut house = House::new("My house");
        house.add_room(room_1.clone());
        house.add_room(room_2.clone());

        assert_eq!(house.get_rooms().len(), expected_rooms.len());
        assert_eq!(house.get_rooms()[0], expected_rooms[0]);
        assert_eq!(house.get_rooms()[1], expected_rooms[1]);
    }

    #[test]
    fn test_add_room() {
        let mut house = House::new("My house");

        assert_eq!(house.get_rooms().len(), 0);
        let room_1 = Room::new("Kitchen", Vec::new());

        house.add_room(room_1.clone());

        assert_eq!(house.get_rooms().len(), 1);
        assert_eq!(house.get_rooms()[0], room_1);
    }

    #[test]
    fn test_devices() {
        let mut house = House::new("My house");
        let room = Room::new("Kitchen", vec!["socket".to_string()]);

        house.add_room(room.clone());

        let expected_devices = vec!["socket".to_string()];

        assert_eq!(
            house.devices(&room.get_name()).len(),
            expected_devices.len()
        );
        assert_eq!(house.devices(&room.get_name())[0], expected_devices[0]);
    }
}
