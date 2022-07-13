use std::sync::Mutex;

use rand::Rng;

#[derive(Debug)]
pub struct Temp(Mutex<f32>);

impl Temp {
    pub fn get_temp(&self) -> f32 {
        let current = *self.0.lock().unwrap();

        current + rand::thread_rng().gen_range(-2.0..2.0)
    }

    pub fn set_temp(&self, val: f32) {
        *self.0.lock().unwrap() = val;
    }
}

impl Default for Temp {
    fn default() -> Self {
        Self(Mutex::new(rand::thread_rng().gen_range(-2.0..28.0)))
    }
}
