use sindri_daemon::Daemon;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::net::UnixListener;

pub struct SocketServer {
    pub listener: UnixListener,
}

impl SocketServer {
    pub async fn new() -> anyhow::Result<Self> {
        let socket_path = "/tmp/sindri.sock";
        if let Err(e) = std::fs::remove_file(socket_path) {
            if e.kind() != std::io::ErrorKind::NotFound {
                eprintln!(
                    "Failed to remove existing socket file {}: {}",
                    socket_path, e
                );
            }
        }
        let listener = UnixListener::bind(socket_path)?;
        Ok(Self { listener })
    }

    pub async fn run(self, daemon: Arc<Daemon>) {
        loop {
            match self.listener.accept().await {
                Ok((stream, _addr)) => {
                    let daemon = daemon.clone();
                    tokio::spawn(async move {
                        if let Err(e) = handle_client(stream, daemon).await {
                            eprintln!("Error handling client: {:?}", e);
                        }
                    });
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {:?}", e);
                }
            }
        }
    }
}

async fn handle_client(
    mut stream: tokio::net::UnixStream,
    _daemon: Arc<Daemon>,
) -> anyhow::Result<()> {
    println!("Client connected");
    let mut buffer = [0u8; 1024];
    let n = stream.read(&mut buffer).await?;
    let message = String::from_utf8_lossy(&buffer[..n]);
    println!("Received message: {}", message);
    Ok(())
}
