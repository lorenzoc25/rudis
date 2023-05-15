use monoio::io::{AsyncReadRent, AsyncWriteRentExt};
use monoio::net::TcpStream;
use std::io::Result;

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Connection { stream }
    }

    pub async fn read_stream(&mut self) -> Result<Vec<u8>> {
        let buf: Vec<u8> = Vec::with_capacity(8 * 1024);
        let (_, buf) = self.stream.read(buf).await;
        Ok(buf)
    }

    pub async fn write_stream(&mut self, data: Vec<u8>) -> Result<()> {
        match self
            .stream
            .write_all(Connection::make_response(&data))
            .await
        {
            (Ok(_), _) => Ok(()),
            (Err(e), _) => Err(e),
        }
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
