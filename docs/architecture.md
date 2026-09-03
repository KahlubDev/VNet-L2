# VNet-L2 Architecture

VNet-L2 is a Rust-based Layer 2 virtual switch.

## Components

VSwitch
The central switching component.

MAC Table
Stores learned MAC addresses and their associated virtual ports.

VPort
Represents a virtual network connection between a host and VSwitch.

UDP Transport
Provides network transport between virtual ports and VSwitch.

TAP Device
Provides a virtual Ethernet interface for integration with the host operating system.

## Frame Flow

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

## Forwarding

When a frame arrives, VSwitch learns the source MAC address.

For a known destination MAC, VSwitch sends the frame to the associated port.

For an unknown destination MAC, VSwitch floods the frame to every port except the incoming port.

Broadcast frames are also flooded to every other port.