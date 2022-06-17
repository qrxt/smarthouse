#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Room {
    pub name: String,
    pub device_names: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RoomError {
    TryingToAddAnExistingDevice,
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

    pub fn add_device(&mut self, device: String) -> Result<(), RoomError> {
        let has_the_same_room = self.device_names.contains(&device);

        match has_the_same_room {
            true => Err(RoomError::TryingToAddAnExistingDevice),
            false => {
                self.device_names.push(device);

                Ok(())
            }
        }
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
}
