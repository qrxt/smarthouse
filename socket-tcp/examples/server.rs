use socket_tcp::server;

fn main() {
    let address = "127.0.0.1:3333";

    server::run_server(address);
}
