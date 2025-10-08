use std::{sync::Arc, time::Duration};

use sindri_daemon::Daemon;
use tokio::{sync::RwLock, time::sleep};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let daemon = Arc::new(RwLock::new(Daemon::new()));
    println!("Sindri Daemon started. Listening for commands...");
    println!("{:?}", daemon);

    let unix_daemon = daemon.clone();
    let unix_listener = tokio::spawn(async move {
        println!("Starting Unix socket listener...");
        loop {
            println!("Daemon state:");
            println!("{:?}", unix_daemon.read().await);
            drop(unix_daemon.read().await);
            sleep(Duration::from_secs(5)).await;
        }
    });

    let health_daemon = daemon.clone();
    let health_listener = tokio::spawn(async move {
        println!("Starting health check endpoint...");
        loop {
            println!("Health check - Daemon state:");
            println!("{:?}", health_daemon.read().await);
            drop(health_daemon.read().await);
            sleep(Duration::from_secs(10)).await;
        }
    });

    tokio::select! {
        _ = unix_listener => {}
        _ = health_listener => {}
        _ = tokio::signal::ctrl_c() => {
            println!("Received Ctrl+C, shutting down...");
        }
    }

    Ok(())
}
