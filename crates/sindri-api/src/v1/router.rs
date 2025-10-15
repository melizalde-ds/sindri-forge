use std::sync::Arc;

use crate::{socket::SocketClient, v1::vms};
use axum::Router;

pub fn router(socket_client: Arc<SocketClient>) -> Router {
    Router::new().nest("/vms", vms::router(socket_client))
}
