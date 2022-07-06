use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    str::FromStr,
};

use crate::{command::Command, socket::Socket};

pub fn run_server<Cb>(address: &str, cb_after_start: Cb)
where
    Cb: Fn(),
{
    let listener = TcpListener::bind(address).unwrap();

    let mut socket = Socket {
        name: "my socket".to_string(),
        power_consumption: 20.0,
        status: false,
    };

    cb_after_start();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                handle_connection(stream, &mut socket);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream, socket: &mut Socket) {
    let mut response = String::new();
    let mut buf_reader = BufReader::new(&stream);

    buf_reader.read_line(&mut response).expect("unable to read");
    let command = Command::from_str(response.trim()).unwrap();

    let message_for_client = match command {
        Command::GetStatus => socket.get_status(),
        Command::TurnOn => {
            socket.turn_on();
            socket.get_status()
        }
        Command::TurnOff => {
            socket.turn_off();
            socket.get_status()
        }
        Command::GetPowerConsumption => socket.get_power_consumption(),
    };

    stream.write_all(message_for_client.as_bytes()).unwrap();
}
