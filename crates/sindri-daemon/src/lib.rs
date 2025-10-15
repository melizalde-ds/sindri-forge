use sindri_core::vm::{VM, VMId};
use std::{collections::HashMap, sync::atomic::AtomicU32};
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct Daemon {
    vms: RwLock<HashMap<VMId, VM>>,
    next_id: AtomicU32,
}

impl Daemon {
    pub fn new() -> Self {
        Self {
            vms: RwLock::new(HashMap::new()),
            next_id: AtomicU32::new(1),
        }
    }
}

impl Default for Daemon {
    fn default() -> Self {
        Self {
            vms: RwLock::new(HashMap::new()),
            next_id: AtomicU32::new(1),
        }
    }
}
