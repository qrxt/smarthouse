use socket_tcp::client::{Client, ConnectionError};

#[tokio::main]
async fn main() -> Result<(), ConnectionError> {
    let socket_client = Client {
        address: "127.0.0.1:3333".to_string(),
    };

    println!("{:?}", socket_client.turn_on().await?);
    println!("{:?}", socket_client.get_status().await?);
    println!("{:?}", socket_client.turn_off().await?);
    println!("{:?}", socket_client.get_status().await?);

    Ok(())
}
