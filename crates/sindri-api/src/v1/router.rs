use crate::v1::vms;
use axum::Router;

pub fn router() -> Router {
    Router::new().nest("/vms", vms::router())
}
