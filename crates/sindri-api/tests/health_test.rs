use axum::body::Body;
use axum::http::{Request, StatusCode};
use std::sync::Arc;
use tower::ServiceExt;

#[tokio::test]
async fn test_healthz_endpoint() {
    let socket_client = Arc::new(sindri_api::create_socket_client(String::from(
        "/tmp/sindri.sock",
    )));
    let app = sindri_api::create_app(socket_client);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_readyz_endpoint() {
    let socket_client = Arc::new(sindri_api::create_socket_client(String::from(
        "/tmp/sindri.sock",
    )));
    let app = sindri_api::create_app(socket_client);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/readyz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
