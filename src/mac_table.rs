use std::collections::HashMap;

use crate::ethernet::MacAddress;

pub type PortId = u32;

pub struct MacTable {
    entries: HashMap<MacAddress, PortId>,
}

impl MacTable {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn learn(&mut self, mac: MacAddress, port: PortId) {
        self.entries.insert(mac, port);
    }

    pub fn lookup(&self, mac: &MacAddress) -> Option<PortId> {
        self.entries.get(mac).copied()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }
}