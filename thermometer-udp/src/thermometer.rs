use std::{net::UdpSocket, sync::Arc, thread};

use crate::temp::Temp;

pub struct Thermometer {
    pub name: String,
    pub address: String,

    temperature: Arc<Temp>,
}

impl Thermometer {
    pub fn new(name: String, address: String) -> Thermometer {
        let temperature = Arc::new(Temp::default());
        let socket = UdpSocket::bind(&address).expect("Couldn't bind address");

        let temp_cloned = temperature.clone();

        thread::spawn(move || -> std::io::Result<()> {
            loop {
                let mut buf: [u8; 4] = [0; 4];

                socket.recv_from(&mut buf)?;
                let temp = f32::from_be_bytes(buf);

                temp_cloned.set_temp(temp);
            }
        });

        Self {
            name,
            address,
            temperature,
        }
    }

    pub fn get_status(&self) -> String {
        format!(
            "[{}] current temperature: {}",
            self.name,
            &self.temperature.get_temp()
        )
    }
}
