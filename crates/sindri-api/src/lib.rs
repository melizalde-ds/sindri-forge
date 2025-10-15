mod error;
mod socket;
mod v1;

use socket::SocketClient;
use std::sync::Arc;

use axum::{Json, Router, routing::get};

pub fn create_app(socket_client: Arc<SocketClient>) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .nest("/api/v1", v1::router(socket_client))
}

pub fn create_socket_client(path: String) -> socket::SocketClient {
    socket::SocketClient::new(path)
}

async fn healthz() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "OK"
    }))
}

async fn readyz() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "OK"
    }))
}
