pub mod devices;
pub mod house;
pub mod room;

use devices::{
    device::{Device, DeviceInfoProvider},
    socket::Socket,
    thermometer::Thermometer,
};
use house::House;
use room::Room;

struct OwningDeviceInfoProvider {
    socket: Socket,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_info(&self, room_name: &str, device_name: &str) -> String {
        let socket_name = &self.socket.name;
        let is_fitting_device = self.socket.parent_room == room_name && device_name == socket_name;

        match device_name {
            _device_name if is_fitting_device => self.socket.get_info(),
            _ => "N/S".to_string(),
        }
    }
}

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
            (true, _) => self.socket.get_info(),
            (false, true) => self.thermo.get_info(),
            _ => "N/S".to_string(),
        }
    }
}

fn example_owning_device_info_provider() {
    let mut house = House::new("My home [owning]");

    let room = Room {
        name: "Living room".to_string(),
        device_names: vec!["my socket".to_string(), "non-existent device".to_string()],
    };

    house.add_room(room);

    let socket1 = Socket::new("my socket", "Living room", false, 2.0);

    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    let report1 = house.create_report_lines(&info_provider_1);

    for line in report1 {
        println!("{}", line);
    }
}

fn example_borrowing_device_info_provider() {
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

    house.add_room(room1);
    house.add_room(room2);

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

fn main() {
    print!("\x1B[2J\x1B[1;1H");

    example_owning_device_info_provider();
    println!();
    example_borrowing_device_info_provider();
}
