use crate::core::errors::LwipErrorCode;
use crate::LwipError;

#[allow(non_camel_case_types)]
#[repr(C, u16)]
pub enum EtherType {
    NotSet = 0,
    LengthFieldMax = 0x05dc, // 1500 base 10; values LTE to this are a length field for an 802.3/802.2 packet
    IPv4 = 0x0800, // IPv4, RFC7042
    ARP = 0x0806, // ARP, RFC7042
    VLAN_C_TAG = 0x8100, // VLAN 802.1q customer tag
    VLAN_S_TAG = 0x88a8, // VLAN 802.1q service provider tag
    IPv6 = 0x86dd, // IPv6, RFC7042
    LLDP = 0x88cc, // LLDP, IEEE 802.1AB
    TRILL = 0x22f3, // TRILL, RFC6325
    L2_IS_IS = 0x22f4, // L2-IS-IS, RFC6325
    RARP = 0x8035, // RFC903
    PPP = 0x880b, // RFC7042
    MPLS = 0x8847, // RFC5332
    MPLS_UPSTREAM_LABEL = 0x8848, // RFC5332 MPLS with upstream-assigned label
    PPPoE_Discovery = 0x8863, // RFC2516 RFC8822 PPPoE Discovery Stage
    PPPoE_Session = 0x8864, // RFC2561, RFC 8822 PPPoE Session Stage
    IEEE_802_1_X = 0x888e, // IEEE 802.1X port-base network access control. EAPOL
    MACSEC = 0x88e5, // IEEE 802.1AE MAC Security
    PBB_INST_TAG = 0x88e7, // Provider Backbone Bridgding (PBB) Instance Tag (I-TAG), IEEE 802.1Q-2014
    MVRP = 0x88f5, // IEEE 802.1Q Multiple VLAN Registration Protocol
    MMRP = 0x88f6, // IEEE 802.1Q Multiple Multicast Registration Protocol (MMRP)
    IEEE_802_11_R = 0x8900, // IEEE 802.11r Fast Roaming Remote Request
    TRILL_FGL = 0x893b, // TRILL Fine Grained Labeling (RFC7172)
    TRILL_BR = 0x8946, // TRILL Bridge Channel RFC-7178
    LOWPAN_ENCAP = 0xa0ed, // LoWPAN encapsulation, RFC-7973
    GRE_X = 0xb7ea, // RFC-8157 GRE control channel
    HD_BASE_T_CMP = 0x8938, // HDBaseT Control and Mgmt Protocol
    RDMAoE = 0x88ce, // RDMA over Ethernet
    IEEE_1722_2016 = 0x22f0, // Transport Protocol for Time-Sensitive Applications in Bridged Local Area Netweorks, IEEE 1722-2016
    MAC_Ctrl = 0x8808, // MAC Control IEEE 802.3
    LocalExperimental2 = 0x88b6, // Local Experimental EtherType #2 IEEE 802.3
    SRP = 0x22ea, // Stream Reservation Protocol IEEE 802.1Q
    ECP = 0x8940, // Edge Control Protocol IEEE 802.1Q
    MDCP = 0x8932, // Mellanox discovery and configuration protocol
    ROHC = 0x22f1, // Robust Header Compression RFC-3095
    IEEE_1588_1 = 0x88f7, // Used in the revised IEEE 1588 Standared for precision clock synch for networked systems
    LocalExperimental1 = 0x88b5, // Local Experimental EtherType #1 as in 802
    FlowFilteringTag = 0x894b, // Flow Filtering Tag F-TAG as in IEEE 802.1q
    MIRP = 0x8929, // Multiple I-SID Registration Protocol IEEE 802.1Q
    IEEE_80211_MP = 0x890d, // IEEE 802.11 Management Protocol
    LLC_Encap = 0x8870, // IEEE 802.1AC LLC encapsulation
    RoCE = 0x8915, // RDMA over Converged Ethernet
    MRP = 0x88e3, // Medium Redundancy Protocol
    IPX = 0x8137, // Internetwork Packet Exchange
    IPX2 = 0x8138, // Internetwork Packet Exchange #2
    EoIB = 0x894a, // Ethernet over InfiniBand
    FCoE = 0x8906, // Fiber Channel over Ethernet
    BCN = 0x8904, // Backward Congestion Notification
    RTAG = 0xf1c1, // Redundancy Tag R-TAG, 802.1cb
    CNM = 0x22e7, // Congestion Notification Message, IEEE 802.1Q
    CN_TAG = 0x22e9, // Congestion Notification Tag, IEEE 802.1Q
    DRCP = 0x8952, // Distributed Relay Control Protocol (DRCP) IEEE 802.1X
    RSNA_PRE_AUTH = 0x88c7, // RSNA Preauth IEEE 802.11
    SCSIoE = 0x889a, // SCSI over Ethernet, Data Storage Institute,
    DCE = 0x8903, // DCE, Cisco
    T_TAG = 0x8905, // Timestamp Tag, Cisco
    VLC = 0xa8c8, // Virtual Link Control protocol, IEEE 1904 working group, IEEE  1904.2
    MIS = 0x8917, // Media Independent Service protocol, IEEE 802.21
    MPLS_ENCAP = 0xab37, // MPLS encapsulation https://datatracker.ietf.org/doc/draft-ietf-bier-mpls-encapsulation/
    sFlow = 0xd672, // sFlow, InMon Corp
    LWAAP = 0x9e65, // LTE-WLAN aggregation protocool, ETSI, 3GPP TS 36.300
    GFP = 0x891f, // ITU-T Recommendation G.7041 Generic Framing Procedure, British Telecom,
    Slow = 0x8809, // Slow protocols as defined in IEEE 802.3
    BridgePortExt = 0x893f, // Bridge Port Extension E-TAG as in 802.1BR
    MacStatus = 0x22e2, // MAC Status Protocol as in 802.1Q
    CongestionIsolationMsg = 0x89a2, // Congestion Isolation Message per 802.1qcz
    ConnectivityFaultMgmt = 0x8902, // Connectivity Fault Management (CFM), 802.1q
    RIST = 0xcce0, // Reliable Internet Stream Transport, Video Services Forum
    // 0x8861 MCAP Multicast Channel Allocation Protocol RFC7042
    // 0x0101 - 0x01FF, experimental
    // 22EF Anagran, Inc This Ethertype will be used for FSA signalling - Q.Flowstatesig .
    // 88e1, HomePlug Powerline Alliance, HomePlug Specification AV MME
    // 88a2, Coraid, Inc, Advanced Techology Advancement (ATA)
    // 88b7, OUI Extended EtherTYpe for publice use and vendor-specific protocols, IEEE 802.1
}

impl EtherType {
}

fn match_ether_type(a: u16) -> Result<EtherType,LwipError> {
    for etype in EtherType::iter() {
        if a == etype as u16 {
            Ok(etype)
        }
    }
    Err(LwipError::new(LwipErrorCode::InvalidData, "no match for supplied raw ether type {:02x}".format(a)))
}

impl TryFrom<u16> for EtherType {
    type Error = LwipError;

    fn try_from(raw_ether_type: u16) -> Result<Self, Self::Error> {
        match_ether_type(raw_ether_type)
    }
}

impl TryFrom<&[u8]> for EtherType {
    type Error = LwipError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let raw_ether_type = u16::from_ne_bytes([value[0],value[1]]);
        match_ether_type(raw_ether_type)
    }
}


