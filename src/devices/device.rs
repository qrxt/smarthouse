use super::{socket::Socket, thermometer::Thermometer};

#[derive(Debug, PartialEq)]
pub enum DeviceItem {
    Thermometer(Thermometer),
    Socket(Socket),
}

pub trait DeviceInfoProvider {
    fn get_info(&self, room_name: &str, device_name: &str) -> String;
}

pub trait Device {
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
