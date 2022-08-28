use crate::house::HouseError;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Room {
    pub name: String,
    pub device_names: Vec<String>,
}

impl Room {
    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn new(name: &str, device_names: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            device_names,
        }
    }

    pub fn is_device_exist(&self, device_name: &str) -> bool {
        self.device_names.iter().any(|d| d == device_name)
    }

    pub fn add_device(&mut self, device: String) -> Result<(), HouseError> {
        if Self::is_device_exist(self, &device) {
            Err(HouseError::TryingToAddAnExistingDevice(device))
        } else {
            self.device_names.push(device);

            Ok(())
        }
    }

    pub fn add_devices(&mut self, devices: Vec<String>) -> Result<(), HouseError> {
        devices
            .into_iter()
            .try_for_each(|d| Self::add_device(self, d))
    }

    pub fn remove_device(&mut self, device_name: &str) {
        let idx = self
            .device_names
            .iter()
            .position(|d| d == device_name)
            .unwrap();

        self.device_names.remove(idx);
    }
}

#[cfg(test)]
mod test_house {
    use super::*;

    #[test]
    fn test_get_name() {
        let room = Room::new(
            "My room",
            vec!["socket".to_string(), "thermometer".to_string()],
        );

        assert_eq!(room.get_name(), "My room");
    }

    #[test]
    fn test_add_device_success() {
        let mut room = Room::new("My room", Vec::new());

        let result = room.add_device("tv".to_string());

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_add_device_error() {
        let mut room = Room::new("My room", Vec::new());
        let device_name = "tv".to_string();

        let _r = room.add_device(device_name.to_owned());
        let result = room.add_device(device_name.to_owned());

        assert_eq!(
            result,
            Err(HouseError::TryingToAddAnExistingDevice(
                device_name.to_owned()
            ))
        );

        assert_eq!(
            result.unwrap_err().to_string(),
            format!("Device with name \"{}\" already exists", device_name)
        )
    }

    #[test]
    fn test_add_multiple_devices_success() {
        let mut room = Room::new("My room", Vec::new());

        let result = room.add_devices(vec!["tv".to_string(), "thermo".to_string()]);

        assert_eq!(
            room.device_names,
            vec!["tv".to_string(), "thermo".to_string()]
        );
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_add_multiple_devices_error() {
        let mut room = Room::new("My room", Vec::new());
        let device_name = "tv";

        let _r = room.add_device(device_name.to_owned());
        let result = room.add_devices(vec![device_name.to_owned(), "thermo".to_string()]);

        assert_eq!(
            result,
            Err(HouseError::TryingToAddAnExistingDevice(
                device_name.to_owned()
            ))
        );

        assert_eq!(
            result.unwrap_err().to_string(),
            format!(
                "Device with name \"{}\" already exists",
                device_name.to_owned()
            )
        )
    }

    #[test]
    fn test_remove_device() {
        let mut room = Room::new("My room", Vec::new());

        let _r = room.add_device("tv".to_string());
        assert_eq!(room.device_names.len(), 1);

        room.remove_device("tv");

        assert_eq!(room.device_names.len(), 0);
    }
}
