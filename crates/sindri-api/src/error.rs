#![allow(unused_imports)]
use anyhow::anyhow;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use sindri_core::errors::vm::SindriError;

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
    fn from_sindri_error_vm_not_found() {
        let err = SindriError::VmNotFound("123".to_string());
        let api_err: ApiError = err.into();
        assert_eq!(api_err.status, StatusCode::NOT_FOUND);
        assert!(api_err.message.contains("VM not found: 123"));
    }

    #[test]
    fn from_sindri_error_vm_already_exists() {
        let err = SindriError::VmAlreadyExists("abc".to_string());
        let api_err: ApiError = err.into();
        assert_eq!(api_err.status, StatusCode::CONFLICT);
        assert!(api_err.message.contains("VM already exists: abc"));
    }

    #[test]
    fn from_sindri_error_invalid_config() {
        let err = SindriError::InvalidConfig("bad config".to_string());
        let api_err: ApiError = err.into();
        assert_eq!(api_err.status, StatusCode::BAD_REQUEST);
        assert!(
            api_err
                .message
                .contains("Invalid VM configuration: bad config")
        );
    }

    #[test]
    fn from_anyhow_error_downcast_sindri() {
        let sindri_err = SindriError::VmNotFound("xyz".to_string());
        let err = anyhow!(sindri_err);
        let api_err: ApiError = err.into();
        assert_eq!(api_err.status, StatusCode::NOT_FOUND);
        assert!(api_err.message.contains("VM not found: xyz"));
    }

    #[test]
    fn from_anyhow_error_other() {
        let err = anyhow!("some error");
        let api_err: ApiError = err.into();
        assert_eq!(api_err.status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(api_err.message, "Internal server error");
    }

    #[test]
    fn into_response_returns_json() {
        let err = ApiError::new(StatusCode::BAD_REQUEST, "bad request");
        let resp = err.into_response();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
