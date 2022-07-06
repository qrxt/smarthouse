use smart_house::room::Room;

fn main() {
    let mut room = Room {
        name: "Kitchen".to_string(),
        device_names: vec![],
    };

    println!("--- Adding devices ---");
    let _r1 = room.add_device("my socket".to_string());
    let _r2 = room.add_devices(vec!["thermometer".to_string(), "tv".to_string()]);

    for device in &room.device_names {
        println!("{:?}", device);
    }

    println!();
    println!("--- Trying to add an existing device ---");
    let result = room.add_device("thermometer".to_string());

    println!("{:?}", result);

    println!();
    println!("--- Removing the device ---");
    room.remove_device("tv");

    for device in &room.device_names {
        println!("{:?}", device);
    }

    println!();
}
