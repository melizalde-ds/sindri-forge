use crate::vm::{VM, VMId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SocketRequest {
    CreateVM(VM),
    GetVM(VMId),
    ListVMs,
    DeleteVM(VMId),
    StartVM(VMId),
    StopVM(VMId),
    GetVMMetrics(VMId),
    UpdateVM(VMId, VM),
}
