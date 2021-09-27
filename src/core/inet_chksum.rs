use crate::packetbuffer::pbuf_h::PacketBuffer;
use crate::core::context::LwipContext;
use crate::core::defines::LwipAddr;

pub fn lwip_standard_chksum(dataptr: &Vec<u8>, mut len: usize) {
    //! Checksum
    let acc: u32;
    let src: u16;
    let octetptr: &mut Vec<u8>;

    acc = 0;
    /* dataptr may be at odd or even addresses */
    octetptr = dataptr;
    while len > 1 {
        /* declare first octet as most significant
        thus assume network order, ignoring host order */
        src = (*octetptr) << 8;
        octetptr += 1;
        /* declare second octet as least significant */
        src |= (*octetptr);
        octetptr += 1;
        acc += src;
        len -= 2;
    }
    if len > 0 {
        /* accumulate remaining octet */
        src = (*octetptr) << 8;
        acc += src;
    }
    /* add deferred carry bits */
    acc = (acc >> 16) + (acc & 0x0000ffff);
    if (acc & 0xffff0000) != 0 {
        acc = (acc >> 16) + (acc & 0x0000ffff);
    }
    /* This maybe a little confusing: reorder sum using lwip_htons()
    instead of lwip_ntohs() since it has a little less call overhead.
    The caller must invert bits for Internet sum ! */
    return lwip_htons(acc);
}





/* Parts of the pseudo checksum which are common to IPv4 and IPv6 */
pub fn inet_cksum_pseudo_base(
    p: &mut PacketBuffer,
    proto: u8,
    proto_len: u16,
    acc: &mut u32
) -> u16 {
    let q: &mut PacketBuffer;
    let swapped: i32 = 0;

    /* iterate through all pbuf in chain */
    // for (q = p; q != NULL; q = q.next) {
    //   LWIP_DEBUGF(INET_DEBUG, ("inet_chksum_pseudo(): checksumming pbuf %p (has next %p) \n",
    //                            q, q.next));
    //   acc += LWIP_CHKSUM(q.payload, q.len);
    //   /*LWIP_DEBUGF(INET_DEBUG, ("inet_chksum_pseudo(): unwrapped lwip_chksum()=%"X32_F" \n", acc));*/
    //   /* just executing this next line is probably faster that the if statement needed
    //      to check whether we really need to execute it, and does no harm */
    //   acc = fold_u32t(acc);
    //   if (q.len % 2 != 0) {
    //     swapped = !swapped;
    //     acc = swap_bytes_in_word(acc);
    //   }
    //   /*LWIP_DEBUGF(INET_DEBUG, ("inet_chksum_pseudo(): wrapped lwip_chksum()=%"X32_F" \n", acc));*/
    // }

    if swapped {
        *acc = SWAP_BYTES_IN_WORD(acc);
    }

    *acc += proto::to_le();
    *acc += proto_len::to_le();

    /* Fold 32-bit sum to 16 bits
    calling this twice is probably faster than if statements... */
    *acc = fold_u32(*acc);
    *acc = fold_u32(*acc);
    // LWIP_DEBUGF(INET_DEBUG, ("inet_chksum_pseudo(): PacketBuffer chain lwip_chksum()=%"X32_F"\n", acc));
    return !(acc & 0xffff);
}

/* inet_chksum_pseudo:
 *
 * Calculates the IPv4 pseudo Internet checksum used by TCP and UDP for a pbuf chain.
 * IP addresses are expected to be in network byte order.
 *
 * @param p chain of pbufs over that a checksum should be calculated (ip data part)
 * @param src source ip address (used for checksum of pseudo header)
 * @param dst destination ip address (used for checksum of pseudo header)
 * @param proto ip protocol (used for checksum of pseudo header)
 * @param proto_len length of the ip data part (used for checksum of pseudo header)
 * @return checksum (as u16) to be saved directly in the protocol header
 */
