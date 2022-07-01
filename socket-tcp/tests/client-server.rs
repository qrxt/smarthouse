use core::time;
use std::thread;

use socket_tcp::{
    client::{Client, ConnectionError},
    server,
};

#[test]
fn main() -> Result<(), ConnectionError> {
    let address = "127.0.0.1:3333".to_string();

    thread::spawn(move || {
        server::run_server(&address);
    });

    let socket_client = Client {
        address: "127.0.0.1:3333".to_string(),
    };

    thread::sleep(time::Duration::from_millis(200));

    assert_eq!(socket_client.turn_on()?, "my socket is on");
    assert_eq!(socket_client.turn_off()?, "my socket is off");
    assert_eq!(
        socket_client.get_power_consumption()?,
        "Power consumption is 20"
    );

    Ok(())
}
