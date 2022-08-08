use thermometer_udp::thermometer::Thermometer;

#[tokio::test]
async fn main() {
    let thermo = Thermometer::new("Thermo#1".to_string(), "127.0.0.1:3334".to_string()).await;

    let result = match thermo.as_ref() {
        Ok(thermo) => thermo.get_status(),
        Err(e) => e.to_string(),
    };

    assert!(result.contains("current temperature:"));
}
