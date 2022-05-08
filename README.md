# lwip-rs

## Notes
* A network interface can have one or more MAC addresses to identify it at layer 2
* A network interface can have one or more IPv4/Ipv6 addresses to identify it at layer 3
* A network interface can have zero layer 3 addresses, but must have at least one layer 2 address
* An ip address/MAC address can only be assigned to one interface
* 

## TODO Items

* TODO: when sending broadcast traffic via an interface, if the interface's broadcast flag is False, then do not send
* TODO: when sending broadcast traffic via IPv4, if the IPv4 dest address is the same as one of the network interface's addresses, then do not send
* TODO: handle multicast IPv4 traffic
* TODO: sniff and handle? VLAN trunking protocol traffic
* TODO: create and bridge interfaces
* TODO: support Netgear Switch Discovery Protocol (NSDP)
* TODO: support Dynamic Trunking Protocol
* TODO: update checksums of layer 3+ packets