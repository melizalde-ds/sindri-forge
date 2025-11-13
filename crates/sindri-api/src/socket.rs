use sindri_core::socket::request::SocketRequest;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct SocketClient {
    pub path: String,
}

impl Default for SocketClient {
    fn default() -> Self {
        SocketClient {
            path: String::from("/tmp/sindri.sock"),
        }
    }
}

impl SocketClient {
    pub fn new(path: String) -> Self {
        SocketClient { path }
    }
    pub async fn send_message(&self, command: SocketRequest) -> anyhow::Result<String> {
        let mut stream = tokio::net::UnixStream::connect(&self.path).await?;
        let message = serde_json::to_string(&command)?;

        println!("Sending message: {}", message);

        stream.write_all(message.as_bytes()).await?;
        stream.write_all(b"\n").await?;

        let mut buffer = vec![0u8; 4096];
        let n = stream.read(&mut buffer).await?;

        let response = String::from_utf8_lossy(&buffer[..n]).to_string();

        Ok(response)
    }
}
