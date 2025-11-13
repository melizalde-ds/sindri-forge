use serde::{Deserialize, Serialize};

use crate::{
    errors::SindriError,
    vm::{VM, VMId, VMStatus},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SocketResponse {
    Ok { data: ResponseData },
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

impl SocketResponse {
    pub fn ok(data: ResponseData) -> Self {
        SocketResponse::Ok { data }
    }

    pub fn error(err: SindriError) -> Self {
        SocketResponse::Error(err)
    }
}
