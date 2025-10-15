use sindri_core::vm::VM;
use std::collections::HashMap;
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct Daemon {
    vms: RwLock<HashMap<u32, VM>>,
    next_id: u32,
}

impl Daemon {
    pub fn new() -> Self {
        Self {
            vms: RwLock::new(HashMap::new()),
            next_id: 1,
        }
    }
}

impl Default for Daemon {
    fn default() -> Self {
        Self {
            vms: RwLock::new(HashMap::new()),
            next_id: 1,
        }
    }
}
