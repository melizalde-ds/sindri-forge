use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum SocketError {
    #[error("Cannot connect to daemon socket at {path}: {reason}")]
    ConnectionFailed { path: String, reason: String },

    #[error("Connection lost while {operation}")]
    ConnectionLost { operation: String },

    #[error("Socket file not found at path: {path}")]
    SocketNotFound { path: String },

    #[error("Request timed out after {seconds} seconds")]
    Timeout { seconds: u64 },

    #[error("Read timeout - no response from daemon")]
    ReadTimeout,

    #[error("Failed to send request: {0}")]
    SendFailed(String),

    #[error("Failed to read response: {0}")]
    ReadFailed(String),

    #[error("Failed to serialize request: {0}")]
    SerializeRequest(String),

    #[error("Failed to deserialize response: {0}")]
    DeserializeResponse(String),

    #[error("Invalid message format: {0}")]
    InvalidFormat(String),

    #[error("Permission denied accessing socket at {path}")]
    PermissionDenied { path: String },
}
