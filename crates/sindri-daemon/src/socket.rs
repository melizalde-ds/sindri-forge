use sindri_daemon::Daemon;
use std::sync::Arc;
use tokio::net::UnixListener;

pub struct SocketServer {
    pub listener: UnixListener,
}

impl SocketServer {
    pub async fn new() -> Result<Self, std::io::Error> {
        let listener = UnixListener::bind("/tmp/sindri.sock")?;
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

async fn handle_client(stream: tokio::net::UnixStream, _daemon: Arc<Daemon>) -> anyhow::Result<()> {
    println!("Client connected");
    let mut buffer = [0u8; 1024];
    stream.try_read(&mut buffer)?;
    let message = String::from_utf8_lossy(&buffer);
    println!("Received message: {}", message);
    Ok(())
}
