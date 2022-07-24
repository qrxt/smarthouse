use socket_tcp::server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:3333";

    server::run_server(address, || {
        println!("Server is listening on {}", &address);
    })
    .await?;

    Ok(())
}
