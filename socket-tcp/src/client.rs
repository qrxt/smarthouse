use std::{
    io::{Read, Write},
    net::TcpStream,
    str::from_utf8,
};

use crate::command::Command;

pub struct Client {
    pub address: String,
}

#[derive(Debug)]
pub enum ConnectionError {
    CantConnect,
}

impl Client {
    fn send_command(&self, command: Command) -> Result<String, ConnectionError> {
        let command_str = match command {
            Command::GetStatus => "get_status",
            Command::TurnOn => "turn_on",
            Command::TurnOff => "turn_off",
            Command::GetPowerConsumption => "get_power_consumption",
        };

        match TcpStream::connect(&self.address) {
            Ok(mut stream) => {
                let msg = format!("{}\n", command_str);

                stream.write_all(msg.as_bytes()).unwrap();

                let mut data = [0; 64];
                match stream.read(&mut data) {
                    Ok(size) => Ok(from_utf8(&data)
                        .unwrap()
                        .chars()
                        .take(size)
                        .collect::<String>()),
                    _ => Err(ConnectionError::CantConnect),
                }
            }
            _ => panic!(),
        }
    }

    pub fn get_status(&self) -> Result<String, ConnectionError> {
        Self::send_command(self, Command::GetStatus)
    }

    pub fn turn_on(&self) -> Result<String, ConnectionError> {
        Self::send_command(self, Command::TurnOn)
    }

    pub fn turn_off(&self) -> Result<String, ConnectionError> {
        Self::send_command(self, Command::TurnOff)
    }

    pub fn get_power_consumption(&self) -> Result<String, ConnectionError> {
        Self::send_command(self, Command::GetPowerConsumption)
    }
}