pub fn inet_chksum_pseudo(
    p: &mut PacketBuffer,
    proto: u8,
    proto_len: u16,
    src: &mut ip4_addr,
    dest: &mut ip4_addr,
) {
    let acc: u32;
    let addr: u32;

    addr = ip4_addr_get_u32(src);
    acc = (addr & 0xffff);
    acc = (acc + ((addr >> 16) & 0xffff));
    addr = ip4_addr_get_u32(dest);
    acc = (acc + (addr & 0xffff));
    acc = (acc + ((addr >> 16) & 0xffff));
    /* fold down to 16 bits */
    acc = fold_u32(acc);
    acc = fold_u32(acc);

    return inet_cksum_pseudo_base(p, proto, proto_len, acc);
}

/*
 * Calculates the checksum with IPv6 pseudo header used by TCP and UDP for a pbuf chain.
 * IPv6 addresses are expected to be in network byte order.
 *
 * @param p chain of pbufs over that a checksum should be calculated (ip data part)
 * @param proto ipv6 protocol/next header (used for checksum of pseudo header)
 * @param proto_len length of the ipv6 payload (used for checksum of pseudo header)
 * @param src source ipv6 address (used for checksum of pseudo header)
 * @param dest destination ipv6 address (used for checksum of pseudo header)
 * @return checksum (as u16) to be saved directly in the protocol header
 */
pub fn ip6_chksum_pseudo(
    p: &mut PacketBuffer,
    proto: u8,
    proto_len: u16,
    src: &mut ip6_addr_t,
    dest: &mut ip6_addr_t,
) {
    let acc: u32 = 0;
    let addr: u32;
    let addr_part: u8;

    // for (addr_part = 0; addr_part < 4; addr_part+= 1) {
    //   addr = src.addr[addr_part];
    //   acc = (acc + (addr & 0xffff));
    //   acc = (acc + ((addr >> 16) & 0xffff));
    //   addr = dest.addr[addr_part];
    //   acc = (acc + (addr & 0xffff));
    //   acc = (acc + ((addr >> 16) & 0xffff));
    // }
    /* fold down to 16 bits */
    *acc = fold_u32(acc);
    *acc = fold_u32(acc);

    return inet_cksum_pseudo_base(p, proto, proto_len, acc);
}

/* ip_chksum_pseudo:
 *
 * Calculates the IPv4 or IPv6 pseudo Internet checksum used by TCP and UDP for a pbuf chain.
 * IP addresses are expected to be in network byte order.
 *
 * @param p chain of pbufs over that a checksum should be calculated (ip data part)
 * @param src source ip address (used for checksum of pseudo header)
 * @param dst destination ip address (used for checksum of pseudo header)
 * @param proto ip protocol (used for checksum of pseudo header)
 * @param proto_len length of the ip data part (used for checksum of pseudo header)
 * @return checksum (as u16) to be saved directly in the protocol header
 */
pub fn ip_chksum_pseudo(
    p: &mut PacketBuffer,
    proto: u8,
    proto_len: u16,
    src: &mut LwipAddr,
    dest: &mut LwipAddr,
) {
    if IP_IS_V6(dest) {
        return ip6_chksum_pseudo(p, proto, proto_len, ip_2_ip6(src), ip_2_ip6(dest));
    } else {
        return inet_chksum_pseudo(p, proto, proto_len, ip_2_ip4(src), ip_2_ip4(dest));
    }
}

