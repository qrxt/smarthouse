use super::device::Device;

#[derive(Debug, PartialEq)]
pub struct Socket {
    pub name: String,
    pub parent_room: String,

    pub status: bool,
    pub power_consumption: f32,
}

impl Socket {
    pub fn new(name: &str, parent_room: &str, status: bool, power_consumption: f32) -> Self {
        Self {
            name: name.to_string(),
            parent_room: parent_room.to_string(),
            status,
            power_consumption,
        }
    }
}

impl Device for Socket {
    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn get_info(&self) -> String {
        let text_status = match &self.status {
            true => "On",
            false => "Off",
        };

        format!(
            "{} is {}. Power consumption is {}",
            self.name, text_status, self.power_consumption
        )
    }
}
