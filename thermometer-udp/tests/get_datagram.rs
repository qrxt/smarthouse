use thermometer_udp::thermometer::Thermometer;

#[test]
fn main() {
    let thermo = Thermometer::new("Thermo#1".to_string(), "127.0.0.1:3334".to_string());

    let result = thermo.get_status();

    assert!(result.contains("current temperature:"));
}
