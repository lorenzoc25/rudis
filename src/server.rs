use crate::command::Command;
use crate::connection::Connection;
use bytes::Bytes;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::net::SocketAddr;
use std::str;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

type ShardedDb = Arc<Vec<Mutex<HashMap<String, Bytes>>>>;

pub struct Server {
    addr: SocketAddr,
    db: ShardedDb,
}

impl Server {
    pub fn new(addr: SocketAddr, num_shards: usize) -> Self {
        let db = new_sharded_db(num_shards);
        Server { addr, db }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(self.addr).await?;

        loop {
            let (socekt, _) = listener.accept().await?;
            let db = self.db.clone();
            tokio::spawn(async move {
                process(socekt, db).await;
            });
        }
    }
}

fn new_sharded_db(num_shards: usize) -> ShardedDb {
    let mut shards = Vec::with_capacity(num_shards);
    for _ in 0..num_shards {
        shards.push(Mutex::new(HashMap::new()));
    }
    Arc::new(shards)
}

fn hash_key(key: &str) -> usize {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish() as usize
}

async fn process(socket: TcpStream, db: ShardedDb) {
    let mut connection = Connection::new(socket).await.unwrap();

    loop {
        let buff = connection.read_frame().await.unwrap();
        let s = match str::from_utf8(&buff) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        println!("{}", s);

        let response: Bytes = match Command::from_bytes(&buff) {
            Command::Get(cmd) => {
                println!("the command is {:?}, {}", cmd, cmd.key());
                let idx = hash_key(cmd.key()) % db.len();
                let db = db[idx].lock().unwrap();

                if let Some(value) = db.get(cmd.key()) {
                    let value_string = std::str::from_utf8(&value).unwrap();
                    Bytes::from(format!("{{\"{}\":\"{}\"}}", cmd.key(), value_string))
                } else {
                    Bytes::copy_from_slice(b"{}")
                }
            }
            Command::Set(cmd) => {
                println!("trying to set {:?} to {:?}", cmd.key(), cmd.val());
                let idx: usize = hash_key(cmd.key()) % db.len();
                let mut db = db[idx].lock().unwrap();

                db.insert(
                    cmd.key().to_string(),
                    Bytes::copy_from_slice(cmd.val().as_bytes()),
                );

                Bytes::copy_from_slice(b"{\"SET\": \"OK\"}")
            }
            Command::Invalid => Bytes::copy_from_slice(b"invalid command"),
        };

        connection.write_frame(&response).await.unwrap();
    }
}
