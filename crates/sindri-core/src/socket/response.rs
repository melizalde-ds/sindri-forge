use serde::{Deserialize, Serialize};

use crate::{
    errors::SindriError,
    vm::{VM, VMId, VMStatus},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SocketResponse {
    OK { data: ResponseData },
    Error(SindriError),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResponseData {
    VM(VM),
    VMList(Vec<VM>),
    VMId(VMId),
    Status(VMStatus),
    Health(HealthCheck),
    Empty,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HealthCheck {
    Healthy,
    Unhealthy(String),
}
