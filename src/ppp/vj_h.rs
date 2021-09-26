/*
 * Definitions for tcp compression routines.
 *
 * $Id: vj.h,v 1.7 2010/02/22 17:52:09 goldsimon Exp $
 *
 * Copyright (c) 1989 Regents of the University of California.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms are permitted
 * provided that the above copyright notice and this paragraph are
 * duplicated in all such forms and that any documentation,
 * advertising materials, and other materials related to such
 * distribution and use acknowledge that the software was developed
 * by the University of California, Berkeley.  The name of the
 * University may not be used to endorse or promote products derived
 * from this software without specific prior written permission.
 * THIS SOFTWARE IS PROVIDED ``AS IS'' AND WITHOUT ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, WITHOUT LIMITATION, THE IMPLIED
 * WARRANTIES OF MERCHANTIBILITY AND FITNESS FOR A PARTICULAR PURPOSE.
 *
 * Van Jacobson (van@helios.ee.lbl.gov), Dec 31, 1989:
 * - Initial distribution.
 */

// #define VJ_H

pub const MAX_SLOTS: u32 = 16; /* must be > 2 and < 256 */

pub const MAX_SLOTS: u32 = 16;
pub const MAX_HDR: u32 = 128;

/*
 * Compressed packet format:
 *
 * The first octet contains the packet type (top 3 bits), TCP
 * 'push' bit, and flags that indicate which of the 4 TCP sequence
 * numbers have changed (bottom 5 bits).  The next octet is a
 * conversation number that associates a saved IP/TCP header with
 * the compressed packet.  The next two octets are the TCP checksum
 * from the original datagram.  The next 0 to 15 octets are
 * sequence number changes, one change per bit set in the header
 * (there may be no changes and there are two special cases where
 * the receiver implicitly knows what changed -- see below).
 *
 * There are 5 numbers which can change (they are always inserted
 * in the following order): TCP urgent pointer, window,
 * acknowlegement, sequence number and IP ID.  (The urgent pointer
 * is different from the others in that its value is sent, not the
 * change in value.)  Since typical use of SLIP links is biased
 * toward small packets (see comments on MTU/MSS below), changes
 * use a variable length coding with one octet for numbers in the
 * range 1 - 255 and 3 octets (0, MSB, LSB) for numbers in the
 * range 256 - 65535 or 0.  (If the change in sequence number or
 * ack is more than 65535, an uncompressed packet is sent.)
 */

/*
 * Packet types (must not conflict with IP protocol version)
 *
 * The top nibble of the first octet is the packet type.  There are
 * three possible types: IP (not proto TCP or tcp with one of the
 * control flags set); uncompressed TCP (a normal IP/TCP packet but
 * with the 8-bit protocol field replaced by an 8-bit connection id --
 * this type of packet syncs the sender & receiver); and compressed
 * TCP (described above).
 *
 * LSB of 4-bit field is TCP "PUSH" bit (a worthless anachronism) and
 * is logically part of the 4-bit "changes" field that follows.  Top
 * three bits are actual packet type.  For backward compatibility
 * and in the interest of conserving bits, numbers are chosen so the
 * IP protocol version number (4) which normally appears in this nibble
 * means "IP packet".
 */

/* packet types */
pub const TYPE_IP: u32 = 0x40;
pub const TYPE_IP: u32 = 0x40;
pub const TYPE_IP: u32 = 0x40;
pub const TYPE_IP: u32 = 0x40;
pub const TYPE_UNCOMPRESSED_TCP: u32 = 0x70;
pub const TYPE_COMPRESSED_TCP: u32 = 0x80;
pub const TYPE_ERROR: u32 = 0x00;

/* Bits in first octet of compressed packet */
pub const NEW_C: u32 = 0x40; /* flag bits for what changed in a packet */
pub const NEW_I: u32 = 0x20;
pub const NEW_S: u32 = 0x08;
pub const NEW_A: u32 = 0x04;
pub const NEW_W: u32 = 0x02;
pub const NEW_U: u32 = 0x01;

/* reserved, special-case values of above */
pub const SPECIAL_I: u32 = (NEW_S | NEW_W | NEW_U); /* echoed interactive traffic */
pub const SPECIAL_D: u32 = (NEW_S | NEW_A | NEW_W | NEW_U); /* unidirectional data */
pub const SPECIALS_MASK: u32 = (NEW_S | NEW_A | NEW_W | NEW_U);

pub const TCP_PUSH_BIT: u32 = 0x10;

/*
 * "state" data for each active tcp conversation on the wire.  This is
 * basically a copy of the entire IP/TCP header from the last packet
 * we saw from the conversation together with a small identifier
 * the transmit & receive ends of the line use to locate saved header.
 */
pub struct cstate {
    // let mut cs_next: &mut cstate; /* next most recently used state (xmit only) */
    pub cs_hlen: u16, /* size of hdr (receive only) */
    pub cs_id: u8,    /* connection # associated with this state */
    pub cs_filler: u8,
    pub csu_hdr: String,
    pub csu_ip: ip_hdr,
}
// #define cs_ip vjcs_u.csu_ip
// #define cs_hdr vjcs_u.csu_hdr

pub struct vjstat {
    pub vjs_packets: u32,        /* outbound packets */
    pub vjs_compressed: u32,     /* outbound compressed packets */
    pub vjs_searches: u32,       /* searches for connection state */
    pub vjs_misses: u32,         /* times couldn't find conn. state */
    pub vjs_uncompressedin: u32, /* inbound uncompressed packets */
    pub vjs_compressedin: u32,   /* inbound compressed packets */
    pub vjs_errorin: u32,        /* inbound unknown type packets */
    pub vjs_tossed: u32,         /* inbound packets tossed because of error */
}

/*
 * all the state data for one serial line (we need one of these per line).
 */
pub struct vjcompress {
    pub last_cs: cstate, /* most recently used tstate */
    pub last_recv: u8,   /* last rcvd conn. id */
    pub last_xmit: u8,   /* last sent conn. id */
    pub flags: u16,
    pub maxSlotIndex: u8,
    pub compressSlot: u8, /* Flag indicating OK to compress slot ID. */
    pub stats: vjstat,
    pub tstate: Vec<cstate>, /* xmit connection states */
    pub rstate: Vec<cstate>, /* receive connection states */
}

/* flag values */
pub const VJF_TOSS: u32 = 1; /* tossing rcvd frames because of input err */

// extern void  vj_compress_init    (comp: &mut vjcompress);
// extern vj_compress_tcp: u8     (comp: &mut vjcompress, pb: &mut Vec<PacketBuffer>);
// extern void  vj_uncompress_err   (comp: &mut vjcompress);
// extern int   vj_uncompress_uncomp(nb: &mut PacketBuffer, comp: &mut vjcompress);
// extern int   vj_uncompress_tcp   (nb: &mut Vec<PacketBuffer>, comp: &mut vjcompress);
