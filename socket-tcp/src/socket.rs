#[derive(Clone)]
pub struct Socket {
    pub name: String,
    pub status: bool,
    pub power_consumption: f32,
}

impl Socket {
    pub fn get_status_text(&self) -> String {
        match &self.status {
            true => format!("{} is on", self.name),
            false => format!("{} is off", self.name),
        }
    }

    pub fn get_status(&self) -> String {
        match &self.status {
            true => "on".to_string(),
            false => "off".to_string(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_power_consumption(&self) -> String {
        let res = match self.status {
            true => format!("{}", self.power_consumption),
            false => "0".to_string(),
        };

        println!("{:?}", res);

        res
    }

    pub fn turn_off(&mut self) {
        println!("Socket turned off");

        self.status = false;
    }

    pub fn turn_on(&mut self) {
        println!("Socket turned on");

        self.status = true;
    }
}
