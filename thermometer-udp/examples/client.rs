use core::time;
use std::thread;

use thermometer_udp::thermometer::Thermometer;

#[tokio::main]
async fn main() {
    let thermo = Thermometer::new("Thermo#1".to_string(), "127.0.0.1:3334".to_string()).await;

    loop {
        println!("{:?}", thermo.get_status());

        thread::sleep(time::Duration::from_secs(2));
    }
}
