use crate::ethernet::{EthernetFrame, MacAddress};
use crate::mac_table::{MacTable, PortId};

pub struct VSwitch {
    mac_table: MacTable,
    ports: Vec<PortId>,
}

impl VSwitch {
    pub fn new(ports: Vec<PortId>) -> Self {
        Self {
            mac_table: MacTable::new(),
            ports,
        }
    }

    pub fn forward(
        &mut self,
        incoming_port: PortId,
        frame: &EthernetFrame,
    ) -> Vec<PortId> {
        self.mac_table.learn(frame.source, incoming_port);

        if frame.destination == MacAddress::broadcast() {
            return self
                .ports
                .iter()
                .copied()
                .filter(|port| *port != incoming_port)
                .collect();
        }

        match self.mac_table.lookup(&frame.destination) {
            Some(destination_port) if destination_port != incoming_port => {
                vec![destination_port]
            }
            Some(_) => Vec::new(),
            None => self
                .ports
                .iter()
                .copied()
                .filter(|port| *port != incoming_port)
                .collect(),
        }
    }
}