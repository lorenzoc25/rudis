use rudis::server::Server;
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let addr = if let Some(addr) = args.get(1) {
        addr.clone()
    } else {
        "127.0.0.1:6379".to_string()
    }
    .parse::<SocketAddr>()
    .unwrap();
    println!("Rudis listening on {:?}", addr);
    let server = Server::new(addr, 16);
    server.run().await.unwrap();
}
