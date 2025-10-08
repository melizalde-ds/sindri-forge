use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VM {
    pub id: u32,
    pub name: String,
    pub config: VMConfig,
    pub network: VMNetwork,
    pub storage: VMStorage,
    pub status: VMStatus,
    pub kernel: KernelConfig,
    pub runtime: Option<VMRuntime>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VMConfig {
    pub cpu: u8,
    pub memory: u64,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VMNetwork {
    pub ip_address: String,
    pub mac_address: String,
    pub subnet: u8,
    pub gateway: String,
    pub dns: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VMStorage {
    pub disks: Vec<Disk>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VMStatus {
    Running,
    Stopped,
    Paused,
    Suspended,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Disk {
    pub id: u32,
    pub size_gb: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KernelConfig {
    pub path: String,
    pub parameters: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VMRuntime {
    Firecracker(FirecrackerRuntime),
    CloudHypervisor(CloudHypervisorRuntime),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirecrackerRuntime {
    pub pid: u32,
    pub socket_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CloudHypervisorRuntime {
    pub pid: u32,
    pub api_socket: String,
}
