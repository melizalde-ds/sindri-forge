use axum::{
    Json, Router,
    extract::Path,
    routing::{delete, get, post, put},
};

async fn list_vms() -> Json<&'static str> {
    Json("List of VMs")
}

async fn create_vm() -> Json<&'static str> {
    Json("Create VM")
}

async fn get_vm(Path(vm_id): Path<String>) -> Json<String> {
    Json(format!("Get VM with ID: {}", vm_id))
}

async fn update_vm(Path(vm_id): Path<String>) -> Json<String> {
    Json(format!("Update VM with ID: {}", vm_id))
}

async fn destroy_vm(Path(vm_id): Path<String>) -> Json<String> {
    Json(format!("Destroy VM with ID: {}", vm_id))
}

async fn start_vm(Path(vm_id): Path<String>) -> Json<String> {
    Json(format!("Start VM with ID: {}", vm_id))
}

async fn stop_vm(Path(vm_id): Path<String>) -> Json<String> {
    Json(format!("Stop VM with ID: {}", vm_id))
}

async fn vm_metrics(Path(vm_id): Path<String>) -> Json<String> {
    Json(format!("Metrics for VM with ID: {}", vm_id))
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