/// Parts of the pseudo checksum which are common to IPv4 and IPv6
///
/// # Arguments
///
/// * `ctx`:
/// * `p`:
/// * `proto`:
/// * `proto_len`:
/// * `chksum_len`:
/// * `acc`:
///
/// returns: u16
///
/// # Examples
///
/// ```
///
/// ```
pub fn inet_cksum_pseudo_partial_base(
    ctx: &LwipContext,
    p: &mut PacketBuffer,
    proto: u8,
    proto_len: u16,
    chksum_len: u16,
    acc: u32,
) -> u16 {
    let q: &mut PacketBuffer;
    let swapped: i32 = 0;
    let chklen: u16;

    /* iterate through all pbuf in chain */
    // for (q = p; (q != NULL) && (chksum_len > 0); q = q.next) {
    //   LWIP_DEBUGF(INET_DEBUG, ("inet_chksum_pseudo(): checksumming pbuf %p (has next %p) \n",
    //                            q, q.next));
    //   chklen = q.len;
    //   if (chklen > chksum_len) {
    //     chklen = chksum_len;
    //   }
    //   acc += LWIP_CHKSUM(q.payload, chklen);
    //   chksum_len = (chksum_len - chklen);
    //   LWIP_ASSERT("delete me", chksum_len < 0x7fff);
    //   /*LWIP_DEBUGF(INET_DEBUG, ("inet_chksum_pseudo(): unwrapped lwip_chksum()=%"X32_F" \n", acc));*/
    //   /* fold the upper bit down */
    //   acc = fold_u32t(acc);
    //   if (q.len % 2 != 0) {
    //     swapped = !swapped;
    //     acc = swap_bytes_in_word(acc);
    //   }
    //   /*LWIP_DEBUGF(INET_DEBUG, ("inet_chksum_pseudo(): wrapped lwip_chksum()=%"X32_F" \n", acc));*/
    // }

    if swapped {
        acc = SWAP_BYTES_IN_WORD(acc);
    }

    acc += proto::to_le(); //lwip_htons(proto);
    acc += lwip_htons(proto_len);

    /* Fold 32-bit sum to 16 bits
    calling this twice is probably faster than if statements... */
    acc = fold_u32(acc);
    acc = fold_u32(acc);
    // LWIP_DEBUGF(INET_DEBUG, ("inet_chksum_pseudo(): PacketBuffer chain lwip_chksum()=%"X32_F"\n", acc));
    return !(acc & 0xffff);
}

/* inet_chksum_pseudo_partial:
 *
 * Calculates the IPv4 pseudo Internet checksum used by TCP and UDP for a pbuf chain.
 * IP addresses are expected to be in network byte order.
 *
 * @param p chain of pbufs over that a checksum should be calculated (ip data part)
 * @param src source ip address (used for checksum of pseudo header)
 * @param dst destination ip address (used for checksum of pseudo header)
 * @param proto ip protocol (used for checksum of pseudo header)
 * @param proto_len length of the ip data part (used for checksum of pseudo header)
 * @return checksum (as u16) to be saved directly in the protocol header
 */
pub fn inet_chksum_pseudo_partial(
    p: &mut PacketBuffer,
    proto: u8,
    proto_len: u16,
    chksum_len: u16,
    src: &mut ip4_addr,
    dest: &mut ip4_addr,
) {
    let acc: u32;
    let addr: u32;

    addr = ip4_addr_get_u32(src);
    acc = (addr & 0xffff);
    acc = (acc + ((addr >> 16) & 0xffff));
    addr = ip4_addr_get_u32(dest);
    acc = (acc + (addr & 0xffff));
    acc = (acc + ((addr >> 16) & 0xffff));
    /* fold down to 16 bits */
    acc = fold_u32(acc);
    acc = fold_u32(acc);

    return inet_cksum_pseudo_partial_base(p, proto, proto_len, chksum_len, acc);
}

/*
 * Calculates the checksum with IPv6 pseudo header used by TCP and UDP for a pbuf chain.
 * IPv6 addresses are expected to be in network byte order. Will only compute for a
 * portion of the payload.
 *
 * @param p chain of pbufs over that a checksum should be calculated (ip data part)
 * @param proto ipv6 protocol/next header (used for checksum of pseudo header)
 * @param proto_len length of the ipv6 payload (used for checksum of pseudo header)
 * @param chksum_len number of payload bytes used to compute chksum
 * @param src source ipv6 address (used for checksum of pseudo header)
 * @param dest destination ipv6 address (used for checksum of pseudo header)
 * @return checksum (as u16) to be saved directly in the protocol header
 */
pub fn ip6_chksum_pseudo_partial(
    p: &mut PacketBuffer,
    proto: u8,
    proto_len: u16,
    chksum_len: u16,
    src: &mut ip6_addr_t,
    dest: &mut ip6_addr_t,
) {
    let acc: u32 = 0;
    let addr: u32;
    let addr_part: u8;

    // for (addr_part = 0; addr_part < 4; addr_part+= 1) {
    //   addr = src.addr[addr_part];
    //   acc = (acc + (addr & 0xffff));
    //   acc = (acc + ((addr >> 16) & 0xffff));
    //   addr = dest.addr[addr_part];
    //   acc = (acc + (addr & 0xffff));
    //   acc = (acc + ((addr >> 16) & 0xffff));
    // }
    /* fold down to 16 bits */
    acc = fold_u32(acc);
    acc = fold_u32(acc);

    return inet_cksum_pseudo_partial_base(p, proto, proto_len, chksum_len, acc);
}

