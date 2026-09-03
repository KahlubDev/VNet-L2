mod ethernet;
mod mac_table;
mod vswitch;

use ethernet::{EthernetFrame, MacAddress};
use vswitch::VSwitch;

fn main() {
    let mac_a = MacAddress::new([0x02, 0x00, 0x00, 0x00, 0x00, 0x01]);
    let mac_b = MacAddress::new([0x02, 0x00, 0x00, 0x00, 0x00, 0x02]);
    let mac_unknown = MacAddress::new([0x02, 0x00, 0x00, 0x00, 0x00, 0x03]);

    let mut switch = VSwitch::new(vec![1, 2, 3]);

    println!("VNet-L2 switch started");
    println!("Ports: 1, 2, 3");

    let frame_a = EthernetFrame::new(
        mac_b,
        mac_a,
        0x0800,
        b"Hello from port 1".to_vec(),
    );

    let forwarded = switch.forward(1, &frame_a);

    println!("Unknown destination:");
    println!("Frame from port 1 forwarded to: {:?}", forwarded);

    let frame_b = EthernetFrame::new(
        mac_a,
        mac_b,
        0x0800,
        b"Reply from port 2".to_vec(),
    );

    let forwarded = switch.forward(2, &frame_b);

    println!("Known destination:");
    println!("Frame from port 2 forwarded to: {:?}", forwarded);

    let broadcast = EthernetFrame::new(
        MacAddress::broadcast(),
        mac_a,
        0x0800,
        b"Broadcast message".to_vec(),
    );

    let forwarded = switch.forward(1, &broadcast);

    println!("Broadcast:");
    println!("Frame from port 1 forwarded to: {:?}", forwarded);

    let _ = mac_unknown;
}