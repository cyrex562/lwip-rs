
/// https://man7.org/linux/man-pages/man7/address_families.7.html
pub enum AddressFamily {
    NotSet,
    AF_UNIX, // local communication unix(7)
    AF_LOCAL, // alias for AF_UNIX
    AF_INET, // IPv4 ip(7)
    AF_AX25, // AX.25 ax25(4)
    AF_IPX, // Novell IPX
    AF_APPLETALK, // AppleTalk ddp(7)
    AF_NETROM, // AX.25 netrom(4)
    AF_BRIDGE, // rtnetlink(7)
    AF_ATMPVC, // ATM on Linux
    AF_X25, // x25(7)
    AF_INET6, // IPv6 protocols ipv6(7)
    AF_ROSE, // Radio Amateur Telecom Society Open AX.25 (see NETROM)
    AF_DECnet, // see Documentation/networking/decnet.txt
    AF_NETBEUI, // reserved for 802.LLC project; never used
    AF_SECURITY, // no longer used
    AF_KEY, // RFC2367
    AF_NETLINK, // netlink(7)
    AF_PACKET, // low-level packet interface packet(7)
    AF_ECONET, // acorn econet protocol, no longer used
    AF_ATMSVC, // ATM switched virtual circuits see ATM on linux howto
    AF_RDS, // Reliable datagram sockets, RDS over RDMA; see rds(7) and rds-rdma(7)
    AF_IRDA, //  Socket interface over IRDA irda(7)
    AF_PPPOX, // generic PPP transport layer for L2TP and PPPoE see documentation/networking/l2tp.txt
    AF_WANPIPE, // legacy protocol for WAN connectivity, no longer used
    AF_LLC, // IEEE 802.2 LLC
    AF_IB, // Infiniband native addressing
    AF_MPLS, // MPLS netlink(7)
    AF_CAN, // CAN documentation/networking/can.rst
    AF_TIPC, // cluster domain sockets protocol tipc.io/programming.html, tipc.io/protocol.html
    AF_BLUETOOTH, // bluetooth low-level socket protocol, bluez-intro
    AF_IUCV, // IUCV z/VM protocol
    AF_RXRPC, // Rx, Andrew File System RPC proto documentation/networking/rxrpc.txt
    AF_ISDN, // modular ISDN driver interface protocol
    AF_PHONET, // nokia cellular modem IPC/RPC interface
    AF_IEEE802154, // IEEE 802.15.4 WPAN raw packet protocol
    AF_CAIF, // Ercissons Communication CPU to Application CPU interface
    AF_ALG, // Interface to kernel crypto API
    AF_VSOCK, // VMware vsockets protocol for hypervisor-guest interaction vsock(7)
    AF_QIPCRTR, // Qualcomm IP router interface protocol
    AF_SMC, // SMC-R, shared comms over RDMA protocol and SMC-D share memory communications DMA for intra-node z/VM RFC7609
    AF_XDP, // XDP Express Data Path interface
}
