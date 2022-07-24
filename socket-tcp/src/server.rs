use std::str::FromStr;
use std::sync::{Arc, Mutex};

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

use crate::{command::Command, socket::Socket};

pub async fn run_server<Cb>(address: &str, cb_after_start: Cb) -> std::io::Result<()>
where
    Cb: Fn(),
{
    let listener = TcpListener::bind(address).await?;

    let socket_default = Socket {
        name: "my socket".to_string(),
        power_consumption: 20.0,
        status: false,
    };
    let smart_socket = Arc::new(Mutex::new(socket_default));

    cb_after_start();

    loop {
        let (stream, _) = listener.accept().await.expect("Can't accept");
        println!("$$$inside loop");

        let smart_socket = smart_socket.clone();
        // tokio::spawn(async move {
        handle_connection(stream, smart_socket).await;
        // });
    }
}

async fn handle_connection(mut stream: TcpStream, socket: Arc<Mutex<Socket>>) {
    let mut response = String::new();
    let mut buf_reader = BufReader::new(&mut stream);

    println!("from handle conn");

    buf_reader
        .read_line(&mut response)
        .await
        .expect("Unable to read");

    let command = Command::from_str(response.trim()).unwrap();

    let message_for_client = match command {
        Command::GetStatus => {
            let socket = socket.lock().unwrap();
            socket.get_status()
        }
        Command::TurnOn => {
            let mut socket = socket.lock().unwrap();

            socket.turn_on();
            socket.get_status()
        }
        Command::TurnOff => {
            let mut socket = socket.lock().unwrap();
            socket.turn_off();
            socket.get_status()
        }
        Command::GetPowerConsumption => {
            let socket = socket.lock().unwrap();
            socket.get_power_consumption()
        }
    };

    stream
        .write_all(message_for_client.as_bytes())
        .await
        .unwrap();
}
