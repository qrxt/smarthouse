#[derive(Debug, PartialEq)]
pub enum DeviceItem {
    Thermometer(Thermometer),
    Socket(Socket),
}

#[derive(Debug)]
struct House {
    name: String,
    rooms: Vec<Room>,
}

#[derive(Debug)]
struct Room {
    name: String,
    device_names: Vec<String>,
}

impl House {
    fn new(name: &str) -> Self {
        Self {
            rooms: Vec::new(),
            name: name.to_string(),
        }
    }

    fn get_rooms(&self) -> &Vec<Room> {
        &self.rooms
    }

    fn add_room(&mut self, room: Room) {
        self.rooms.push(room)
    }

    fn devices(&self, room_name: &str) -> &Vec<String> {
        let fitting_room = self
            .get_rooms()
            .iter()
            .find(|r| r.name == room_name)
            .unwrap();

        &fitting_room.device_names
    }

    fn create_report_lines<T: DeviceInfoProvider>(&self, provider: &T) -> Vec<String> {
        let mut report = Vec::new();

        report.push(format!("House: {}", self.name));
        report.push("~~~~~".to_string());
        for room in self.get_rooms() {
            report.push(format!("{}:", room.name));

            for device_name in self.devices(&room.name) {
                report.push(provider.get_info(&room.name, device_name))
            }
        }
        report.push("~~~~~".to_string());

        report
    }

    fn _create_report<T: DeviceInfoProvider>(&self, provider: &T) -> String {
        self.create_report_lines(provider).join("\n")
    }
}

trait DeviceInfoProvider {
    fn get_info(&self, room_name: &str, device_name: &str) -> String;
}
trait Device {
    fn get_name(&self) -> String;

    fn get_info(&self) -> String;
}

impl Device for DeviceItem {
    fn get_name(&self) -> String {
        match self {
            DeviceItem::Socket(socket) => socket.get_name(),
            DeviceItem::Thermometer(thermometer) => thermometer.get_name(),
        }
    }

    fn get_info(&self) -> String {
        match self {
            DeviceItem::Socket(socket) => socket.get_info(),
            DeviceItem::Thermometer(thermometer) => thermometer.get_info(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Socket {
    name: String,
    parent_room: String,

    status: bool,
    power_consumption: f32,
}

impl Socket {
    fn new(name: &str, parent_room: &str, status: bool, power_consumption: f32) -> Self {
        Self {
            name: name.to_string(),
            parent_room: parent_room.to_string(),
            status,
            power_consumption,
        }
    }
}

impl Device for Socket {
    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn get_info(&self) -> String {
        let text_status = match &self.status {
            true => "On",
            false => "Off",
        };

        format!(
            "{} is {}. Power consumption is {}",
            self.name, text_status, self.power_consumption
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Thermometer {
    name: String,
    parent_room: String,

    status: bool,
    temperature: f32,
}

impl Thermometer {
    fn new(name: &str, parent_room: &str, status: bool, temperature: f32) -> Self {
        Self {
            name: name.to_string(),
            parent_room: parent_room.to_string(),
            status,
            temperature,
        }
    }
}

impl Device for Thermometer {
    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn get_info(&self) -> String {
        format!("{} temperature is {}", self.name, self.temperature)
    }
}

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
