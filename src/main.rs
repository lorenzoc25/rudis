use clap::Parser;
use rudis_http::server::Server;
use std::net::SocketAddr;

/// Mini redis server that supports http interface
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of shards to use for the database
    #[arg(short, long, default_value_t = 16)]
    num_shards: usize,

    /// Address of the http redis server
    #[arg(short, long, default_value = "127.0.0.1:6379")]
    addr: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let addr = args.addr.parse::<SocketAddr>().unwrap();
    println!("Rudis listening on {:?}", addr);
    let server = Server::new(addr, args.num_shards);
    server.run().await.unwrap();
}
