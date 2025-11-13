use std::sync::Arc;

use crate::{error::ApiError, socket::SocketClient};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, get, post, put},
};
use sindri_core::{
    socket::request::SocketRequest,
    vm::{KernelConfig, VM, VMId},
};

async fn list_vms(
    State(socket_client): State<Arc<SocketClient>>,
) -> Result<Json<String>, ApiError> {
    let response = socket_client.send_message(SocketRequest::ListVMs).await?;
    Ok(Json(response))
}

async fn create_vm(
    State(socket_client): State<Arc<SocketClient>>,
) -> Result<Json<String>, ApiError> {
    let vm = VM::new(
        0,
        "test".to_string(),
        4,
        2048,
        KernelConfig {
            parameters: vec![],
            path: "/path/to/kernel".to_string(),
        },
    );
    let response = socket_client
        .send_message(SocketRequest::CreateVM(vm))
        .await?;
    Ok(Json(response))
}

async fn get_vm(
    State(socket_client): State<Arc<SocketClient>>,
    Path(vm_id): Path<u32>,
) -> Result<Json<String>, ApiError> {
    let vm_id = VMId::from(vm_id);
    let response = socket_client
        .send_message(SocketRequest::GetVM(vm_id))
        .await?;
    Ok(Json(response))
}

async fn update_vm(
    State(socket_client): State<Arc<SocketClient>>,
    Path(vm_id): Path<u32>,
) -> Result<Json<String>, ApiError> {
    let vm_id = VMId::from(vm_id);
    let vm = VM::new(
        0,
        "test".to_string(),
        4,
        2048,
        KernelConfig {
            parameters: vec![],
            path: "/path/to/kernel".to_string(),
        },
    );
    let response = socket_client
        .send_message(SocketRequest::UpdateVM(vm_id, vm))
        .await?;
    Ok(Json(response))
}

async fn destroy_vm(
    State(socket_client): State<Arc<SocketClient>>,
    Path(vm_id): Path<u32>,
) -> Result<Json<String>, ApiError> {
    let vm_id = VMId::from(vm_id);
    let response = socket_client
        .send_message(SocketRequest::DeleteVM(vm_id))
        .await?;
    Ok(Json(response))
}

async fn start_vm(
    State(socket_client): State<Arc<SocketClient>>,
    Path(vm_id): Path<u32>,
) -> Result<Json<String>, ApiError> {
    let vm_id = VMId::from(vm_id);
    let response = socket_client
        .send_message(SocketRequest::StartVM(vm_id))
        .await?;
    Ok(Json(response))
}

async fn stop_vm(
    State(socket_client): State<Arc<SocketClient>>,
    Path(vm_id): Path<u32>,
) -> Result<Json<String>, ApiError> {
    let vm_id = VMId::from(vm_id);
    let response = socket_client
        .send_message(SocketRequest::StopVM(vm_id))
        .await?;
    Ok(Json(response))
}

async fn vm_metrics(
    State(socket_client): State<Arc<SocketClient>>,
    Path(vm_id): Path<u32>,
) -> Result<Json<String>, ApiError> {
    let vm_id = VMId::from(vm_id);
    let response = socket_client
        .send_message(SocketRequest::GetVMMetrics(vm_id))
        .await?;
    Ok(Json(response))
}

pub fn router(socket_client: Arc<SocketClient>) -> Router {
    Router::new()
        .route("/", get(list_vms))
        .route("/", post(create_vm))
        .route("/{id}", put(update_vm))
        .route("/{id}", get(get_vm))
        .route("/{id}", delete(destroy_vm))
        .route("/{id}/start", post(start_vm))
        .route("/{id}/stop", post(stop_vm))
        .route("/{id}/metrics", get(vm_metrics))
        .with_state(socket_client)
}
