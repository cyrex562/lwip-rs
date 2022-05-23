use crate::core::errors::LwipErrorCode;
use crate::LwipError;

#[allow(non_camel_case_types)]
#[repr(C, u16)]
pub enum EtherType {
    NotSet = 0,
    LengthFieldMax = 0x05dc, // values under this are a length field for an 802.3/802.2 packet
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
    IEEE_802_1_X = 0x888e, // IEEE 802.1X port-base network access control
    MACSEC = 0x88e5, // IEEE 802.1AE MAC Security
    PBB_INST_TAG = 0x88e7, // Provider Backbone Bridgding (PBB) Instance Tag, IEEE 802.1Q-2014
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
    
    // 0x8861 MCAP Multicast Channel Allocation Protocol RFC7042
    // 0x0101 - 0x01FF, experimental

}

impl EtherType {
    pub fn is_len_type(&self, raw_ether_type: u16) -> bool {
        raw_ether_type <= self::ETHERTYP_LEN_FIELD_MAX
    }
}

impl TryFrom<u16> for EtherType {
    type Error = LwipError;

    fn try_from(raw_ether_type: u16) -> Result<Self, Self::Error> {
        match raw_ether_type {
            x if x == EtherType::NotSet as u16 => Ok(EtherType::NotSet),
            x if x <= EtherType::LengthFieldMax as u16 => Ok(EtherType::LengthFieldMax),
            x if x == EtherType::IPv4 as u16 => Ok(EtherType::IPv4),
            x if x == EtherType::IPv6 as u16 => Ok(EtherType::IPv6),
            x if x == EtherType::ARP as u165 => Ok(EtherType::ARP),
            _ => Err(LwipError::new(LwipErrorCode::InvalidData, "invalid/unsupported raw ether type {:02x}".format(raw_ether_type)))
        }
    }
}

impl TryInto<u16> for EtherType {
    type Error = LwipError;

    fn try_into(self) -> Result<u16, Self::Error> {
        match EtherType {
            EtherType::NotSet => Ok(0),
            EtherType::LengthFieldMax => Ok(EtherType::LengthFieldMax as u16),
            EtherType::IPv4 => Ok(EtherType::IPv4 as u16),
            EtherType::IPv6 => Ok(EtherType::IPv6 as u16),
            EtherType::ARP => Ok(EtherType::ARP as u16),
            _ => Err(LwipError::new(LwipErrorCode::InvalidData, "invalid/unsupported ethertype for conversion to u16: {}".format(EtherType)))
        }
    }
}


// 8906                          Cisco Systems, Inc                           FCoE - Fibre Channel over Ethernet
//                               170 W Tasman Drive
//                               San Jose  CA  95134
//                               US
//
//
// 8904                          Cisco Systems, Inc                            BCN (Backward Congestion Notification) data frame tag
//                               170 W Tasman Drive
//                               San Jose  CA  95134
//                               US
//

// 22F4                          IETF TRILL Working Group                     IS-IS (Intermediate System to Intermediate System) is a link state routing
//                               c/o Internet Society                         protocol described in ISO/IEC 10589:2002 and IETF RFC 1195. For an example
//                               Reston  VA  20190-5108                       of Layer 2 use, see
//                               US                                           http://tools.ietf.org/id/draft-ietf-trill-rbridge-protocol-15.txt
//                                                                            The
//                                                                            final document can be found here: http://www.ietf.org/rfc/rfc6325.txt
//
//
// 22EF                          Anagran, Inc                                 This Ethertype will be used for FSA signalling - Q.Flowstatesig .
//                               580 North Pastoria Ave.
//                               Sunnyvale  CA  94085
//                               US
//
//

