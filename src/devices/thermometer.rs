use super::device::{Device, DeviceConnectionError};

#[derive(Debug, PartialEq)]
pub struct Thermometer {
    pub name: String,
    pub parent_room: String,

    pub status: bool,
    pub temperature: f32,
}

impl Thermometer {
    pub fn new(name: &str, parent_room: &str, status: bool, temperature: f32) -> Self {
        Self {
            name: name.to_string(),
            parent_room: parent_room.to_string(),
            status,
            temperature,
        }
    }
}

impl Device for Thermometer {
    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn get_info(&self) -> Result<String, DeviceConnectionError> {
        let res = format!("{} temperature is {}", self.name, self.temperature);

        Ok(res)
    }
}

#[cfg(test)]
mod test_thermometer {
    use super::*;

    #[test]
    fn test_get_name() {
        let expected_name = "my_thermometer";
        let thermometer = Thermometer::new("my_thermometer", "Kitchen", false, 0.0);

        assert_eq!(thermometer.get_name(), expected_name);
    }

    #[test]
    fn test_get_info() {
        let expected_device_info = "my_thermometer temperature is 26";
        let thermo = Thermometer::new("my_thermometer", "Kitchen", true, 26.0);

        assert_eq!(thermo.get_info().unwrap(), expected_device_info);
    }
}
