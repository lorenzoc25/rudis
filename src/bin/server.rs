use bytes::Bytes;
use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

// type Db = Arc<Mutex<HashMap<String, Bytes>>>;
type ShardedDb = Arc<Vec<Mutex<HashMap<String, Bytes>>>>;

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

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    // let db = Arc::new(Mutex::new(HashMap::new()));

    let db = new_sharded_db(16);

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: ShardedDb) {
    use mini_redis::Command::{self, Get, Set};

    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let key = cmd.key().to_string();
                let idx = hash_key(&key) % db.len();
                let mut db = db[idx].lock().unwrap();
                // The value is stored as `Vec<u8>`
                db.insert(key, cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let key = cmd.key().to_string();
                let idx = hash_key(&key) % db.len();
                let db = db[idx].lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    // `Frame::Bulk` expects data to be of type `Bytes`. This
                    // type will be covered later in the tutorial. For now,
                    // `&Vec<u8>` is converted to `Bytes` using `into()`.
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        connection.write_frame(&response).await.unwrap();
    }
}
