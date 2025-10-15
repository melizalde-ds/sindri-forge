use std::sync::Arc;

use crate::{error::ApiError, socket::SocketClient};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, get, post, put},
};

async fn list_vms(
    State(socket_client): State<Arc<SocketClient>>,
) -> Result<Json<String>, ApiError> {
    let response = socket_client.send_message("LIST_VMS").await?;
    Ok(Json(response))
}

async fn create_vm(
    State(socket_client): State<Arc<SocketClient>>,
) -> Result<Json<String>, ApiError> {
    let response = socket_client.send_message("CREATE_VM").await?;
    Ok(Json(response))
}

async fn get_vm(
    State(socket_client): State<Arc<SocketClient>>,
    Path(vm_id): Path<u32>,
) -> Result<Json<String>, ApiError> {
    let response = socket_client
        .send_message(format!("GET_VM {}", vm_id).as_str())
        .await?;
    Ok(Json(response))
}

async fn update_vm(
    State(socket_client): State<Arc<SocketClient>>,
    Path(vm_id): Path<u32>,
) -> Result<Json<String>, ApiError> {
    let response = socket_client
        .send_message(format!("UPDATE_VM {}", vm_id).as_str())
        .await?;
    Ok(Json(response))
}

async fn destroy_vm(
    State(socket_client): State<Arc<SocketClient>>,
    Path(vm_id): Path<u32>,
) -> Result<Json<String>, ApiError> {
    let response = socket_client
        .send_message(format!("DESTROY_VM {}", vm_id).as_str())
        .await?;
    Ok(Json(response))
}

async fn start_vm(
    State(socket_client): State<Arc<SocketClient>>,
    Path(vm_id): Path<u32>,
) -> Result<Json<String>, ApiError> {
    let response = socket_client
        .send_message(format!("START_VM {}", vm_id).as_str())
        .await?;
    Ok(Json(response))
}

async fn stop_vm(
    State(socket_client): State<Arc<SocketClient>>,
    Path(vm_id): Path<u32>,
) -> Result<Json<String>, ApiError> {
    let response = socket_client
        .send_message(format!("STOP_VM {}", vm_id).as_str())
        .await?;
    Ok(Json(response))
}

async fn vm_metrics(
    State(socket_client): State<Arc<SocketClient>>,
    Path(vm_id): Path<u32>,
) -> Result<Json<String>, ApiError> {
    let response = socket_client
        .send_message(format!("VM_METRICS {}", vm_id).as_str())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_list_vms() {
        let result = list_vms().await.unwrap();
        assert_eq!(result.0, "List of VMs");
    }

    #[tokio::test]
    async fn test_create_vm() {
        let result = create_vm().await.unwrap();
        assert_eq!(result.0, "Create VM");
    }

    #[tokio::test]
    async fn test_get_vm() {
        let path = Path(123);
        let result = get_vm(path).await.unwrap();
        assert_eq!(result.0, "Get VM with ID: 123");
    }

    #[tokio::test]
    async fn test_update_vm() {
        let path = Path(456);
        let result = update_vm(path).await.unwrap();
        assert_eq!(result.0, "Update VM with ID: 456");
    }

    #[tokio::test]
    async fn test_destroy_vm() {
        let path = Path(789);
        let result = destroy_vm(path).await.unwrap();
        assert_eq!(result.0, "Destroy VM with ID: 789");
    }

    #[tokio::test]
    async fn test_start_vm() {
        let path = Path(111);
        let result = start_vm(path).await.unwrap();
        assert_eq!(result.0, "Start VM with ID: 111");
    }

    #[tokio::test]
    async fn test_stop_vm() {
        let path = Path(222);
        let result = stop_vm(path).await.unwrap();
        assert_eq!(result.0, "Stop VM with ID: 222");
    }

    #[tokio::test]
    async fn test_vm_metrics() {
        let path = Path(333);
        let result = vm_metrics(path).await.unwrap();
        assert_eq!(result.0, "Metrics for VM with ID: 333");
    }
    #[test]
    fn test_router_created() {
        let socket_client = Arc::new(SocketClient::default());
        let _ = router(socket_client);
    }
}
