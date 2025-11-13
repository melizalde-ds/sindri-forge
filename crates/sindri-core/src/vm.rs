use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VMId(pub u32);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VM {
    pub id: VMId,
    pub name: String,
    pub config: VMConfig,
    pub network: VMNetwork,
    pub storage: VMStorage,
    pub status: VMStatus,
    pub kernel: KernelConfig,
    pub runtime: Option<VMRuntime>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VMConfig {
    pub cpu: u8,
    pub memory: u64,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VMNetwork {
    pub ip_address: String,
    pub mac_address: String,
    pub subnet: u8,
    pub gateway: String,
    pub dns: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VMStorage {
    pub disks: Vec<Disk>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VMStatus {
    Running,
    Stopped,
    Paused,
    Suspended,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Disk {
    pub id: u32,
    pub size_gb: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KernelConfig {
    pub path: String,
    pub parameters: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VMRuntime {
    Firecracker(FirecrackerRuntime),
    CloudHypervisor(CloudHypervisorRuntime),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FirecrackerRuntime {
    pub pid: u32,
    pub socket_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CloudHypervisorRuntime {
    pub pid: u32,
    pub api_socket: String,
}

impl VM {
    pub fn new(
        id: u32,
        name: String,
        config: VMConfig,
        network: VMNetwork,
        storage: VMStorage,
        kernel: KernelConfig,
    ) -> Self {
        VM {
            id: VMId(id),
            name,
            config,
            network,
            storage,
            status: VMStatus::Stopped,
            kernel,
            runtime: None,
        }
    }
}

impl VMConfig {
    pub fn new(cpu: u8, memory: u64, metadata: Option<HashMap<String, String>>) -> Self {
        VMConfig {
            cpu,
            memory,
            metadata,
        }
    }
}

impl From<u32> for VMId {
    fn from(id: u32) -> Self {
        VMId(id)
    }
}
