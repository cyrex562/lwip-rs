
// 0000      0000-05DC       -             -             IEEE802.3 Length Field         [Neil_Sembower]
pub const ETHERTYPE_LEN_FIELD_MIN: u16 = 0x0000;
pub const ETHERTYP_LEN_FIELD_MAX: u16 = 0x05dc;
// 0257      0101-01FF       -             -             Experimental                   [Neil_Sembower]
//                                                                                      [Boggs, D., J. Shoch, E. Taft, and R. Metcalfe, "PUP: An
//                                                                                      Internetwork Architecture", XEROX Palo Alto Research

// 2048      0800            513           1001          Internet Protocol version 4    [RFC7042]
//                                                       (IPv4)
pub const ETHERTYPE_IPV4: u16 = 0x0800;
// 2054      0806            -             -             Address Resolution Protocol    [RFC7042]
//                                                       (ARP)
pub const ETHERTYPE_ARP: u16 = 0x0806;

//           22F3                                        TRILL                          [RFC6325]
pub const ETHERTYPE_TRILL: u16 = 0x22f3;
//           22F4                                        L2-IS-IS                       [RFC6325]
pub const ETHERTYPE_L2_IS_IS: u16 = 0x22f4;

// 32821     8035            -             -             Reverse Address Resolution     [RFC903][Joseph_Murdock]
//                                                       Protocol (RARP)
pub const ETHERTYPE_RARP: u16 = 0x8035;

//                                                       Customer VLAN Tag Type (C-Tag,
// 33024     8100            -             -             formerly called the Q-Tag)     [RFC7042]
//                                                       (initially Wellfleet)
pub const ETHERTYPE_VLAN_TAG: u16 = 0x8100;

//           86DD                                        Internet Protocol version 6    [RFC7042]
//                                                       (IPv6)
pub const ETHERTYPE_IPV6: u16 = 0x86dd;

//           880B                                        Point-to-Point Protocol (PPP)  [RFC7042]
pub const ETHERTYPE_PPP: u16 = 0x880b;

//           8847                                        MPLS                           [RFC5332]
pub const ETHERTYPE_MPLS: u16 = 0x8847;
//           8848                                        MPLS with upstream-assigned    [RFC5332]
//                                                       label
pub const ETHERTYPE_MPLS_UPSTREAM_LABEL: u16 = 0x8848;
//           8861                                        Multicast Channel Allocation   [RFC7042]
//                                                       Protocol (MCAP)
// 34915     8863            -             -             PPP over Ethernet (PPPoE)      [RFC2516]
//                                                       Discovery Stage
pub const ETHERTYPE_PPPOE_DISCO: u16 = 0x8863;
// 34916     8864            -             -             PPP over Ethernet (PPPoE)      [RFC2516][RFC8822]
//                                                       Session Stage
pub const ETHERTYPE_PPPOE_SESS: u16 = 0x8864;
// 34958     888E            -             -             IEEE Std 802.1X - Port-based   [IEEE]
//                                                       network access control
pub const ETHERTYPE_8021X: u16 = 0x888e;
// 34984     88A8            -             -             IEEE Std 802.1Q - Service VLAN [IEEE]
//                                                       tag identifier (S-Tag)
pub const ETHERTYPE_VLAN_S_TAG: u16 = 0x88a8;

// 35020     88CC            -             -             IEEE Std 802.1AB - Link Layer  [IEEE]
//                                                       Discovery Protocol (LLDP)
pub const ETHERTYPE_LLDP: u16 = 0x88cc;
// 35045     88E5            -             -             IEEE Std 802.1AE - Media       [IEEE]
//                                                       Access Control Security
pub const ETHERTYPE_MACSEC: u16 = 0x88e5;
// 35047     88E7            -             -             Provider Backbone Bridging     [IEEE Std 802.1Q-2014]
//                                                       Instance tag
pub const ETHERTYPE_PBB_INST_TAG: u16 = 0x88e7;
//                                                       IEEE Std 802.1Q - Multiple
// 35061     88F5            -             -             VLAN Registration Protocol     [IEEE]
//                                                       (MVRP)
pub const ETHERTYPE_MVRP: u16 = 0x88f5;
//                                                       IEEE Std 802.1Q - Multiple
// 35062     88F6            -             -             Multicast Registration         [IEEE]
//                                                       Protocol (MMRP)
pub const ETHERTYPE_MMRP: u16 = 0x88f6;
// 35085     890D            -             -             IEEE Std 802.11 - Fast Roaming [IEEE]
//                                                       Remote Request (802.11r)
pub const ETHERTYPE_80211R: u16 = 0x8900;

