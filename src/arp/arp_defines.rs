//  Number   Operation Code (op)           References
//    0     Reserved               [RFC5494]
pub const ARP_OP_CODE_RES_LO: u16 = 0;
//    1     REQUEST                [RFC826][RFC5227]
pub const ARP_OP_CODE_REQ: u16 = 1;
//    2     REPLY                  [RFC826][RFC5227]
pub const ARP_OP_CODE_REPLY: u16 = 2;
//    3     request Reverse        [RFC903]
pub const RARP_OP_CODE_REQ: u16 = 3;
//    4     reply Reverse          [RFC903]
pub const RARP_OP_CODE_REPLY: u16 = 4;
//    5     DRARP-Request          [RFC1931]
pub const DRARP_OP_CODE_REQ: u16 = 5;
//    6     DRARP-Reply            [RFC1931]
pub const DRARP_OP_CODE_REPLY: u16 = 6;
//    7     DRARP-Error            [RFC1931]
pub const DRARP_OP_CODE_ERR: u16 = 7;
//    8     InARP-Request          [RFC2390]
pub const INARP_OP_CODE_REQ: u16 = 8;
//    9     InARP-Reply            [RFC2390]
pub const INARP_OP_CODE_REPLY: u16 = 9;
//    10    ARP-NAK                [RFC1577]
pub const ARP_OP_CODE_NAK: u16 = 10;
//    11    MARS-Request           [Grenville_Armitage]
//    12    MARS-Multi             [Grenville_Armitage]
//    13    MARS-MServ             [Grenville_Armitage]
//    14    MARS-Join              [Grenville_Armitage]
//    15    MARS-Leave             [Grenville_Armitage]
//    16    MARS-NAK               [Grenville_Armitage]
//    17    MARS-Unserv            [Grenville_Armitage]
//    18    MARS-SJoin             [Grenville_Armitage]
//    19    MARS-SLeave            [Grenville_Armitage]
//    20    MARS-Grouplist-Request [Grenville_Armitage]
//    21    MARS-Grouplist-Reply   [Grenville_Armitage]
//    22    MARS-Redirect-Map      [Grenville_Armitage]
//    23    MAPOS-UNARP            [Mitsuru_Maruyama][RFC2176]
//    24    OP_EXP1                [RFC5494]
pub const ARP_OP_CODE_EXP1: u16 = 24;
//    25    OP_EXP2                [RFC5494]
pub const ARP_OP_CODE_EXP2: u16 = 25;
// 26-65534 Unassigned
pub const ARP_OP_CODE_UNASSIGNED_START: u16 = 26;
pub const ARP_OP_CODE_UNASSIGNED_END: u16 = 0xfffe;
//  65535   Reserved               [RFC5494]
pub const ARP_OP_CODE_RES_HI: u16 = 0xffff;

//    0	Reserved	[RFC5494]
pub const ARP_HWTYPE_RES_LO: u16 = 0;
// 1	Ethernet (10Mb)	[Jon_Postel]
pub const ARP_HWTYPE_ETH: u16 = 1;
// 2	Experimental Ethernet (3Mb)	[Jon_Postel]
pub const ARP_HWTYPE_EXP_ETH: u16 = 2;
// 3	Amateur Radio AX.25	[Philip_Koch]
// 4	Proteon ProNET Token Ring	[Avri_Doria]
// 5	Chaos	[Gill_Pratt]
// 6	IEEE 802 Networks	[Jon_Postel]
pub const ARP_HWTYPE_IEEE_802: u16 = 6;
// 7	ARCNET	[RFC1201]
// 8	Hyperchannel	[Jon_Postel]
// 9	Lanstar	[Tom_Unger]
// 10	Autonet Short Address	[Mike_Burrows]
// 11	LocalTalk	[Joyce_K_Reynolds]
// 12	LocalNet (IBM PCNet or SYTEK LocalNET)	[Joseph Murdock]
// 13	Ultra link	[Rajiv_Dhingra]
// 14	SMDS	[George_Clapp]
// 15	Frame Relay	[Andy_Malis]
// 16	Asynchronous Transmission Mode (ATM)	[[JXB2]]
// 17	HDLC	[Jon_Postel]
// 18	Fibre Channel	[RFC4338]
pub const ARP_HWTYPE_FIBRE_CHAN: u16 = 18;
// 19	Asynchronous Transmission Mode (ATM)	[RFC2225]
// 20	Serial Line	[Jon_Postel]
pub const ARP_HWTYPE_SERIAL: u16 = 20;
// 21	Asynchronous Transmission Mode (ATM)	[Mike_Burrows]
// 22	MIL-STD-188-220	[Herb_Jensen]
// 23	Metricom	[Jonathan_Stone]
// 24	IEEE 1394.1995	[Myron_Hattig]
// 25	MAPOS	[Mitsuru_Maruyama][RFC2176]
// 26	Twinaxial	[Marion_Pitts]
// 27	EUI-64	[Kenji_Fujisawa]
// 28	HIPARP	[Jean_Michel_Pittet]
// 29	IP and ARP over ISO 7816-3	[Scott_Guthery]
// 30	ARPSec	[Jerome_Etienne]
// 31	IPsec tunnel	[RFC3456]
pub const ARP_HWTYPE_IPSEC: u16 = 31;
// 32	InfiniBand (TM)	[RFC4391]
pub const ARP_HWTYPE_INFINIBAND: u16 = 32;
// 33	TIA-102 Project 25 Common Air Interface (CAI)	[Jeff Anderson, Telecommunications Industry of America (TIA) TR-8.5 Formulating Group, <cja015&motorola.com>, June 2004]
// 34	Wiegand Interface	[Scott_Guthery_2]
// 35	Pure IP	[Inaky_Perez-Gonzalez]
// 36	HW_EXP1	[RFC5494]
// 37	HFI	[Tseng-Hui_Lin]
pub const ARP_HWTYPE_UNASSIGNED_1_LOW: u16 = 38;
pub const ARP_HWTYPE_UNASSIGNED_1_HI: u16 = 255;
// 38-255	Unassigned
// 256	HW_EXP2	[RFC5494]
// 257	AEthernet	[Geoffroy_Gramaize]
// 258-65534	Unassigned
pub const ARP_HWTYPE_UNASSIGNED_2_LO: u16 = 258;
pub const ARP_HWTYPE_UNASSIGNED_2_HI: u16 = 65534;

// 65535	Reserved	[RFC5494]
