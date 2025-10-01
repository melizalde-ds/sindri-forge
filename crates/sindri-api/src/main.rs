use axum::{Router, routing::get};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(async || "OK"))
        .route("/hello-world", get(hello_world));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    let listener = TcpListener::bind(addr).await?;
    let _ = axum::serve(listener, app).await?;
    Ok(())
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}
