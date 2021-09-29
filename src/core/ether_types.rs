/*
 * @file
 * IEEE assigned numbers
 *
 * @defgroup ieee IEEE assigned numbers
 * @ingroup infrastructure
 */

/*
 * Copyright (c) 2017 Dirk Ziegelmeier.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without modification,
 * are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice,
 *    this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 * 3. The name of the author may not be used to endorse or promote products
 *    derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR IMPLIED
 * WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT
 * SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT
 * OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING
 * IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY
 * OF SUCH DAMAGE.
 *
 * This file is part of the lwIP TCP/IP stack.
 *
 * Author: Dirk Ziegelmeier <dziegel@gmx.de>
 *
 */

// #define LWIP_HDR_PROT_IEEE_H

/*
 * @ingroup ieee
 * A list of often ethtypes (although lwIP does not use all of them).
 */
enum EtherType {
    // values between 0x0000 and 0x05DC imply an IEEE 802.3 type packet and are the length field
    // values between 0x0101 and 0x01FF are for experimental use
    // Xerox PUP
    XeroxPup = 0x0200,
    // Xerox PUP Addr Trans
    XeroxPupAddrTrans = 0x0201,
    // 0x0202 -- 0x03FF are unused
    // Nixdorf
    Nixdorf = 0x0400,
    // XEROX NS IDP
    XeroxNsIdp = 0x0600,
    // DLOG
    DLOG1 = 0x0660,
    // DLOG
    DLOG2 = 0x0661,
    // Internet protocol v4
    IPv4 = 0x0800,
    // X.75 Internet
    X75Internet = 0x0801,
    // NBS Internet
    NbsInternet = 0x0802,
    // ECMA Internet
    EcmaInternet = 0x0803,
    // Chaosnet
    Chaosnet = 0x0804,
    // X.25 Level 3
    X25Level3 = 0x0805,
    //  Address resolution protocol 
    ARP = 0x0806,
    // XNS Compatibility
    XnsCompat = 0x0807,
    // Frame Relay ARP
    FrameRelayArp = 0x0808,
    // Symbolics Private
    SymbolicsPrivate = 0x081C,
    // Xyplex 0x0888 = 0x088A
    //  Wake on lan 
    // ETHTYPE_WOL = 0x0842,
    //  RARP 
    // ETHTYPE_RARP = 0x8035,
    //  Virtual local area network 
    UngermannBassNetDebug = 0x0900,
    // Xerox IEEE 802.3 PUP
    XeroxIeee8023Pup = 0x0A00,
    // PUP Addr Trans
    PupAddrTrans = 0x0A01,
    // BanyanVines
    BanyanVines = 0x0BAD,
    // VINES Loopback
    VinesLoopback = 0x0BAE,
    // VINES Echo
    VinesEcho = 0x0BAF,
    // Berkeley Trailer nego
    BerkeleyTrailerNego = 0x1000,
    // Berkeley Trailer Encap IP 0x1001 - 0x100F
    // Valid Systems
    ValidSystems = 0x1600,
    // TRILL RFC6325
    Trill = 0x22F3,
    // L2-IS-IS RFC6325
    L2ISIS = 0x22F4,
    // PCS Basic Block Protocol
    PcsBasicBlock = 0x4242,
    // BBN Simnet
    BbnSimnet = 0x5208,
    // DEC Unassigned
    DecUnassigned2 = 0x6000,
    // DEC MOP Dump/Load
    DecMopDumpLoad = 0x6001,
    // DEC MOP Remote Console
    DecMopRemoteConsole = 0x6002,
    // DEC DECNET Phase IV Route
    DecDecNetPhase4Route = 0x6003,
    // DEC LAT
    DecLat = 0x6004,
    // DEC Diagnostic Protocol
    DecDiagProto = 0x6005,
    // DEC Customer Protocol
    DecCustomerProto = 0x6006,
    // DEC LAVC SCA
    DecLavcSca = 0x6007,
    // DEC Unassigned 0x6008 - 0x6009
    // 3Com Reserved 0x6010 - 0x6014
    // Trans Ether Bridging RFC1701
    TransEtherBridging = 0x6558,
    // Raw Frame Relay RFC1701
    RawFrameRelay = 0x6559,
    // Ungermann-Bass download
    UngermannBassDownload = 0x7000,
    // Ungermann-Bass dia/loop
    UngermannBassDiaLoop = 0x7002,
    // LRT 0x7020-0x7029
    Proteon = 0x7030,
    Cabletron = 0x7034,
    // Cronus VLN RFC824
    CronusVln = 0x8003,
    // Cronus Direct
    CronusDirect = 0x8004,
    HpProbe = 0x8005,
    Nestar = 0x8006,
    ATT = 0x8008,
    Excelan = 0x8010,
    // SGI diagnostics
    SgiDiagnostics = 0x8013,
    // SGI network games
    SgiNetGames = 0x8014,
    // SGI reserved
    SgiReserved = 0x8015,
    // SGI bounce server
    SgiBounceServer = 0x8016,
    ApolloDomain = 0x8019,
    Tymshare = 0x802E,
    Tigan = 0x802F,
    RevArp = 0x8035,
    AeonicSystems = 0x8036,
    // DEC LANBridge
    DecLanBridge = 0x8038,
    // DEC Unassigned 0x8039-0x803C
    // DEC Ethernet Encryption
    DecEthernetEncryption = 0x803D,
    // DEC Unassigned
    DecUnassigned = 0x803E,
    // DEC LAN Traffic Monitor
    DecLanTrafficMonitor = 0x803F,
    // DEC Unassigned 0x8040-0x8042
    // Planning Research Corp
    PlanningResearchCorp = 0x8044,
    ATT2 = 0x8046,
    ATT3 = 0x8047,
    ExperData = 0x8049,
    StanfordVKernelExp = 0x805B,
    StanfordVKernelProd = 0x805C,
    LittleMachines = 0x8060,
    CounterpointComputers = 0x8062,
    // Univ of Mass Amherst 0x8065-0x8066
    VeecoIntegratedAuto = 0x8067,
    GeneralDynamics = 0x8068,
    ATT4 = 0x8069,
    Autophon = 0x806A,
    ComDesign = 0x806C,
    ComputgraphicCorp = 0x806D,
    // Landmark Graphics Corp 0x806E - 0x8077
    Matra = 0x807A,
    DanskDataElektronik = 0x807B,
    MeritInternodal = 0x807C,
    // Vitalink Communications 0x807D-0x807F
    VitalinkTransLan3 = 0x8080,
    // Counterpoint Computers 0x8081 - 0x8083
    AppleTalk = 0x809B,
    // Datability 0x809C-0x809E
    SpiderSystemsLtd = 0x809F,
    NixdorfComputers = 0x80A3,
    // Siemens Gammasonics 0x80A4 - 0x80B3
    // DCA Data Exchange Cluster 0x80C0 - 0x80C3
    BanyanSystems1 = 0x80C4,
    BanyanSystems2 = 0x80C5,
    PacerSoftware = 0x80C6,
    ApplitekCorporation = 0x80C7,
    // Intergraph Corporation 0x80C8-0x80CC
    // Harris Corporation 0x80CD-0x80CE
    // Taylor Instrument 0x80CF-0x80D2
    // Rosemount Corporation 0x80D3-0x80D4
    // IBM SNA Service on Ether
    IbmSnaSvcEther = 0x80D5,
    VarianAssociates = 0x80DD,
    // Integrated Solutions TRFS 0x80DE-0x80DF
    // Allen-Bradley 0x80E0-0x80E3
    // Datability 0x80E4-0x80F0
    Retix = 0x80F2,
    AppleTalkAARP = 0x80F3,
    // Kinetics 0x80F4-0x80F5
    ApolloComputer = 0x80F7,
    WellfleetCommunications = 0x80FF,
    Vlan = 0x8100,
    // Wellfleet Communications 0x8101-0x8103
    // Symbolics Private 0x8107-0x8109
    HayesMicrocomputers = 0x8130,
    VgLabratorySystems = 0x8131,
    // Bridge Communications 0x8132-0x8136
    // Novell Inc 0x8137-0x8138
    // KTI 0x8139-0x813D
    Logicraft = 0x8148,
    NetworkComputingDevices = 0x8149,
    AlphaMicro = 0x814A,
    Snmp = 0x814C,
    Biin1 = 0x814D,
    Biin2 = 0x814E,
    TechnicallyEliteConcept = 0x814F,
    RationalCorp = 0x8150,
    // Qualcomm 0x8151-0x8153
    // Computer Protocol Pty Ltd 0x815C-0x15E
    // Charles River Data System 0x8164-0x8166
    Xtp = 0x817D,
    SgiTimeWarner = 0x817E,
    // HIPPI-FP Encap
    HippiFpEncap = 0x8180,
    // STP, HIPPI-ST
    StpHippiSt = 0x8181,
    // Reserved for HIPPI-6400
    RsrvdHippi6400 = 0x8182,
    // Reserved for HIPPI-6400
    RsrvdHippi64002 = 0x8183,
    // SGI 0x8184-0x818C
    MotorolaComputer = 0x818D,
    // Qualcomm 0x819A-0x81A3
    // Arai Bunkichi
    AraiBunkichi = 0x81A4,
    // RAD Network Devices 0x81A5-0x81AE
    // Xyplex 0x81B7-0x81B9
    // Apricot Computers 0x81CC-0x81D5
    // Artisoft 0x81D6-0x81DD
    // Polygon 0x81E6-0x81EF
    // Comsat Labs 0x81F0-0x81F2
    // SAIC 0x81F3-0x81F5
    // VG Analytical 0x81F6-0x81F8
    // Quantum Software 0x8203-0x8205,
    // Ascom Banking Systems 0x8221-0x8222,
    // Advanced Encryption System 0x823E-0x240
    // Charles River Data System 0x8263-0x826A
    // Athena Programming 0x827F-0x8282
    // Inst Ind Info Tech 0x829A-0x829B
    // Taurus Controls 0x829C-0x82AB
    // Walker Richer & Quinn 0x82AC-0x8693
    // Idea Courier 0x8694-0x869D
    // Computer Network Tech 0x869E-0x86A1
    // Gateway Communications 0x86A3-0x86AC
    // SECTRA
    Sectra = 0x86DB,
    //  Internet protocol v6 RFC7042
    Ipv6 = 0x86DD,
    DeltaControls = 0x86DE,
    // ATOMIC
    Atomic = 0x86DF,
    // Landis & Gyr Powers 0x86E0-0x86EF
    // Motorola 0x8700-0x8710
    // TCP/IP Compression RFC1144    RFC1701
    TcpIpComp = 0x876B,
    // IP Autonomous Systems RFC1701
    IpAutoSys = 0x876C,
    // Secure Data RFC1701
    SecureData = 0x876D,
    // IEEE Std 802.3 Ethernet Passive Optical Net RFC7042
    Ieee8023EPON = 0x8808,
    // Point to Point Protocol RFC7042
    PPP = 0x880B,
    // General Switch Management Protocol
    GenSwMgmtProto = 0x880C,
    // Ethernet NIC hardware and software testing
    EtherNicHwSwTest = 0x8822,
    // MPLS
    Mpls = 0x8847,
    // MPLS with upstream-assigned label
    MplsUpstreamLabel = 0x8848,
    // Multicast Channel Allocation Protocol
    McastChanAllocProto = 0x8861,
    //  PPP Over Ethernet Discovery Stage 
    PppoeDisc = 0x8863,
    //  PPP Over Ethernet Session Stage 
    PppoeSession = 0x8864,
    // IEEE Std 802.1X port based access control
    Ieee8021XPortBasedAccessCon = 0x888E,
    // IEEE Std 802.1Q Service VLAN Tag ID S-Tag
    Ieee8021QSvcVlanSTag = 0x88A8,
    // IEEE Std 802 Local Experimental Ether Type
    Ieee802LocalExp = 0x88B5,
    // IEEE Std 802 Local Experimental Ether Type 2
    Ieee802LocalExp2 = 0x88B6,
    // IEEE Std 802 OUI Extended Ethertype
    Ieee802OuiExt = 0x88B7,
    // IEEE Std 802.11 Pre-Auth
    Ieee80211PreAuth = 0x88C7,
    // IEEE Std 802.1AB LLDP
    Ieee8021ABLLDP = 0x88CC,
    // IEEE Std 802.1AE Media Access Control Security
    Ieee8021AEMACSEC = 0x88E5,
    // Provider Backbone Bridging Instance Tag IEEE 802.1Q-2014
    ProvBackBrdgInstTag = 0x88E7,
    // IEEE Std 802.1Q Multi VLAN Registration Protocol
    Ieee8021QMVRP = 0x88F5,
    // IEEE Std 802.1Q Multiple Multicast Registration Protocol
    Ieee8021QMMRP = 0x88F6,
    // IEEE Std 802.11 Fast Roaming Remote Request 802.11r
    Ieee80211FastRoamRemReq = 0x890D,
    // IEEE Std 802.21 Media Independent Handover Protocol
    Ieee80221MediaIndepHandoverProto = 0x8917,
    // IEEE Std 802.1Qbe Multiple I-SID Registration Protocol
    Ieee8021Qbe = 0x8929,
    // Trill Fine Grained Labeling (FGL) RFC7172
    TrillFGL = 0x893B,
    // IEEE Std 802.1Qbg - ECP Protocol
    Ieee8021QbgEcp = 0x8940,
    // TRILL RBridge Channel
    TrillRBridgeChann = 0x8946,
    // Geo Networking per ETSI EN 302 636-4-1
    GeoNet = 0x8947,
    // Network Service Header
    NetSvcHdr = 0x894F,
    // Invisible Software 0x8A96-0x8A97
    Loopback = 0x9000,
    // 3COM Bridge XNS Sys Mgmt
    X3comBridgeXnsSysMgmt = 0x9001,
    // 3COM Bridge TCP-IP Sys
    X3comBridgeTcpIpSys = 0x9002,
    // 3COM Bridge Loop Detect
    X3comBridgeLoopDetect = 0x9003,
    // Multi-Topology
    MultiTopology = 0x9A22,
    // LoWPAN encapsulation
    LowpanEncap = 0xA0ED,
    // Ethertype used to identify a channel encapsulated in payload of GRE
    GreChannel = 0xB7EA,
    // BBN VITAL - LanBridge cache
    BbnVitalLanBridgeCache = 0xFF00,
    // ISC Bunker Ramo 0xFF00-0xFF0F
    Reserved = 0xFFFF,
    //  Jumbo Frames 
    Jumbo = 0x8870,
    //  Process field network 
    Profinet = 0x8892,
    //  Ethernet for control automation technology 
    Ethercat = 0x88A4,
    //  Link layer discovery protocol 
    // ETHTYPE_LLDP = 0x88CC,
    //  Serial real-time communication system 
    Sercos = 0x88CD,
    //  Media redundancy protocol 
    MediaRedundancyProto = 0x88E3,
    //  Precision time protocol 
    PrecisTimeProto = 0x88F7,
    //  Q-in-Q, 802.1ad 
    VlanQinQ = 0x9100,
}