/* ip_chksum_pseudo_partial:
 *
 * Calculates the IPv4 or IPv6 pseudo Internet checksum used by TCP and UDP for a pbuf chain.
 *
 * @param p chain of pbufs over that a checksum should be calculated (ip data part)
 * @param src source ip address (used for checksum of pseudo header)
 * @param dst destination ip address (used for checksum of pseudo header)
 * @param proto ip protocol (used for checksum of pseudo header)
 * @param proto_len length of the ip data part (used for checksum of pseudo header)
 * @return checksum (as u16) to be saved directly in the protocol header
 */
pub fn ip_chksum_pseudo_partial(
    p: &mut PacketBuffer,
    proto: u8,
    proto_len: u16,
    chksum_len: u16,
    src: &mut LwipAddr,
    dest: &mut LwipAddr,
) {
    if (IP_IS_V6(dest)) {
        return ip6_chksum_pseudo_partial(
            p,
            proto,
            proto_len,
            chksum_len,
            ip_2_ip6(src),
            ip_2_ip6(dest),
        );
    } else {
        return inet_chksum_pseudo_partial(
            p,
            proto,
            proto_len,
            chksum_len,
            ip_2_ip4(src),
            ip_2_ip4(dest),
        );
    }
}

/* inet_chksum:
 *
 * Calculates the Internet checksum over a portion of memory. Used primarily for IP
 * and ICMP.
 *
 * @param dataptr start of the buffer to calculate the checksum (no alignment needed)
 * @param len length of the buffer to calculate the checksum
 * @return checksum (as u16) to be saved directly in the protocol header
 */

pub fn inet_chksum(dataptr: &Vec<u8>, len: usize) {
    return !LWIP_CHKSUM(dataptr, len);
}

/*
 * Calculate a checksum over a chain of pbufs (without pseudo-header, much like
 * inet_chksum only pbufs are used).
 *
 * @param p pbuf chain over that the checksum should be calculated
 * @return checksum (as u16) to be saved directly in the protocol header
 */
pub fn inet_chksum_pbuf(p: &mut PacketBuffer) {
    let acc: u32;
    let q: &mut PacketBuffer;
    let swapped: i32 = 0;

    acc = 0;
    // for (q = p; q != NULL; q = q.next) {
    //   acc += LWIP_CHKSUM(q.payload, q.len);
    //   acc = fold_u32t(acc);
    //   if (q.len % 2 != 0) {
    //     swapped = !swapped;
    //     acc = swap_bytes_in_word(acc);
    //   }
    // }

    if (swapped) {
        acc = SWAP_BYTES_IN_WORD(acc);
    }
    return !(acc & 0xffff);
}

/* These are some implementations for LWIP_CHKSUM_COPY, which copies data
 * like MEMCPY but generates a checksum at the same time. Since this is a
 * performance-sensitive function, you might want to create your own version
 * in assembly targeted at your hardware by defining it in lwipopts.h:
 *   // #define LWIP_CHKSUM_COPY(dst, src, len) your_chksum_copy(dst, src, len)
 */

/* Safe but slow: first call MEMCPY, then call LWIP_CHKSUM.
 * For architectures with big caches, data might still be in cache when
 * generating the checksum after copying.
 */
pub fn lwip_chksum_copy(dst: &mut Vec<u8>, src: &Vec<u8>, len: usize) {
    MEMCPY(dst, src, len);
    return LWIP_CHKSUM(dst, len);
}

/// Swap the bytes in an u16: much like lwip_htons() for little-endian
pub fn swap_bytes_in_word(w: u16) -> u16 { (((w) & 0xff) << 8) | (((w) & 0xff00) >> 8)}

/// Split an u32 in two u16s and add them up
pub fn fold_u32(u: u32) -> u32 {
   (((u) >> 16) + ((u) & 0x0000ffff))
}
