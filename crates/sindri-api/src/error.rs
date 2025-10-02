use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use sindri_core::errors::vm::SindriError;

pub struct ApiError {
    status: StatusCode,
    message: String,
}

impl ApiError {
    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
        }
    }
}

impl From<SindriError> for ApiError {
    fn from(err: SindriError) -> Self {
        match err {
            SindriError::VmNotFound(id) => {
                ApiError::new(StatusCode::NOT_FOUND, format!("VM not found: {}", id))
            }
            SindriError::VmAlreadyExists(id) => {
                ApiError::new(StatusCode::CONFLICT, format!("VM already exists: {}", id))
            }
            SindriError::InvalidConfig(details) => ApiError::new(
                StatusCode::BAD_REQUEST,
                format!("Invalid VM configuration: {}", details),
            ),
        }
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        if let Ok(sindri_err) = err.downcast::<SindriError>() {
            return sindri_err.into();
        }

        ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Internal server error".to_string(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "error": self.message,
        }));

        (self.status, body).into_response()
    }
}
