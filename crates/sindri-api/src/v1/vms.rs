use crate::error::ApiError;
use axum::{
    Json, Router,
    extract::Path,
    routing::{delete, get, post, put},
};

async fn list_vms() -> Result<Json<&'static str>, ApiError> {
    Ok(Json("List of VMs"))
}

async fn create_vm() -> Result<Json<&'static str>, ApiError> {
    Ok(Json("Create VM"))
}

async fn get_vm(Path(vm_id): Path<String>) -> Result<Json<String>, ApiError> {
    Ok(Json(format!("Get VM with ID: {}", vm_id)))
}

async fn update_vm(Path(vm_id): Path<String>) -> Result<Json<String>, ApiError> {
    Ok(Json(format!("Update VM with ID: {}", vm_id)))
}

async fn destroy_vm(Path(vm_id): Path<String>) -> Result<Json<String>, ApiError> {
    Ok(Json(format!("Destroy VM with ID: {}", vm_id)))
}

async fn start_vm(Path(vm_id): Path<String>) -> Result<Json<String>, ApiError> {
    Ok(Json(format!("Start VM with ID: {}", vm_id)))
}

async fn stop_vm(Path(vm_id): Path<String>) -> Result<Json<String>, ApiError> {
    Ok(Json(format!("Stop VM with ID: {}", vm_id)))
}

async fn vm_metrics(Path(vm_id): Path<String>) -> Result<Json<String>, ApiError> {
    Ok(Json(format!("Metrics for VM with ID: {}", vm_id)))
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(list_vms))
        .route("/", post(create_vm))
        .route("/{id}", put(update_vm))
        .route("/{id}", get(get_vm))
        .route("/{id}", delete(destroy_vm))
        .route("/{id}/start", post(start_vm))
        .route("/{id}/stop", post(stop_vm))
        .route("/{id}/metrics", get(vm_metrics))
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
        let path = Path("vm-123".to_string());
        let result = get_vm(path).await.unwrap();
        assert_eq!(result.0, "Get VM with ID: vm-123");
    }

    #[tokio::test]
    async fn test_update_vm() {
        let path = Path("vm-456".to_string());
        let result = update_vm(path).await.unwrap();
        assert_eq!(result.0, "Update VM with ID: vm-456");
    }

    #[tokio::test]
    async fn test_destroy_vm() {
        let path = Path("vm-789".to_string());
        let result = destroy_vm(path).await.unwrap();
        assert_eq!(result.0, "Destroy VM with ID: vm-789");
    }

    #[tokio::test]
    async fn test_start_vm() {
        let path = Path("vm-111".to_string());
        let result = start_vm(path).await.unwrap();
        assert_eq!(result.0, "Start VM with ID: vm-111");
    }

    #[tokio::test]
    async fn test_stop_vm() {
        let path = Path("vm-222".to_string());
        let result = stop_vm(path).await.unwrap();
        assert_eq!(result.0, "Stop VM with ID: vm-222");
    }

    #[tokio::test]
    async fn test_vm_metrics() {
        let path = Path("vm-333".to_string());
        let result = vm_metrics(path).await.unwrap();
        assert_eq!(result.0, "Metrics for VM with ID: vm-333");
    }

    #[test]
    fn test_router_created() {
        let _ = router();
    }
}
