mod ethernet;

use ethernet::{EthernetFrame, MacAddress};

fn main() {
    let source = MacAddress::new([0x02, 0x00, 0x00, 0x00, 0x00, 0x01]);
    let destination = MacAddress::broadcast();

    let frame = EthernetFrame::new(
        destination,
        source,
        0x0800,
        b"Hello from VNet-L2".to_vec(),
    );

    println!("Source MAC: {}", frame.source);
    println!("Destination MAC: {}", frame.destination);
    println!("EtherType: 0x{:04x}", frame.ether_type);
    println!("Payload: {}", String::from_utf8_lossy(&frame.payload));
}