mod error;
mod v1;

use axum::{Json, Router, routing::get};

pub fn create_app() -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .nest("/api/v1", v1::router())
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
