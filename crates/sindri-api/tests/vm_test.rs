use std::sync::Arc;

use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn test_list_vms() {
    let socket_client = Arc::new(sindri_api::create_socket_client(String::from(
        "/tmp/sindri.sock",
    )));
    let app = sindri_api::create_app(socket_client);

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/api/v1/vms")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("List of VMs"));
}

#[tokio::test]
async fn test_create_vm() {
    let socket_client = Arc::new(sindri_api::create_socket_client(String::from(
        "/tmp/sindri.sock",
    )));
    let app = sindri_api::create_app(socket_client);

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/vms")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Create VM"));
}

#[tokio::test]
async fn test_get_vm() {
    let socket_client = Arc::new(sindri_api::create_socket_client(String::from(
        "/tmp/sindri.sock",
    )));
    let app = sindri_api::create_app(socket_client);

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/api/v1/vms/123")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("123"));
}

#[tokio::test]
async fn test_update_vm() {
    let socket_client = Arc::new(sindri_api::create_socket_client(String::from(
        "/tmp/sindri.sock",
    )));
    let app = sindri_api::create_app(socket_client);

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::PUT)
                .uri("/api/v1/vms/456")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("456"));
}

#[tokio::test]
async fn test_destroy_vm() {
    let socket_client = Arc::new(sindri_api::create_socket_client(String::from(
        "/tmp/sindri.sock",
    )));
    let app = sindri_api::create_app(socket_client);

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::DELETE)
                .uri("/api/v1/vms/789")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("789"));
}

#[tokio::test]
async fn test_start_vm() {
    let socket_client = Arc::new(sindri_api::create_socket_client(String::from(
        "/tmp/sindri.sock",
    )));
    let app = sindri_api::create_app(socket_client);

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/vms/111/start")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("111"));
}

#[tokio::test]
async fn test_stop_vm() {
    let socket_client = Arc::new(sindri_api::create_socket_client(String::from(
        "/tmp/sindri.sock",
    )));
    let app = sindri_api::create_app(socket_client);

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/vms/222/stop")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("222"));
}

#[tokio::test]
async fn test_vm_metrics() {
    let socket_client = Arc::new(sindri_api::create_socket_client(String::from(
        "/tmp/sindri.sock",
    )));
    let app = sindri_api::create_app(socket_client);

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/api/v1/vms/333/metrics")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("333"));
}

#[tokio::test]
async fn test_invalid_route() {
    let socket_client = Arc::new(sindri_api::create_socket_client(String::from(
        "/tmp/sindri.sock",
    )));
    let app = sindri_api::create_app(socket_client);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/invalid")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
