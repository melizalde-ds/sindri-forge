use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum VMError {
    #[error("VM not found: {0}")]
    VmNotFound(u32),

    #[error("VM already exists: {0}")]
    VmAlreadyExists(u32),

    #[error("Invalid VM state: {vm_id}, current: {current}, expected: {expected}")]
    InvalidState {
        vm_id: u32,
        current: String,
        expected: String,
    },

    #[error("VM creation failed: {0}")]
    CreationFailed(u32),

    #[error("VM start failed: {0}")]
    StartFailed(u32),

    #[error("VM stop failed: {0}")]
    StopFailed(u32),

    #[error("VM deletion failed: {0}")]
    DeletionFailed(u32),
}
