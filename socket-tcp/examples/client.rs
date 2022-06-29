use socket_tcp::client::{Client, ConnectionError};

fn main() -> Result<(), ConnectionError> {
    let address = "127.0.0.1:3333".to_string();

    let socket_client = Client { address };

    println!("{:?}", socket_client.turn_on()?);
    println!("{:?}", socket_client.turn_off()?);
    println!("{:?}", socket_client.get_power_consumption()?);

    Ok(())
}
