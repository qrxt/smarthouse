use std::str::from_utf8;

use thiserror::Error;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::command::Command;

pub struct Client {
    pub address: String,
}

#[derive(Debug, Error)]
pub enum ConnectionError {
    #[error("Failed to connect to server: {:?}", .0)]
    CantConnect(String),
    #[error("Failed to read from connection")]
    CantRead,
}

impl Client {
    async fn send_command(&self, command: Command) -> Result<String, ConnectionError> {
        let command_str = match command {
            Command::GetStatus => "get_status",
            Command::TurnOn => "turn_on",
            Command::TurnOff => "turn_off",
            Command::GetPowerConsumption => "get_power_consumption",
        };

        match TcpStream::connect(&self.address).await {
            Ok(mut stream) => {
                let msg = format!("{}\n", command_str);

                stream.write_all(msg.as_bytes()).await.unwrap();

                let mut data = [0; 64];
                match stream.read(&mut data).await {
                    Ok(size) => {
                        let subslice = data.get(0..size).unwrap();
                        let result = from_utf8(subslice).unwrap().to_owned();

                        Ok(result)
                    }
                    _ => Err(ConnectionError::CantRead),
                }
            }
            Err(e) => Err(ConnectionError::CantConnect(e.to_string())),
        }
    }

    pub async fn get_status(&self) -> Result<String, ConnectionError> {
        Self::send_command(self, Command::GetStatus).await
    }

    pub async fn turn_on(&self) -> Result<String, ConnectionError> {
        Self::send_command(self, Command::TurnOn).await
    }

    pub async fn turn_off(&self) -> Result<String, ConnectionError> {
        Self::send_command(self, Command::TurnOff).await
    }

    pub async fn get_power_consumption(&self) -> Result<String, ConnectionError> {
        Self::send_command(self, Command::GetPowerConsumption).await
    }
}
