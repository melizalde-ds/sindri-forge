use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VMId(pub u32);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VM {
    pub id: VMId,
    pub name: String,
    pub cpu: u8,
    pub memory: u64,
    pub status: VMStatus,
    pub kernel: KernelConfig,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VMStatus {
    Running,
    Stopped,
    Paused,
    Suspended,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KernelConfig {
    pub path: String,
    pub parameters: Vec<String>,
}

impl VM {
    pub fn new(id: u32, name: String, cpu: u8, memory: u64, kernel: KernelConfig) -> Self {
        VM {
            id: VMId(id),
            name,
            cpu,
            memory,
            status: VMStatus::Stopped,
            kernel,
        }
    }
}

impl From<u32> for VMId {
    fn from(id: u32) -> Self {
        VMId(id)
    }
}
