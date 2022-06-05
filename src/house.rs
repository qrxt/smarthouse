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
        report.push("~~~~~".to_string());
        for room in self.get_rooms() {
            report.push(format!("{}:", room.name));

            for device_name in self.devices(&room.name) {
                report.push(provider.get_info(&room.name, device_name))
            }
        }
        report.push("~~~~~".to_string());

        report
    }

    pub fn create_report<T: DeviceInfoProvider>(&self, provider: &T) -> String {
        self.create_report_lines(provider).join("\n")
    }
}
