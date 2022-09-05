use socket_tcp::{
    client::{Client, ConnectionError},
    server,
};

#[tokio::test]
async fn test_client_server() -> Result<(), ConnectionError> {
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
        assert_eq!(socket_client.turn_on().await?, "on");
        assert_eq!(socket_client.get_power_consumption().await?, "20");
        assert_eq!(socket_client.turn_off().await?, "off");
        assert_eq!(socket_client.get_power_consumption().await?, "0");

        Ok(())
    }

    let handle = tokio::spawn(async move { run_tests(socket_client).await });
    handle.await.unwrap()?;

    Ok(())
}

#[tokio::test]
async fn test_cant_connect_error() -> Result<(), ConnectionError> {
    let socket_client = Client {
        address: "127.0.0.1:3334".to_string(),
    };

    let result = socket_client.turn_on().await.unwrap_err().to_string();

    assert_eq!(
        result,
        "Failed to connect to server: \"Connection refused (os error 111)\""
    );

    Ok(())
}