// 35131     893B            -             -             TRILL Fine Grained Labeling    [RFC7172]
//                                                       (FGL)
pub const ETHERTYPE_TRILL_FGL: u16 = 0x893b;

// 35142     8946            -             -             TRILL RBridge Channel          [RFC7178]
pub const ETHERTYPE_TRILL_BR: u16 = 0x8946;

// 41197     A0ED            -             -             LoWPAN encapsulation           [RFC7973]
pub const ETHERTYPE_LOWPAN_ENCAP: u16 = 0xa0ed;
//                                                       The Ethertype will be used to
//                                                       identify a "Channel" in which
//                                                       control messages are
//                                                       encapsulated as payload of GRE
// 47082     B7EA            -             -             packets. When a GRE packet     [RFC8157]
//                                                       tagged with the Ethertype is
//                                                       received, the payload will be
//                                                       handed to the network
//                                                       processor for processing.
pub const ETHERTYPE_GRE_X: u16 = 0xb7ea;

// Ethertype                     ORGANIZATION / ADDRESS                       PROTOCO
// 8938                          HDBaseT Alliance                             HDBaseT Control and Management Protocol (www.hdbaset.org)
//                               3855 SW 153rd Drive
//                               Beaverton  OR  97006
//                               US
//
//
// 892E                          Ozmo Devices                                 A protocol to enable USB type data and USB type commands to be sent between
//                               2595 E. Bayshore Rd. Suite 100               two devices connected by an IEEE802 link, typically an IEEE802.11 link. The
//                               Palo Alto  CA  94303                         protocol allows encoding of all information that would normally be present
//                               US                                           in a wired USB request, such as request_id and rcode along with payload
//                                                                            data. Defined header structures allow correct delivery of the required
//                                                                            fields. The informaion provided through the protocol enables USB like
//                                                                            behaviour to be implemented between wirelessly connected devices.
//                                                                            www.ozmodevices.com

// 22F3                          IETF TRILL Working Group                     TRILL combine the advantages of bridges and routers and is the application
//                               c/o Internet Society                         of link state routing to the VLAN aware customer bridging problem. The
//                               Reston  VA  20190-5108                       TRILL protocol is described in the base protocol document at
//                               US                                           http://tools.ietf.org/id/draft-ietf-trill-rbridge-protocol-15.txt
//                                                                            The
//                                                                            final document can be found here: http://www.ietf.org/rfc/rfc6325.txt
//
//

// 88ce                          Level 5 Networks, Inc.                       Type Protocol:
//                               840 W. California Ave., STE 240
//                               Sunnyvale  CA  94086                         Remote Direct Memory Access over Ethernet (RDMAoE): a
//                               US                                           protocol for low-latency, low-overhead memory-to-memory communications
//                                                                            among hosts interconnected by Ethernet
//

// "22F0                          	IEEE 1722 Working Group                     IEEE Std. 1722-2016 Transport Protocol for Time-Sensitive Applications in"
//                               1722 Chair c/o IEEE                          Bridged Local Area Networks
//                               Piscataway  NJ  08854
//                               US
// 8808                          IEEE 802.3 Working Group                     MAC Control as defined in IEEE std 802.3
//                               IEEE 802.3 Chair, c/o RAC Administrator, IEEE
//                               Piscataway  NJ  08854
//                               US
//
//
// 88f5                          IEEE 802.1 Chair                             Multiple VLAN Registration Protocol (MVRP) defined in IEEE Std 802.1Q
//                                c/o RAC Administrator , IEEE
//                               Piscataway  NJ  08554
//                               US
//
//
// 88b6                          IEEE 802.1 Chair                             Local Experimental EtherType 2 as defined in IEEE Std 802.  This EtherType
//                                c/o RAC Administrator , IEEE                value is available for public use and for prototype and vendor-specific
//                               Piscataway  NJ  08554                        protocol development.
//                               US
//
//
// 88cc                          IEEE 802.1 Chair                             Link Layer Discovery Protocol (LLDP) defined in IEEE Std 802.1AB
//                                c/o RAC Administrator , IEEE
//                               Piscataway  NJ  08554
//                               US
//
//
// 22EA                          IEEE 802.1 Chair                             Stream Reservation Protocol (SRP) defined in IEEE Std 802.1Q
//                                c/o RAC Administrator , IEEE
//                               Piscataway  NJ  08554
//                               US
//
//
// 8940                          IEEE 802.1 Chair                             Edge Control Protocol (ECP) defined in IEEE Std 802.1Q for use with IEEE
//                                c/o RAC Administrator , IEEE                Std
//                               Piscataway  NJ  08554                        802.1BR
//                               US                                           http://www.ieee802.org/1/files/private/bg-drafts/d2/802-1qbg-d2-0.pdf
//
//                                                                            user:
//                                                                            p8021
//                                                                            pass: go_wildcats
//
//
// 8910                          IEEE 802.1 Working Group                     Encapsulated Addresses for use with the Backbone Service Instance as
//                               IEEE 802.1 Chair, c/o RAC Administrator IEEE defined in IEEE Std 802.1Q
//                               Piscataway  NJ  08854
//                               US
//
//
// 880B                          US Robotics Corporation                      PPP - IETF RFC 2637
//                               1300 E. Woodfield Roar, Suite: 506
//                               Schaumburg  IL  60173
//                               US
//
//

