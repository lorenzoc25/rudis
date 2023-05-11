use crate::connection::Connection;
use bytes::Bytes;
use httparse::{Request, Status};
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
    use crate::command::Command::{Get, Set};

    let mut connection = Connection::new(socket).await.unwrap();

    loop {
        let buff = connection.read_frame().await.unwrap();
        let s = match str::from_utf8(&buff) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        println!("{}", s);
        let mut headers: [httparse::Header; 16] = [httparse::EMPTY_HEADER; 16];
        let req = parse_request(&buff, &mut headers).unwrap();

        // for header in req.headers {
        //     println!("{}: {}", header.name, String::from_utf8_lossy(header.value));
        // }
        // let response = match Command::from_frame(frame).unwrap() {
        //     Set(cmd) => {
        //         let key = cmd.key().to_string();
        //         let idx = hash_key(&key) % db.len();
        //         let mut db = db[idx].lock().unwrap();

        //         db.insert(key, cmd.value().clone());
        //         Frame::Simple("OK".to_string())
        //     }
        //     Get(cmd) => {
        //         let key = cmd.key().to_string();
        //         let idx = hash_key(&key) % db.len();
        //         let db = db[idx].lock().unwrap();
        //         if let Some(value) = db.get(cmd.key()) {
        //             // `Frame::Bulk` expects data to be of type `Bytes`. This
        //             // type will be covered later in the tutorial. For now,
        //             // `&Vec<u8>` is converted to `Bytes` using `into()`.
        //             Frame::Bulk(value.clone().into())
        //         } else {
        //             Frame::Null
        //         }
        //     }
        //     cmd => panic!("unimplemented {:?}", cmd),
        // };
        let response = "hello world".as_bytes();

        connection.write_frame(&response).await.unwrap();
    }
}

fn parse_request<'a>(
    buff: &'a [u8],
    headers: &'a mut [httparse::Header<'a>; 16],
) -> Option<Request<'a, 'a>> {
    let mut req = Request::new(headers);
    match req.parse(buff) {
        Ok(Status::Complete(_)) => Some(req),
        _ => None,
    }
}
