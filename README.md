# lwip-rs

## Notes
* A network interface can have one or more MAC addresses to identify it at layer 2
* A network interface can have one or more IPv4/Ipv6 addresses to identify it at layer 3
* A network interface can have zero layer 3 addresses, but must have at least one layer 2 address
* An ip address/MAC address can only be assigned to one interface
* 

## Requirements:
* For PCAP interface types, npcap or libpcap are required to be instsalled on the system
  * NPCAP: [https://npcap.com/]

## TODO Items

* TODO: when sending broadcast traffic via an interface, if the interface's broadcast flag is False, then do not send
* TODO: when sending broadcast traffic via IPv4, if the IPv4 dest address is the same as one of the network interface's addresses, then do not send
* TODO: handle multicast IPv4 traffic
* TODO: sniff and handle? VLAN trunking protocol traffic
* TODO: create and bridge interfaces
* TODO: support Netgear Switch Discovery Protocol (NSDP)
* TODO: support Dynamic Trunking Protocol
* TODO: update checksums of layer 3+ packets

## References

[https://datatracker.ietf.org/doc/html/rfc7126]
[https://www.iana.org/assignments/ip-parameters/ip-parameters.xhtml]
[https://datatracker.ietf.org/doc/html/rfc6192]
[https://en.wikipedia.org/wiki/IPv4]
[https://en.wikipedia.org/wiki/Multicast_address]
[https://en.wikipedia.org/wiki/MAC_address]
[https://en.wikipedia.org/wiki/Link-state_routing_protocol]
[https://en.wikipedia.org/wiki/IEEE_802.1aq]
[https://en.wikipedia.org/wiki/Ethernet_frame]
[https://en.wikipedia.org/wiki/IEEE_802.1Q]
[https://www.ieee802.org/1/pages/802.1ca.html]
[https://en.wikipedia.org/wiki/Multilayer_switch]/
