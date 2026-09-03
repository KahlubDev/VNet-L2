use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct MacAddress([u8; 6]);

impl MacAddress {
    pub fn new(bytes: [u8; 6]) -> Self {
        Self(bytes)
    }

    pub fn broadcast() -> Self {
        Self([0xff; 6])
    }

    pub fn bytes(&self) -> [u8; 6] {
        self.0
    }
}

impl fmt::Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.0[0],
            self.0[1],
            self.0[2],
            self.0[3],
            self.0[4],
            self.0[5]
        )
    }
}

impl fmt::Debug for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

#[derive(Debug, Clone)]
pub struct EthernetFrame {
    pub destination: MacAddress,
    pub source: MacAddress,
    pub ether_type: u16,
    pub payload: Vec<u8>,
}

impl EthernetFrame {
    pub fn new(
        destination: MacAddress,
        source: MacAddress,
        ether_type: u16,
        payload: Vec<u8>,
    ) -> Self {
        Self {
            destination,
            source,
            ether_type,
            payload,
        }
    }
}