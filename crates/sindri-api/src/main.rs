mod v1;

use axum::{Json, Router, routing::get};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .nest("/api/v1", crate::v1::router());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    let listener = TcpListener::bind(addr).await?;
    let _ = axum::serve(listener, app).await?;
    Ok(())
}

async fn healthz() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "OK"
    }))
}

async fn readyz() -> Json<serde_json::Value> {
    // TODO: Add readiness checks here
    Json(serde_json::json!({
        "status": "OK"
    }))
}
