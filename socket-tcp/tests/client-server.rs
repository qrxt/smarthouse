// use core::time;
use std::{
    sync::{Arc, Barrier},
    thread,
};

use socket_tcp::{
    client::{Client, ConnectionError},
    server,
};

#[test]
fn main() -> Result<(), ConnectionError> {
    let address = "127.0.0.1:3333".to_string();
    let barrier = Arc::new(Barrier::new(2));

    let c = barrier.clone();
    thread::spawn(move || {
        server::run_server(&address, || {
            println!("Server is listening on {}", address);
            c.wait();
        });
    });

    barrier.wait();
    let socket_client = Client {
        address: "127.0.0.1:3333".to_string(),
    };

    assert_eq!(socket_client.turn_on()?, "my socket is on");
    assert_eq!(socket_client.turn_off()?, "my socket is off");
    assert_eq!(
        socket_client.get_power_consumption()?,
        "Power consumption is 20"
    );

    Ok(())
}
