use crate::core::errors::LwipErrorCode;
use crate::LwipError;

#[allow(non_camel_case_types)]
#[repr(C,u8)]
pub enum IpProto {
    HopOpt = 0, // IPv6 Hop By Hop, RF-C8200
    ICMP = 1, // ICMP, RFC-792
    IGMP = 2, // IGMP, RFC-1112
    GGP = 3, // GW to GW, RFC-823
    IPv4 = 4, // IPv4 Encap, RFC-2003
    ST = 5, // Streawm, RFC-1190, RFC-1819
    TCP = 6, // TCP, RFC-ietf-tcpm-rfc-793bis-28
    CBT = 7, // ??
    EGP = 8, // Ext GW Proto, RFC-888
    IGP = 9, // any private interior GW (used by cisco for IGRP)
    BBN_RCC_MON = 10, // BBN RCC Monitoring, ??
    NVP_II = 11, // Network Voice Proto, RFC-741
    PUP = 12, // Xerox PUP
    // ARGUS = 13, DEPRECATED
    EMCON = 14, // ??
    XNET = 15, // Cross Net Debugger, IEN 158
    CHAOS = 16, // MIT Chaos?
    UDP = 17, // UDP, RFC-768
    MUX = 18, // Multiplexing IEN 90
    DCN_MEAS = 19, // DCN Measurement Subsystems ???
    HMP = 20, // Host Monitoring RFC-869
    PRM = 21, // Packet Radio Measurement ??
    XNS_IDP = 22, // Xerox NS IDP
    Trunk1 = 23, // Trunk-1 ??
    Trunk2 = 24, // Trunk-2 ??
    Leaf1 = 25, // Leaf-1 ??
    Leaf2 = 26, // Leaf-2 ??
    RDP = 27, // Reliable Data Protocol, RFC-908
    IRTP = 28, // Internet Reliable Transaction, RFC-938
    ISO_TP4 = 29, // IOS Transport Protocol Class 4, RFC-905
    NETBLT = 30, // Bulk Data Transfer Protocol, RFC-969
    MFE_NSP = 31, // MFE Network Services Protocol, ??
    MERIT_INP = 32, // MERIT Internodal Protocol ??
    DCCP = 33, // Datagram Congestion Control Protocol, RFC-4340
    IP_3PC = 34, // Third Party Connect Protocol, ??
    IDPR = 35, // Inter-Domain Policy Routing Protocol, ??
    XTP = 36, // XTP, ??
    DDP = 37, // Datagram Delivery Protocol, ??
    IDPR_CMTP = 38, // IDPR Control Message Transport Proto, ??
    TP_PP = 39, // TP++ Transport Protocol, ??
    IL = 40, // IL Transport Protocol, ??
    IPv6 = 41, // IPv6 Encapsulation, RFC-2473
    SDRP = 42, // Source Demand Routing Protocol, ??
    IPV6_Route = 43, // Routing Header for IPv6, ??
    IPV6_Frag = 44, // Fragment Header for IPv6, ??
    IDRP = 45, // Inter-Domain Routing Protocol, ??
    RSVP = 46, // Reservation Protocol, RFC-2205, RFC-3209
    GRE = 47, // Generic Routing Encapsulation, RFC-2784
    DSR = 48, // Dynamic Source Routing Protocol, RFC-4728
    BNA = 49, // BNA, ??
    ESP = 50, // Encap Security Payload, RFC-4303
    AH = 51, // Authentication Header, RFC-4302
    I_NLSP = 52, // Integrated Net Layer Security TUBA, ??
    // SWIPE Deprecated, 53, IP with Encryption
    NARP = 54, // NBMA Address Resolution Protocol, RFC-1735
    MOBILE = 55, // IP Mobility, ??
    TLSP = 56, // TLS Protocol Using Kryptonet key management, ??
    SKIP = 57, // SKIP, ??
    IPv6_ICMP = 58, // ICMP for IPv6, RFC-8200
    IPv6_NoNxt = 59, // No next hdr for IPv6, RFC-8200
    IPv6_Opts = 60, // IPv6 Destination Options, RFC-8200
    AnyHostInternal = 61, // Any Host Internal Protocol, ??
    CFTP = 62, // CFTP, ??
    AnyLocalNet = 63, // Any Local Network, ??
    SAT_EXPAK = 64, // SATNET and Backroom EXPAK, ??
    KRYPTOLAN = 65, // Kryptolan, ??
    RVD = 66, // MIT Remote Virtual Disk Protocol, ??
    IPPC = 67, // Internet Pluribus Packet Core, ??
    AnyDistFS = 68, // Any distributed File System, ??
    SAT_MON = 69, // SATNET Monitoring, ??
    VISA = 70, // VISA Protocol, ??
    IPCV = 71, // IP Core Utility, ??
    CPNX = 72, // Computer Protocol Network Executive, ??
    CPHB = 73, // Computer Protocol Heart Beat, ??
    WSN = 74, // Wing Span Network, ??
    PVP = 75, // Packet Video Protocol, ??
    BR_SAT_MON = 76, // Backroom SATNET Monitoring, ??
    SUN_ND = 77, // Sun ND Protocol-Temporary, ??
    WB_MON = 78, // WIDEBAND Monitoring, ??
    WB_EXPAK = 79, // WIDEBAND EXPAK, ??
    ISO_IP = 80, // ISO IP, ??
    VMTP = 81, // VMTP, ??
    SECURE_VMTP = 82, // Secure VMTP, ??
    VINES = 83, // VINES, ??
    TTP_IPTM = 84, // Transaction Transport Protocol, ??; Internet Protocol Traffic Manager, ??
    NSFNET_IGP = 85, // NSFNET-IGP, ??
    DGP = 86, // Dissimilar Gateway Protocol, ??
    TCF = 87, // TCF, ??
    EIGRP = 88, // EIGRP, RFC-7868
    OSPF_IGP = 89, // OSPF IGP, RFC-1583, RFC-2328, RFC-5340
    SpriteRPC = 90, // Sprite RPC Protocol
    LARP = 91, // Locus Address Resolution Protocol, ??
    MTP = 92, // Multicast Transport Protocol
    AX_25 = 93, // AX.25 Frames
    IPIP = 94, // IP-within-IP encapsulation protocol, ??
    // MICP Deprecated, 95, Mobile Internetworking Control Protocol, ??
    SCC_SP = 96, // Semaphore Communications Security Protocol, ??
    ETHER_IP = 97, // Ethernet within IP Encapsulation, RFC-3378
    ENCAP = 98, // Encapsulation Header, RFC-1241
    AnyPrivEncrypt = 99, // Any Private Encryption Scheme
    GMTP = 100, // GMTP, ??
    IFMP = 101, // Ipsilon Flow Management Protocol, ??
    PNNI = 102, // PNNI over IP, ??
    PIM = 103, // Protocol Independent Multicast, RFC-7761
    ARIS = 104, // ARIS, ??
    SCPS = 105, // SCPS, ??
    QNX = 106, // QNX, ??
    A_N = 107, // Active Networks, ??
    IPComp = 108, // IP Payload Compression Protocol, RFC-2393
    SNP = 109, // Sitara Networks Protocol, ??
    CompaqPeer = 110, // Compaq Peer Protocol, ??
    IPXinIP = 111, // IPX in IP, ??
    VRRP = 112, // Virtual Router Redundancy Protocol, RFC-5798
    PGM = 113, // PGM Reliable Transport Protocol, ??
    Any0Hop = 114, // Any Zero Hop Protocol, ??
    L2TP = 115, // Layer Two Tunnelling Protocol, RFC-3931
    DDX = 116, // D-II Data Exchange, ??
    IATP = 117, // Interactive Agent Transfer Protocol, ??
    STP = 118, // Schedule Transfer Protocol, ??
    SRP = 119, // SpectraLink Radio Protocol, ??
    UTI = 120, // UTI, ??
    SMP = 121, // Simple Message Protocol, ??
    // DEPRECATED SM, Simple Multicast Protocol, 122, draft-perlman-simple-multicast
    PTP = 123, // Performance Transparency Protocol, ??
    ISIS_IPV4 = 124, // IS-IS over IPv4, ??
    FIRE = 125, // FIRE, ??
    CRTP = 126, // Combat Radio Transport Protocol, ??
    CRUDP = 127, // Combat Radio User Datagram, ??
    SSCOPMCE = 128, // SSCOPMCE, ??
    IPLT = 129, // IPLT, ??
    SPS = 130, // Secure Packet Shield, ??
    PIPE = 131, // Private IP Encapsulation within IP, ??
    SCTP = 132, // Stream Control Transmission Protocol, ??
    FC = 133, // Fibre Channel, RFC-6172
    RSVP_E2E_IGNORE = 134, // RSVP E2E IGNORE, RFC-3175
    MobilityHeader = 135, // Mobility Header, RFC-6275
    UDPLite = 136, // UDPLite, RFC-3828
    MPLS_IP = 137, // MPLS-in-IP, RFC-4023
    MANET = 138, // MANET protocols RFC-5498
    HIP = 139, // Host Identity Protocol, RFC-7401
    Shim6 = 140, // Shim6 Protocol, RFC-5533
    WESP = 141, // Wrapped Encapsulating Security Payload, RFC-5840
    ROHC = 142, // Robust Header Compression, RFC-5858
    Ethernet = 143, // Ethernet, RFC-8986
    UnassignedStart = 144,
    UnassignedEnd = 145,
    Experimental1 = 146, // RFC-3692
    Experimental2 = 147, // RFC-3692
    Reserved = 255, // IANA
}

impl IpProto {

}

fn match_ip_proto(a: u8) -> IpProto {
    let mut out_ipp: IpProto = IpProto::Reserved;
    for ipp in IpProto::iter() {
        if a == ipp as u8 {
            out_ipp = ipp;
        }
    }
    out_ipp
}

impl From<u8> for IpProto {
    fn from(a: u8) -> Self {
        match_ip_proto(a)
    }
}
