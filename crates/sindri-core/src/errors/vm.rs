use thiserror::Error;

#[derive(Error, Debug)]
pub enum VMError {
    #[error("VM not found: {0}")]
    VmNotFound(u32),

    #[error("VM already exists: {0}")]
    VmAlreadyExists(u32),

    #[error("Invalid VM configuration: {0}")]
    InvalidConfig(String),
}
