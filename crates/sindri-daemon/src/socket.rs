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
        todo!("Implement socket server logic here");
    }
}