// F1C1                          IEEE 802.1 Chair                             Redundancy tag (R-TAG) as defined in IEEE Std 802.1CB
//                                c/o RAC Administrator , IEEE
//                               Piscataway  NJ  08554
//                               US
//
//
// 22E7                          IEEE 802.1 Chair                             Congestion Notification Message (CNM) as defined in IEEE Std 802.1Q
//                                c/o RAC Administrator , IEEE
//                               Piscataway  NJ  08554
//                               US
//
//
// 22E9                          IEEE 802.1 Chair                             Congestion Notification Tag (CN-TAG) as defined in IEEE Std 802.1Q
//                                c/o RAC Administrator , IEEE
//                               Piscataway  NJ  08554
//                               US
//
//
// 888e                          IEEE 802.1 Chair                             Port Access Entity (PAE) for Ethernet Access Protocol (EAP) over LAN
//                                c/o RAC Administrator , IEEE                (EAPOL) as defined in IEEE Std 802.1X
//                               Piscataway  NJ  08554
//                               US
//
//
// 88a8                          IEEE 802.1 Chair                             Service VLAN Tag (S-TAG) or Backbone VLAN Tag (B-TAG) as defined in IEEE
//                                c/o RAC Administrator , IEEE                Std 802.1Q
//                               Piscataway  NJ  08554
//                               US
//
//
// 8952                          IEEE 802.1 Chair                             Distributed Relay Control Protocol (DRCP) as defined in IEEE Std 802.1AX
//                                c/o RAC Administrator , IEEE
//                               Piscataway  NJ  08554
//                               US
//
//
// 88e7                          IEEE 802.1 Working Group                     Backbone Service Instance Tag (I-TAG) as defined in IEEE Std 802.1Q
//                               M/S P7903B12
//                               Santa Clara  CA  95054
//                               US
//
//
// 88c7                          IEEE 802.11 Working Group                    RSNA Preauthentication as defined in IEEE Std 802.11
//                               c/o RAC Administrator
//                               Piscataway   NJ  08854
//                               US
//
//

// 0806                          Symbolics, Inc.                              Address Resolution Protocol - A. R. P.
//                               243 Vassar Street
//                               Cambridge    02139
//                               US
//
//
// 88e1                          HomePlug Powerline Alliance, Inc.            HomePlug Specification AV MME
//                               2400 Camino Ramon, #375
//                               San Ramon    94583
//                               US
//
//
// 88a2                          Coraid Inc.                                  Advanced Technology Advancement (ATA) Protocol
//                               565 Research Dr.
//                               Athens  GA  30605
//                               US
//
//
// 889a                          Data Storage Institute                       SCSI over Ethernet
//                               DSI Building, 5 Engineering Drive 1
//                               Kent Ridge Crescent  NUS  117608
//                               SG
//
//
// 8903                          Cisco Systems, Inc                           DCE
//                               170 W Tasman Drive
//                               San Jose  CA  95134
//                               US
//
// 8905                          Cisco Systems, Inc                           T-Tag (Timestamp Tag)
//                               170 W Tasman Drive                           This tag carries timestamp information as part of the
//                               San Jose  CA  95134                          Ethernet frame.
//                               US
//
//

// A8C8                          IEEE 1904 Access Networks Working Group      The Virtual Link Control (VLC) protocol for Ethernet-based subscriber
//                               445 Hoes Lane                                access networks. The VLC protocol is specified in IEEE Std. 1904.2. For
//                               Piscataway  NJ  08854-4141                   more information, visit https://www.ieee1904.org/2
//                               US
//

// 88b7                          IEEE 802.1 Chair                             OUI Extended EtherType as defined in IEEE Std 802. This EtherType value is
//                                c/o RAC Administrator , IEEE                available for public use and for prototype and vendor-specific protocol
//                               Piscataway  NJ  08554                        development.
//                               US
//
//
// 88e5                          IEEE 802.1 Working Group                     Media Access Control (MAC) Security tag as defined in IEEE Std 802.1AE
//                               IEEE 802.1 Chair, c/o RAC Administrator IEEE
//                               Piscataway  NJ  08854
//                               US
//

// 8917                          IEEE 802.21 Working Group                    Media Independent Service (MIS) protocol as defined in IEEE Std 802.21
//                               IEEE 802.21 Chair, c/o RAC Administrator IEEE
//                               Piscataway  NJ  08854
//                               US
//
//

// AB37                          IETF Routing Area                            Please read draft-ietf-bier-mpls-encapsulation-12 at
//                               5177 Brandin Court                           https://datatracker.ietf.org/doc/draft-ietf-bier-mpls-encapsulation/.
//                               Fremont  CA  94538
//                               US
//

// D672                          InMon Corp.                                  sFlow is is a multi-vendor measurement technology for sampling packets in
//                               1 Sansome Street, FL35                       Ethernet devices.
//                               San Francisco  CA  94104
//                               US

// 9E65                          ETSI                                         LTE-WLAN Aggregation Adaptation Protocol (LWAAP), used by LTE-WLAN
//                               650 Route des lucioles                       Aggregation (LWA) as defined in 3GPP TS 36.300
//                               Sophia antipolis    06921                    http://www.3gpp.org/DynaReport/36300.htm
//                               FR
//

