use rudis::server::Server;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:6379".parse::<SocketAddr>().unwrap();
    let server = Server::new(addr, 16);
    server.run().await.unwrap();
}
