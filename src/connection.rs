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
        println!("server got {:?}", data);
        self.stream.write_all(data).await?;
        Ok(())
    }
}
