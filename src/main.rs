mod ethernet;
mod mac_table;
mod vswitch;
mod transport;

use ethernet::{EthernetFrame, MacAddress};
use vswitch::VSwitch;

#[tokio::main]
async fn main() {
    let mac_a = MacAddress::new([0x02, 0x00, 0x00, 0x00, 0x00, 0x01]);
    let mac_b = MacAddress::new([0x02, 0x00, 0x00, 0x00, 0x00, 0x02]);

    let mut switch = VSwitch::new(vec![1, 2, 3]);

    let frame = EthernetFrame::new(
        mac_b,
        mac_a,
        0x0800,
        b"Hello from VNet-L2".to_vec(),
    );

    let forwarded = switch.forward(1, &frame);

    println!("VNet-L2 Layer 2 switch");
    println!("Source MAC: {}", frame.source);
    println!("Destination MAC: {}", frame.destination);
    println!("Forwarded ports: {:?}", forwarded);

    if let Err(error) = transport::start_udp_server("127.0.0.1:9000").await {
        eprintln!("UDP server error: {}", error);
    }
}