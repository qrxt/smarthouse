use smart_house::{house::House, room::Room};

fn main() {
    let mut house = House::new("My home [borrowing]");

    let room1 = Room {
        name: "Kitchen".to_string(),
        device_names: vec![
            "my socket".to_string(),
            "thermometer".to_string(),
            "non-existent device".to_string(),
        ],
    };

    let room2 = Room {
        name: "Hallway".to_string(),
        device_names: vec!["door sensor".to_string()],
    };

    let room3 = Room {
        name: "Living room".to_string(),
        device_names: vec!["tv".to_string(), "thermo".to_string()],
    };

    let rooms = vec![room1, room2];

    println!("--- Adding rooms ---");
    let _r1 = house.add_room(room3);
    let _r2 = house.add_rooms(rooms);

    for room in house.get_rooms() {
        println!("{:?}", room);
    }

    println!();
    println!("--- Trying to add an existing room ---");
    let result = house.add_room(Room {
        name: "Kitchen".to_string(),
        device_names: Vec::new(),
    });

    println!("{:?}", result);

    println!();
    println!("--- Removing the room ---");
    house.remove_room("Kitchen");

    for room in house.get_rooms() {
        println!("{:?}", room);
    }

    println!();
}
