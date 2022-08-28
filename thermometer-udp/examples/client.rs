use core::time;
use std::thread;

use thermometer_udp::thermometer::Thermometer;

#[tokio::main]
async fn main() {
    let thermo = Thermometer::new("Thermo#1".to_string(), "127.0.0.1:3334".to_string()).await;

    loop {
        match thermo.as_ref() {
            Ok(thermo) => {
                println!("{:?}", thermo.get_status());
            }
            Err(e) => eprintln!("{}", e),
        }

        thread::sleep(time::Duration::from_secs(2));
    }
}
