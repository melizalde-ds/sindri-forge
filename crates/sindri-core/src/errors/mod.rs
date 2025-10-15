use serde::{Deserialize, Serialize};

pub mod socket;
pub mod vm;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum SindriError {
    Vm(vm::VMError),
    Socket(socket::SocketError),
}

pub type Result<T> = std::result::Result<T, SindriError>;
