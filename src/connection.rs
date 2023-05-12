use tokio::io::{AsyncReadExt, AsyncWriteExt, Result};
use tokio::net::TcpStream;

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub async fn new(stream: TcpStream) -> Result<Connection> {
        Ok(Connection { stream })
    }

    pub async fn read_frame(&mut self) -> Result<Vec<u8>> {
        let mut buf = [0u8; 1024];
        let n = self.stream.read(&mut buf).await?;
        Ok(buf[..n].to_vec())
    }

    pub async fn write_frame(&mut self, data: &[u8]) -> Result<()> {
        println!("server got {:?}", std::str::from_utf8(data).unwrap());
        self.stream
            .write_all(&Connection::make_response(data))
            .await?;
        Ok(())
    }

    fn make_response(data: &[u8]) -> Vec<u8> {
        let mut response = Vec::new();
        response.extend_from_slice(b"HTTP/1.1 200 OK\r\n");
        response.extend_from_slice(b"Content-Type: application/json\r\n");
        response.extend_from_slice(b"Content-Length: ");
        response.extend_from_slice(data.len().to_string().as_bytes());
        response.extend_from_slice(b"\r\n\r\n");
        response.extend_from_slice(data);
        response
    }
}
