use thiserror::Error;

#[derive(Error, Debug)]
pub enum SindriError {
    #[error("VM not found: {0}")]
    VmNotFound(String),

    #[error("VM already exists: {0}")]
    VmAlreadyExists(String),

    #[error("Invalid VM configuration: {0}")]
    InvalidConfig(String),
}
