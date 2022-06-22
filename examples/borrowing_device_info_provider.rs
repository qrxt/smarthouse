use smart_house::{
    devices::{
        device::{Device, DeviceInfoProvider},
        socket::Socket,
        thermometer::Thermometer,
    },
    house::House,
    room::Room,
};

struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a Socket,
    thermo: &'b Thermometer,
}

impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn get_info(&self, room_name: &str, device_name: &str) -> String {
        if self.socket.parent_room != room_name {
            return "N/S".to_string();
        }

        let is_socket = self.socket.name == device_name;
        let is_thermo = self.thermo.name == device_name;

        match (is_socket, is_thermo) {
            (true, _) => self.socket.get_info().unwrap(),
            (false, true) => self.thermo.get_info().unwrap(),
            _ => "N/S".to_string(),
        }
    }
}

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

    let _r1 = house.add_room(room1);
    let _r2 = house.add_room(room2);

    let socket2 = Socket::new("my socket", "Kitchen", true, 4.0);
    let thermo = Thermometer::new("thermometer", "Kitchen", true, 24.0);

    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };

    let report2 = house.create_report_lines(&info_provider_2);

    for line in report2 {
        println!("{}", line);
    }
}
