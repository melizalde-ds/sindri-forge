use anyhow::anyhow;
use axum::{
    Json,
    extract::rejection::{ExtensionRejection, JsonRejection, PathRejection, QueryRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use sindri_core::errors::vm::VMError;

#[derive(Debug)]
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

impl From<VMError> for ApiError {
    fn from(err: VMError) -> Self {
        match err {
            VMError::VmNotFound(id) => ApiError::new(
                StatusCode::NOT_FOUND,
                format!("VM with ID {} not found", id),
            ),
            VMError::VmAlreadyExists(id) => ApiError::new(
                StatusCode::CONFLICT,
                format!("VM with ID {} already exists", id),
            ),
            VMError::CreationFailed(id) => ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to create VM with ID: {}", id),
            ),
            VMError::StartFailed(id) => ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to start VM with ID: {}", id),
            ),
            VMError::StopFailed(id) => ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to stop VM with ID: {}", id),
            ),
            VMError::DeletionFailed(id) => ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to delete VM with ID: {}", id),
            ),
            VMError::InvalidState {
                vm_id,
                current,
                expected,
            } => ApiError::new(
                StatusCode::BAD_REQUEST,
                format!(
                    "VM with ID: {} is in invalid state: current {}, expected: {}",
                    vm_id, current, expected
                ),
            ),
        }
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        if let Ok(sindri_err) = err.downcast::<VMError>() {
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

// TODO: Add more rejection types as needed and use a cleaner wrapper
impl From<JsonRejection> for ApiError {
    fn from(rejection: JsonRejection) -> Self {
        ApiError::new(
            StatusCode::BAD_REQUEST,
            format!("Invalid JSON request: {}", rejection),
        )
    }
}

// TODO: Add more rejection types as needed and use a cleaner wrapper
impl From<PathRejection> for ApiError {
    fn from(rejection: PathRejection) -> Self {
        ApiError::new(
            StatusCode::BAD_REQUEST,
            format!("Invalid path parameter: {}", rejection),
        )
    }
}

// TODO: Add more rejection types as needed and use a cleaner wrapper
impl From<QueryRejection> for ApiError {
    fn from(rejection: QueryRejection) -> Self {
        ApiError::new(
            StatusCode::BAD_REQUEST,
            format!("Invalid query parameter: {}", rejection),
        )
    }
}

// TODO: Add more rejection types as needed and use a cleaner wrapper
impl From<ExtensionRejection> for ApiError {
    fn from(rejection: ExtensionRejection) -> Self {
        ApiError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Missing extension: {}", rejection),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_error_new_sets_fields() {
        let err = ApiError::new(StatusCode::BAD_REQUEST, "bad request");
        assert_eq!(err.status, StatusCode::BAD_REQUEST);
        assert_eq!(err.message, "bad request");
    }

    #[test]
    fn from_vm_error_not_found() {
        let err = VMError::VmNotFound(123);
        let api_err: ApiError = err.into();
        assert_eq!(api_err.status, StatusCode::NOT_FOUND);
        assert!(api_err.message.contains("VM with ID 123 not found"));
    }

    #[test]
    fn from_vm_error_already_exists() {
        let err = VMError::VmAlreadyExists(456);
        let api_err: ApiError = err.into();
        assert_eq!(api_err.status, StatusCode::CONFLICT);
        assert!(api_err.message.contains("VM with ID 456 already exists"));
    }

    #[test]
    fn from_vm_error_creation_failed() {
        let err = VMError::CreationFailed(789);
        let api_err: ApiError = err.into();
        assert_eq!(api_err.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert!(api_err.message.contains("Failed to create VM with ID: 789"));
    }

    #[test]
    fn from_vm_error_start_failed() {
        let err = VMError::StartFailed(101);
        let api_err: ApiError = err.into();
        assert_eq!(api_err.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert!(api_err.message.contains("Failed to start VM with ID: 101"));
    }

    #[test]
    fn from_vm_error_stop_failed() {
        let err = VMError::StopFailed(202);
        let api_err: ApiError = err.into();
        assert_eq!(api_err.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert!(api_err.message.contains("Failed to stop VM with ID: 202"));
    }

    #[test]
    fn from_vm_error_deletion_failed() {
        let err = VMError::DeletionFailed(303);
        let api_err: ApiError = err.into();
        assert_eq!(api_err.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert!(api_err.message.contains("Failed to delete VM with ID: 303"));
    }

    #[test]
    fn from_vm_error_invalid_state() {
        let err = VMError::InvalidState {
            vm_id: 404,
            current: "Stopped".to_string(),
            expected: "Running".to_string(),
        };
        let api_err: ApiError = err.into();
        assert_eq!(api_err.status, StatusCode::BAD_REQUEST);
        assert!(api_err.message.contains("VM with ID: 404"));
        assert!(api_err.message.contains("current Stopped"));
        assert!(api_err.message.contains("expected: Running"));
    }

    #[test]
    fn from_anyhow_error_downcast_vm_error() {
        let vm_err = VMError::VmNotFound(123);
        let err = anyhow!(vm_err);
        let api_err: ApiError = err.into();
        assert_eq!(api_err.status, StatusCode::NOT_FOUND);
        assert!(api_err.message.contains("VM with ID 123 not found"));
    }

    #[test]
    fn from_anyhow_error_generic() {
        let err = anyhow!("some random error");
        let api_err: ApiError = err.into();
        assert_eq!(api_err.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(api_err.message, "Internal server error");
    }

    #[test]
    fn into_response_returns_correct_status() {
        let err = ApiError::new(StatusCode::BAD_REQUEST, "test error");
        let resp = err.into_response();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
