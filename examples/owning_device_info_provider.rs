use smart_house::{
    devices::{
        device::{Device, DeviceInfoProvider},
        socket::Socket,
    },
    house::House,
    room::Room,
};

struct OwningDeviceInfoProvider {
    socket: Socket,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_info(&self, room_name: &str, device_name: &str) -> String {
        let socket_name = &self.socket.name;
        let is_fitting_device = self.socket.parent_room == room_name && device_name == socket_name;

        match device_name {
            _device_name if is_fitting_device => self.socket.get_info().unwrap(),
            _ => "N/S".to_string(),
        }
    }
}

fn main() {
    let mut house = House::new("My home [owning]");

    let room = Room {
        name: "Living room".to_string(),
        device_names: vec!["my socket".to_string(), "non-existent device".to_string()],
    };

    let _r1 = house.add_room(room);

    let socket1 = Socket::new("my socket", "Living room", false, 2.0);

    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    let report1 = house.create_report_lines(&info_provider_1);

    for line in report1 {
        println!("{}", line);
    }
}