// 8932                          Mellanox Technologies, Inc.                  Mellanox discovery and configuration protocol.
//                               350 Oakmead Parkway, Suite 100
//                               Sunnyvale  CA  94085
//                               US

// 8946                          IETF TRILL Working Group                     The RBridge Channel protocol is specified
//                               c/o Internet Society                         in
//                               Reston  VA  20190-5108                       http://www.ietf.org/id/draft-ietf-trill-rbridge-channel-05.txt. Most of
//                               US                                           the
//                                                                            document is about communication between RBridges. Section 4
//                                                                            describes
//                                                                            the differences for transmission between and end station and an
//                                                                            RBridge.
//

// 22F1                          Association of Radio Industries and Businesses (ARIB)ROHC (Robust Header Compression) is an IP header compression protocol
//                               Nittochi Bldg. 11F,                          specified in IETF RFC 3095 &quot;RObust Header Compression (ROHC):
//                               Tokyo    100-0013                            Framework and four profiles: RTP, UDP, ESP, and uncompressed&quot;. The
//                               JP                                           specification is available at http://www.ietf.org/rfc/rfc3095.txt.
//

// 88f7                          IEEE I&M Society TC9                         The EtherType field assignment is to be used in the revised IEEE 1588,
//                               100 Bureau Drive                             Standard for a Precision Clock Synchronization Protocol for Networked
//                               Gaithersburg  MD  20899-8220                 Measurement and Control Systems.
//                               US
//                                                                            The URL for the standard activities
//                                                                            is: http://ieee1588.nist.gov
//

// 88b5                          IEEE 802.1 Chair                             Local Experimental EtherType 1 as defined in IEEE Std 802.  This EtherType
//                                c/o RAC Administrator , IEEE                value is available for public use and for prototype and vendor-specific
//                               Piscataway  NJ  08554                        protocol development.
//                               US
//
//
// 894B                          IEEE 802.1 Chair                             flow filtering tag (F-TAG) as defined in IEEE Std 802.1Q
//                                c/o RAC Administrator , IEEE
//                               Piscataway  NJ  08554
//                               US
//
//
// 8929                          IEEE 802.1 Chair                             Multiple I-SID RegistrationProtocol (MIRP) as defined in IEEE Std 802.1Q
//                                c/o RAC Administrator , IEEE
//                               Piscataway  NJ  08554
//                               US
//

// 890d                          IEEE 802.11 Working Group                    Management protocol as defined in IEEE Std 802.11
//                               c/o RAC Administrator
//                               Piscataway   NJ  08854
//                               US
//
//
// 8870                          IEEE 802.1 Working Group                     LLC encapsulation as defined by IEEE Std 802.1AC.
//                                c/o RAC Administrator , IEEE
//                               Piscataway  NJ  08554
//                               US

// 8915                          Mellanox Technologies, Inc.                  RoCE - RDMA over Converged Ethernet
//                               350 Oakmead Parkway, Suite 100
//                               Sunnyvale  CA  94085
//                               US
//
//
// 88e3                          SIEMENS AG                                   MRP (medium redundancy protocol)
//                               Oestliche RheinbrÃ¼ckenstraÃŸe 50
//                               Karlsruhe  Baden-WÃ¼rttemberg  76181
//                               DE
//
//

// 8137                          Novell, Inc.                                 Internetwork Packet Exchange (IPX)
//                               122 EAST 1700 SOUTH
//                               Provo  UT   84606
//                               US
//
//
// 8138                          Novell, Inc.                                 Internetwork Packet Exchange (IPX)
//                               122 EAST 1700 SOUTH
//                               Provo  UT   84606
//                               US
//
//

// 8948                          Mellanox Technologies, Inc.                  EoIB: Ethernet over InfiniBand Protocol
//                               350 Oakmead Parkway, Suite 100
//                               Sunnyvale  CA  94085
//                               US

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
/
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
/
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
