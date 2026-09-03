# VNet-L2

A Rust-based Layer 2 virtual network switch inspired by the architecture of ZeroTier-style virtual networking.

VNet-L2 explores how Ethernet frames move between virtual network ports through a central virtual switch. The project focuses on MAC address learning, Layer 2 forwarding, frame flooding, and UDP-based transport.

## Project Status

- Core Layer 2 switching: Complete
- MAC address learning: Complete
- Ethernet frame representation: Complete
- UDP transport: Complete
- TAP integration: Planned
- Multi-host virtual networking: Planned

## Architecture

```text
Host A
  |
 TAP
  |
 VPort
  |
 UDP
  |
 VSwitch
  |
 MAC Table
  |
 UDP
  |
 VPort
  |
 TAP
  |
Host B
````

## Features

* Ethernet frame representation
* MAC address handling
* MAC address learning
* Known destination forwarding
* Unknown destination flooding
* Broadcast frame handling
* UDP transport
* Rust-based implementation
* Modular project structure

## How It Works

When an Ethernet frame enters the virtual switch, the source MAC address is learned and associated with the incoming port.

The destination MAC address is then checked against the MAC table.

If the destination is known, the frame is forwarded to the associated port.

If the destination is unknown, the frame is flooded to the other ports.

Broadcast frames are also forwarded to the other ports.

This demonstrates the basic forwarding behavior of a Layer 2 switch.

## Project Structure

```text
vnet-l2/
├── src/
│   ├── main.rs
│   ├── ethernet.rs
│   ├── mac_table.rs
│   ├── vswitch.rs
│   └── transport.rs
├── docs/
│   └── architecture.md
├── tests/
├── Cargo.toml
├── Cargo.lock
└── README.md
```

## Technologies

* Rust
* Tokio
* UDP
* Ethernet
* Layer 2 networking
* MAC address learning
* Linux
* WSL2
* Git
* GitHub
* tcpdump
* Wireshark

## Running the Project

### Clone the Repository

```bash
git clone https://github.com/KahlubDev/VNet-L2.git
cd VNet-L2
```

### Build the Project

```bash
cargo build
```

### Run the Project

```bash
cargo run
```

### Run Tests

```bash
cargo test
```

The UDP component listens on:

```text
127.0.0.1:9000
```

## What I Learned

This project helped me develop a practical understanding of Layer 2 networking and how virtual networking systems are structured.

### Networking

* How Ethernet frames are structured
* How source and destination MAC addresses work
* How switches learn MAC addresses
* How MAC tables support frame forwarding
* How unknown destinations are flooded
* How broadcast traffic is handled
* How UDP provides transport between virtual network components
* How virtual network components communicate through a central switch

### Rust

* Rust modules and project organization
* Structs and associated methods
* Ownership and borrowing
* Collections such as `HashMap`
* `Option` handling
* Error handling with `Result`
* Asynchronous programming with Tokio
* Separating networking logic into reusable modules

### Systems Programming

* Designing components around network responsibilities
* Representing network frames in software
* Building a virtual switching layer
* Separating switching logic from network transport
* Working with Linux networking tools
* Understanding how virtual interfaces interact with networking software

### Development Workflow

* Creating a Rust project with Cargo
* Organizing a project into multiple modules
* Using Git branches and commits
* Maintaining incremental development milestones
* Testing changes with `cargo test`
* Checking code with `cargo check`
* Documenting system architecture
* Using GitHub as a project portfolio

## Learning Goals

The main goal of VNet-L2 is to understand virtual networking by implementing core networking concepts rather than relying entirely on existing networking frameworks.

Future development will focus on:

* TAP interface integration
* Real virtual Ethernet interfaces
* Multi-host communication
* Packet serialization and deserialization
* Packet capture and traffic analysis
* Network error handling
* Authentication between virtual network components
* Encryption for UDP communication
* Improved testing and network simulation

## Development Approach

The project is being developed incrementally, with each networking concept implemented and tested as a separate milestone.

### Completed Milestones

1. Rust project initialization
2. Ethernet frame representation
3. MAC address handling
4. MAC address learning
5. Layer 2 forwarding
6. UDP transport

### Planned Milestones

7. Virtual port architecture
8. TAP interface integration
9. Multi-host connectivity
10. Packet serialization and deserialization
11. Network traffic analysis
12. Authentication
13. Encrypted communication
14. Improved automated testing

## Reference

The project architecture was inspired by:

[https://github.com/peiyuanix/build-your-own-zerotier](https://github.com/peiyuanix/build-your-own-zerotier)

The reference project demonstrates a Layer 2 virtual switch using a VSwitch, VPorts, UDP communication, MAC address learning, and TAP devices.

VNet-L2 is an independent Rust implementation created for learning and experimentation.

## Disclaimer

VNet-L2 is an educational networking project.

The project is intended to demonstrate networking concepts and systems programming techniques. It is not intended to replace production-grade VPN or virtual networking software.

## License

This project is intended for educational and experimental use.

```