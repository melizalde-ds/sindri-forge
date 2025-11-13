use crate::vm::{VM, VMId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Represents a request to be sent to the VM management daemon via Unix socket.
pub enum SocketRequest {
    /// Creates a new VM with the specified configuration.
    CreateVM(VM),
    /// Retrieves information about a VM with the given ID.
    GetVM(VMId),
    /// Lists all VMs managed by the daemon.
    ListVMs,
    /// Deletes the VM with the specified ID.
    DeleteVM(VMId),
    /// Starts the VM with the specified ID.
    StartVM(VMId),
    /// Stops the VM with the specified ID.
    StopVM(VMId),
    /// Retrieves metrics for the VM with the specified ID.
    GetVMMetrics(VMId),
    /// Updates the configuration of the VM with the specified ID.
    UpdateVM(VMId, VM),
}
