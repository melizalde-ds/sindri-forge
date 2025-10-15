mod socket;

use sindri_daemon::Daemon;
use std::{sync::Arc, time::Duration};
use tokio::signal::unix::{SignalKind, signal};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let daemon = Arc::new(Daemon::new());
    let socket_server = socket::SocketServer::new().await?;

    let vm_state = daemon.clone();
    let state_checker = tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(30));
        loop {
            interval.tick().await;
            println!("Checking VM state...");
        }
    });

    let daemon_state = daemon.clone();
    let socket_task = tokio::spawn(async move { socket_server.run(daemon_state).await });

    shutdown_signal().await;
    println!("Shutting down gracefully...");

    state_checker.abort();
    socket_task.abort();

    Ok(())
}

async fn shutdown_signal() {
    let mut sigterm = signal(SignalKind::terminate()).unwrap();
    let mut sigint = signal(SignalKind::interrupt()).unwrap();

    tokio::select! {
        _ = sigterm.recv() => println!("Received SIGTERM"),
        _ = sigint.recv() => println!("Received SIGINT"),
    }
}
