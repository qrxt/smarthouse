use socket_tcp::{
    client::{Client, ConnectionError},
    server,
};

#[tokio::test]
async fn main() -> Result<(), ConnectionError> {
    let address = "127.0.0.1:3333".to_string();

    let socket_client = Client {
        address: "127.0.0.1:3333".to_string(),
    };

    tokio::spawn(async move {
        server::run_server(&address, || {
            println!("Server started on {}", &address);
        })
        .await
        .unwrap();
    });

    async fn run_tests(socket_client: Client) -> Result<(), ConnectionError> {
        assert_eq!(socket_client.turn_on().await?, "my socket is on");
        assert_eq!(socket_client.turn_off().await?, "my socket is off");
        assert_eq!(
            socket_client.get_power_consumption().await?,
            "Power consumption is 20"
        );

        Ok(())
    }

    let handle = tokio::spawn(async move { run_tests(socket_client).await });
    handle.await.unwrap()?;

    Ok(())
}
