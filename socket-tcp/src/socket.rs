#[derive(Clone)]
pub struct Socket {
    pub name: String,
    pub status: bool,
    pub power_consumption: f32,
}

impl Socket {
    pub fn get_status(&self) -> String {
        match &self.status {
            true => format!("{} is on", self.name),
            false => format!("{} is off", self.name),
        }
    }

    pub fn get_power_consumption(&self) -> String {
        format!("Power consumption is {}", self.power_consumption)
    }

    pub fn turn_off(&mut self) {
        self.status = false;
    }

    pub fn turn_on(&mut self) {
        self.status = true;
    }
}
