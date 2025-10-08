use sindri_core::vm::VM;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Daemon {
    vms: HashMap<u32, VM>,
    next_id: u32,
}

impl Daemon {
    pub fn new() -> Self {
        Self {
            vms: HashMap::new(),
            next_id: 1,
        }
    }
}
