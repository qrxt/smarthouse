#[derive(Debug, PartialEq, Clone)]
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