// 8847                          Cisco Systems                                8847: MPLS (multiprotocol label switching) label stack - unicast
//                               1414 Massachusetts Ave.
//                               Boxborough  MA  01719                        reference: RFC 3032
//                               US                                            URL:
//                                                                            <ftp://ftp.rfc-editor.org/in-notes/rfc3032.txt
//
//                                                                            8848: MPLS (multiprotocol
//                                                                            label switching) label stack - multicast
//                                                                             reference: RFC 3032
//                                                                             URL:
//                                                                            <ftp://ftp.rfc-editor.org/in-notes/rfc3032.txt
//
//
// 8848                          Cisco Systems                                8847: MPLS (multiprotocol label switching) label stack - unicast
//                               1414 Massachusetts Ave.
//                               Boxborough  MA  01719                        reference: RFC 3032
//                               US                                            URL:
//                                                                            <ftp://ftp.rfc-editor.org/in-notes/rfc3032.txt
//
//                                                                            8848: MPLS (multiprotocol
//                                                                            label switching) label stack - multicast
//                                                                             reference: RFC 3032
//                                                                             URL:
//                                                                            <ftp://ftp.rfc-editor.org/in-notes/rfc3032.txt
//

// 893B                          IETF TRILL Working Group                     This EtherType is expected to be useful in a number of applications
//                               c/o Internet Society                         but
//                               Reston  VA  20190-5108                       this application is particularly motivated by standards use in
//                               US                                           an
//                                                                            extension to the IETF TRILL protocol standard. This EtherType
//                                                                            provide
//                                                                            a way to supply a 12-bit extension to the VLAN ID in a
//                                                                            proceeding VLAN tag
//                                                                            and space for an additional 3-bit priority value.
//                                                                            See the Protocol
//                                                                            description
//                                                                            at
//                                                                            http://www.pothole.com/~dee3/drafts/draft-eastlake-trill-rbridge-fine-labeling-02.txt,
//                                                                            especially
//                                                                            Section 2.3.
//
//

// 891f                          British Telecommunications Plc.              ITU-T Recommendation G.7041 Generic Framing Procedure
//                               81 Newgate Street                            (GFP)
//                                 London  EC1A 7AJ
//                               GB                                           http://www.itu.int/net/home/index.aspx
//

// 8809                          IEEE 802.3 Working Group                     Slow Protocols as defined in IEEE std 802.3
//                               IEEE 802.3 Chair, c/o RAC Administrator, IEEE
//                               Piscataway  NJ  08854
//                               US
//
//
// C9D1                          IEEE 802.1 Chair                             Legacy assignment (use 8870 instead) - LLC encapsulation as defined by IEEE
//                                c/o RAC Administrator , IEEE                Std 802.1AC-2016.
//                               Piscataway  NJ  08554
//                               US
//
//
// 893F                          IEEE 802.1 Chair                             Bridge Port Extension tag (E-TAG) as defined in IEEE Std 802.1BR
//                                c/o RAC Administrator , IEEE
//                               Piscataway  NJ  08554
//                               US
//
//
// 22E2                          IEEE 802.1 Chair                             MAC Status Protocol (MSP) as defined in IEEE Std 802.1Q
//                                c/o RAC Administrator , IEEE
//                               Piscataway  NJ  08554
//                               US
//
//
// 88f6                          IEEE 802.1 Chair                             Multiple MAC Registration Protocol (MMRP) as defined in IEEE Std 802.1Q
//                                c/o RAC Administrator , IEEE
//                               Piscataway  NJ  08554
//                               US
//
//
// 89A2                          IEEE 802.1 Chair                             Congestion Isolation Message (CIM) as defined in IEEE 802.1Qcz amendment to
//                                c/o RAC Administrator , IEEE                IEEE Std 802.1Q
//                               Piscataway  NJ  08554
//                               US
//
//
// 8902                          IEEE 802.1 Chair                             Connectivity Fault Management (CFM) Protocol Data Unit (PDU) Encapsulation
//                                c/o RAC Administrator , IEEE                as defined in IEEE 802.1Q
//                               Piscataway  NJ  08554
//                               US
//
//
// 8100                          IEEE 802.1 Chair                             Customer VLAN Tag (C-TAG) as defined in IEEE Std 802.1Q
//                                c/o RAC Administrator , IEEE
//                               Piscataway  NJ  08554
//                               US
//
//
// CCE0                          Video Services Forum, Inc.                   Reliable Internet Stream Transport is an open source, open specification
//                               208 84th Street 08242                        transport protocol designed for reliable transmission of video over lossy
//                               Sea Isle City  NJ  08243                     networks (including the internet) with low latency and high quality.
//                               US
//
//
