use super::device::{Device, DeviceConnectionError};

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

    pub fn turn_on() -> Result<(), DeviceConnectionError> {
        Ok(())
    }

    pub fn turn_off() -> Result<(), DeviceConnectionError> {
        Ok(())
    }
}

impl Device for Socket {
    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn get_info(&self) -> Result<String, DeviceConnectionError> {
        let text_status = match &self.status {
            true => "On",
            false => "Off",
        };

        let res = format!(
            "{} is {}. Power consumption is {}",
            self.name, text_status, self.power_consumption
        );

        Ok(res)
    }
}

#[cfg(test)]
mod test_socket {
    use super::*;

    #[test]
    fn test_get_name() {
        let expected_name = "my_socket";
        let socket = Socket::new(expected_name, "Living room", false, 0.0);

        assert_eq!(socket.get_name(), expected_name);
    }

    #[test]
    fn test_get_info() {
        let expected_device_info = "my_socket is Off. Power consumption is 0";
        let socket = Socket::new("my_socket", "Living room", false, 0.0);

        assert_eq!(socket.get_info().unwrap(), expected_device_info);
    }
}
