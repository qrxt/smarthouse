use std::sync::Arc;
use thiserror::Error;
use tokio::net::UdpSocket;

use crate::temp::Temp;

pub struct Thermometer {
    pub name: String,
    pub address: String,

    temperature: Arc<Temp>,
}

#[derive(Debug, Error)]
pub enum ThermometerError {
    #[error("Failed to connect to server: {:?}", .0)]
    BindError(String),
}

impl Thermometer {
    pub async fn new(name: String, address: String) -> Result<Thermometer, ThermometerError> {
        let temperature = Arc::new(Temp::default());
        let socket = UdpSocket::bind(&address).await;

        let temp_cloned = temperature.clone();

        match socket {
            Ok(socket) => {
                tokio::spawn(async move {
                    loop {
                        let mut buf: [u8; 4] = [0; 4];

                        socket
                            .recv_from(&mut buf)
                            .await
                            .expect("Unable to receive data");

                        let temp = f32::from_be_bytes(buf);

                        temp_cloned.set_temp(temp);
                    }
                });
            }
            Err(e) => return Err(ThermometerError::BindError(e.to_string())),
        }

        Ok(Self {
            name,
            address,
            temperature,
        })
    }

    pub fn get_status(&self) -> String {
        format!(
            "[{}] current temperature: {}",
            self.name,
            &self.temperature.get_temp()
        )
    }
}
