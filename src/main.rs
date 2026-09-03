mod ethernet;
mod mac_table;

use ethernet::MacAddress;
use mac_table::MacTable;

fn main() {
    let mac_a = MacAddress::new([0x02, 0x00, 0x00, 0x00, 0x00, 0x01]);
    let mac_b = MacAddress::new([0x02, 0x00, 0x00, 0x00, 0x00, 0x02]);

    let mut table = MacTable::new();

    table.learn(mac_a, 1);
    table.learn(mac_b, 2);

    println!("MAC table entries: {}", table.len());

    match table.lookup(&mac_a) {
        Some(port) => println!("{} -> port {}", mac_a, port),
        None => println!("{} -> unknown", mac_a),
    }

    match table.lookup(&mac_b) {
        Some(port) => println!("{} -> port {}", mac_b, port),
        None => println!("{} -> unknown", mac_b),
    }
}