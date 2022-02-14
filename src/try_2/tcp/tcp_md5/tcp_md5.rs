/*
 * @file: An implementation of TCP MD5 signatures by using various hooks in
 * lwIP to implement custom tcp options and custom socket options.
 */

/*
 * Copyright (c) 2018 Simon Goldschmidt
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
 * Author: Simon Goldschmidt <goldsimon@gmx.de>
 */

//  pull in md5 of ppp?

//#undef  LWIP_INCLUDED_POLARSSL_MD5
// pub const LWIP_INCLUDED_POLARSSL_MD5: u32 = 1;

// #error tcp_md5 needs LWIP_TCP_PCB_NUM_EXT_ARGS

// pub const LWIP_TCP_OPT_MD5: u32 = 19; //  number of the md5 option
// pub const LWIP_TCP_OPT_LEN_MD5: u32 = 18; //  length of the md5 option
// pub const LWIP_TCP_OPT_LEN_MD5_OUT: u32 = 20; //  18 + alignment
// pub const LWIP_TCP_MD5_DIGEST_LEN: u32 = 16;

//  This keeps the md5 state internally
pub struct tcp_md5_conn_info {
    // let mut next: &mut tcp_md5_conn_info;
    pub remote_addr: LwipAddr,
    pub remote_port: u16,
    pub key: [u8; TCP_MD5SIG_MAXKEYLEN],
    pub key_len: u16,
}

//  Callback function prototypes:
// pub fn tcp_md5_extarg_destroy(id: u8, data: &mut Vec<u8>);
// pub fn tcp_md5_extarg_passive_open(id: u8, lpcb: &mut TcpContext_listen, cpcb: &mut TcpContext) -> Result<(), LwipError>;
//  Define our tcp ext arg callback structure:
// const struct tcp_ext_arg_callbacks tcp_md5_ext_arg_callbacks = {
//   tcp_md5_extarg_destroy,
//   tcp_md5_extarg_passive_open
// };

// static tcp_md5_extarg_id: u8 = LWIP_TCP_PCB_NUM_EXT_ARG_ID_INVALID;
// static tcp_md5_opts_buf: [u8;40];

//  Initialize this module (allocates a tcp ext arg id)
pub fn tcp_md5_init() {
    tcp_md5_extarg_id = tcp_ext_arg_alloc_id();
}

//  Create a conn-info structure that holds the md5 state per connection
pub fn tcp_md5_conn_info_alloc() -> tcp_md5_conn_info {
    return mem_malloc(sizeof(tcp_md5_conn_info));
}

//  Frees a conn-info structure that holds the md5 state per connection
pub fn tcp_md5_conn_info_free(info: &mut tcp_md5_conn_info) {
    mem_free(info);
}

//  A pcb is about to be destroyed. Free its extdata
pub fn tcp_md5_extarg_destroy(id: u8, data: &mut Vec<u8>) {
    let mut iter: &mut tcp_md5_conn_info;

    LWIP_ASSERT(
        "tcp_md5_extarg_id != LWIP_TCP_PCB_NUM_EXT_ARG_ID_INVALID",
        tcp_md5_extarg_id != LWIP_TCP_PCB_NUM_EXT_ARG_ID_INVALID,
    );
    LWIP_ASSERT("id == tcp_md5_extarg_id", id == tcp_md5_extarg_id);

    iter = data;
    while (iter != None) {
        let info: &mut tcp_md5_conn_info = iter;
        iter = iter.next;
        tcp_md5_conn_info_free(info);
    }
}

//  Try to find an md5 connection info for the specified remote connection
pub fn tcp_md5_get_info(
    pcb: &mut TcpContext,
    remote_ip: &mut LwipAddr,
    remote_port: u16,
) -> tcp_md5_conn_info {
    if (pcb != None) {
        let info: &mut tcp_md5_conn_info = tcp_ext_arg_get(pcb, tcp_md5_extarg_id);
        while (info != None) {
            if (ip_addr_cmp(&info.remote_addr, remote_ip)) {
                if (info.remote_port == remote_port) {
                    return info;
                }
            }
            info = info.next;
        }
    }
    return None;
}

/* Passive open: copy md5 connection info from listen pcb to connection pcb
 * or return error (connection will be closed)
 */
pub fn tcp_md5_extarg_passive_open(
    id: u8,
    lpcb: &mut TcpContext_listen,
    cpcb: &mut TcpContext,
) -> Result<(), LwipError> {
    let mut iter: &mut tcp_md5_conn_info;

    LWIP_ASSERT("lpcb != NULL", lpcb != None);
    LWIP_ASSERT("cpcb != NULL", cpcb != None);
    LWIP_ASSERT(
        "tcp_md5_extarg_id != LWIP_TCP_PCB_NUM_EXT_ARG_ID_INVALID",
        tcp_md5_extarg_id != LWIP_TCP_PCB_NUM_EXT_ARG_ID_INVALID,
    );
    LWIP_ASSERT("id == tcp_md5_extarg_id", id == tcp_md5_extarg_id);

    iter = tcp_ext_arg_get(lpcb, id);
    while (iter != None) {
        if (iter.remote_port == cpcb.remote_port) {
            if (ip_addr_cmp(&iter.remote_addr, &cpcb.remote_ip)) {
                let info: tcp_md5_conn_info = tcp_md5_conn_info_alloc();
                if (info != None) {
                    memcpy(info, iter, sizeof(tcp_md5_conn_info));
                    tcp_ext_arg_set(cpcb, id, info);
                    tcp_ext_arg_set_callbacks(cpcb, id, &tcp_md5_ext_arg_callbacks);
                    return Ok(());
                } else {
                    return ERR_MEM;
                }
            }
        }
        iter = iter.next;
    }
    //  remote connection not found
    return ERR_VAL;
}

//  Parse tcp header options and return 1 if an md5 signature option was found
pub fn tcp_md5_parseopt(opts: &mut Vec<u8>, optlen: u16, md5_digest_out: &mut Vec<u8>) {
    let data: u8;
    let optidx: u16;

    //  Parse the TCP MSS option, if present.
    if (optlen != 0) {
        for optidx in 0..optlen {
            let opt: u8 = opts[optidx += 1];
            match (opt) {
                LWIP_TCP_OPT_EOL => {
                    //  End of options.
                    //          LWIP_DEBUGF(TCP_INPUT_DEBUG, ("tcp_parseopt: EOL\n"));
                    return 0;
                }
                LWIP_TCP_OPT_NOP => {
                    //  NOP option.
                    //          LWIP_DEBUGF(TCP_INPUT_DEBUG, ("tcp_parseopt: NOP\n"));
                }
                LWIP_TCP_OPT_MD5 => {
                    //          LWIP_DEBUGF(TCP_INPUT_DEBUG, ("tcp_parseopt: MD5\n"));
                    if (opts[optidx += 1] != LWIP_TCP_OPT_LEN_MD5
                        || (optidx - 2 + LWIP_TCP_OPT_LEN_MD5) > optlen)
                    {
                        //  Bad length
                        //            LWIP_DEBUGF(TCP_INPUT_DEBUG, ("tcp_parseopt: bad length\n"));
                        return 0;
                    }
                    //  An MD5 option with the right option length.
                    memcpy(md5_digest_out, &opts[optidx], LWIP_TCP_MD5_DIGEST_LEN);
                    //  no need to process the options further
                    return 1;
                }

                _ => {
                    //          LWIP_DEBUGF(TCP_INPUT_DEBUG, ("tcp_parseopt: other\n"));
                    data = opts[optidx += 1];
                    if (data < 2) {
                        //            LWIP_DEBUGF(TCP_INPUT_DEBUG, ("tcp_parseopt: bad length\n"));
                        /* If the length field is zero, the options are malformed
                        and we don't process them further. */
                        return 0;
                    }
                    /* All other options have a length field, so that we easily
                    can skip past them. */
                    optidx += data - 2;
                }
            }
        }
    }
    return 0;
}

/* Get tcp options into contiguous memory. May be required if input pbufs
 * are chained.
 */
pub fn tcp_md5_options_singlebuf(
    hdr: &mut tcp_hdr,
    optlen: u16,
    opt1len: u16,
    opt2: &mut Vec<u8>,
) -> Vec<u8> {
    let opts: &mut Vec<u8>;
    LWIP_ASSERT("hdr != NULL", hdr != None);
    LWIP_ASSERT("optlen >= opt1len", optlen >= opt1len);
    opts = hdr + TCP_HLEN;
    if (optlen == opt1len) {
        //  arleady in one piece
        return opts;
    }
    if (optlen > sizeof(tcp_md5_opts_buf)) {
        //  options too long
        return None;
    }
    LWIP_ASSERT("opt2 != NULL", opt2 != None);
    //  copy first part
    memcpy(tcp_md5_opts_buf, opts, opt1len);
    //  copy second part
    memcpy(&tcp_md5_opts_buf[opt1len], opt2, optlen - opt1len);
    return tcp_md5_opts_buf;
}

//  Create the md5 digest for a given segment
pub fn tcp_md5_create_digest(
    ip_src: &mut LwipAddr,
    ip_dst: &mut LwipAddr,
    hdr: &mut tcp_hdr,
    key: &mut Vec<u8>,
    key_len: usize,
    digest_out: &mut Vec<u8>,
    p: &mut PacketBuffer,
) {
    let ctx: md5_context;
    let tmp8: u8;
    let tmp16: u16;
    let addr_len: usize = IP_ADDR_RAW_SIZE(*ip_src);

    if (p != None) {
        LWIP_ASSERT("pbuf must not poto: i32 tcp header here!", hdr != p.payload);
    }

    //  Generate the hash, using MD5.
    md5_starts(&ctx);
    /* 1. the TCP pseudo-header (in the order: source IP address,
    destination IP address, zero-padded protocol number, and
    segment length) */
    md5_update(&ctx, ip_src, addr_len);
    md5_update(&ctx, ip_dst, addr_len);
    tmp8 = 0; //  zero-padded
    md5_update(&ctx, &tmp8, 1);
    tmp8 = IP_PROTO_TCP;
    md5_update(&ctx, &tmp8, 1);
    // tmp16 = lwip_htons(TCPH_HDRLEN_BYTES(hdr) + (p ? p.tot_len : 0));
    md5_update(&ctx, &tmp16, 2);
    /* 2. the TCP header, excluding options, and assuming a checksum of
    zero */
    md5_update(&ctx, hdr, sizeof(tcp_hdr));
    //  3. the TCP segment data (if any)
    if ((p != None) && (p.tot_len != 0)) {
        let q: &mut PacketBuffer;
        // for (q = p; q != None; q = q.next) {
        //   md5_update(&ctx,q.payload, q.len);
        // }
    }
    /* 4. an independently-specified key or password, known to both TCPs
    and presumably connection-specific */
    md5_update(&ctx, key, key_len);

    md5_finish(&ctx, digest_out);
    return 1;
}

//  Duplicate a tcp header and make sure the fields are in network byte order
pub fn tcp_md5_dup_tcphdr(
    tcphdr_copy: &mut tcp_hdr,
    tcphdr_in: &mut tcp_hdr,
    tcphdr_in_is_host_order: i32,
) {
    memcpy(tcphdr_copy, tcphdr_in, sizeof(tcp_hdr));
    tcphdr_copy.chksum = 0; //  checksum is zero for the pseudo header
    if (tcphdr_in_is_host_order) {
        //  lwIP writes the TCP header values back to the buffer, we need to invert that here:
        tcphdr_copy.src = lwip_htons(tcphdr_copy.src);
        tcphdr_copy.dest = lwip_htons(tcphdr_copy.dest);
        tcphdr_copy.seqno = lwip_htonl(tcphdr_copy.seqno);
        tcphdr_copy.ackno = lwip_htonl(tcphdr_copy.ackno);
        tcphdr_copy.wnd = lwip_htons(tcphdr_copy.wnd);
        tcphdr_copy.urgp = lwip_htons(tcphdr_copy.urgp);
    }
}

//  Check if md5 is enabled on a given pcb
pub fn tcp_md5_is_enabled_on_pcb(pcb: &mut TcpContext) {
    if (tcp_md5_extarg_id != LWIP_TCP_PCB_NUM_EXT_ARG_ID_INVALID) {
        let info: &mut tcp_md5_conn_info = tcp_ext_arg_get(pcb, tcp_md5_extarg_id);
        if (info != None) {
            return 1;
        }
    }
    return 0;
}

//  Check if md5 is enabled on a given listen pcb
pub fn tcp_md5_is_enabled_on_lpcb(lpcb: &mut TcpContext_listen) {
    //  same as for connection pcbs
    return tcp_md5_is_enabled_on_pcb(lpcb);
}

//  Hook implementation for LWIP_HOOK_TCP_OPT_LENGTH_SEGMENT
pub fn tcp_md5_get_additional_option_length(
    pcb: &mut TcpContext,
    internal_option_length: u8,
) -> u8 {
    if ((pcb != None) && tcp_md5_is_enabled_on_pcb(pcb)) {
        let new_option_length: u8 = internal_option_length + LWIP_TCP_OPT_LEN_MD5_OUT;
        LWIP_ASSERT("overflow", new_option_length > internal_option_length);
        LWIP_ASSERT(
            "options too long",
            new_option_length <= TCP_MAX_OPTION_BYTES,
        );
        return new_option_length;
    }
    return internal_option_length;
}

//  Hook implementation for LWIP_HOOK_TCP_INPACKET_PCB when called for listen pcbs
pub fn tcp_md5_check_listen(
    lpcb: &mut TcpContext_listen,
    hdr: &mut tcp_hdr,
    optlen: u16,
    opt1len: u16,
    opt2: &mut Vec<u8>,
) -> Result<(), LwipError> {
    LWIP_ASSERT("lpcb != NULL", lpcb != None);

    if (tcp_md5_is_enabled_on_lpcb(lpcb)) {
        let opts: Vec<u8>;
        let digest_received: [u8; LWIP_TCP_MD5_DIGEST_LEN];
        let digest_calculated: [u8; LWIP_TCP_MD5_DIGEST_LEN];
        let info: tcp_md5_conn_info = tcp_md5_get_info(lpcb, ip_current_src_addr(), hdr.src);
        if (info != None) {
            opts = tcp_md5_options_singlebuf(hdr, optlen, opt1len, opt2);
            if (opts != None) {
                if (tcp_md5_parseopt(&mut opts, optlen, digest_received)) {
                    let tcphdr_copy: tcp_hdr;
                    tcp_md5_dup_tcphdr(&tcphdr_copy, hdr, 1);
                    if (tcp_md5_create_digest(
                        ip_current_src_addr(),
                        ip_current_dest_addr(),
                        &tcphdr_copy,
                        info.key,
                        info.key_len,
                        digest_calculated,
                        None,
                    )) {
                        //  everything set up, compare the digests
                        if (!memcmp(digest_received, digest_calculated, LWIP_TCP_MD5_DIGEST_LEN)) {
                            //  equal
                            return Ok(());
                        }
                        //  not equal
                    }
                }
            }
        }
        //  md5 enabled on this pcb but no match or other error -> fail
        return ERR_VAL;
    }
    return Ok(());
}

//  Hook implementation for LWIP_HOOK_TCP_INPACKET_PCB
pub fn tcp_md5_check_inpacket(
    pcb: &mut TcpContext,
    hdr: &mut tcp_hdr,
    optlen: u16,
    opt1len: u16,
    opt2: &mut Vec<u8>,
    p: &mut PacketBuffer,
) {
    LWIP_ASSERT("pcb != NULL", pcb != None);

    if (pcb.state == LISTEN) {
        return tcp_md5_check_listen(pcb, hdr, optlen, opt1len, opt2);
    }

    if (tcp_md5_is_enabled_on_pcb(pcb)) {
        let info: tcp_md5_conn_info = tcp_md5_get_info(pcb, ip_current_src_addr(), hdr.src);
        if (info != None) {
            let opts: Vec<u8>;
            let digest_received: [u8; LWIP_TCP_MD5_DIGEST_LEN];
            let digest_calculated: [u8; LWIP_TCP_MD5_DIGEST_LEN];
            opts = tcp_md5_options_singlebuf(hdr, optlen, opt1len, opt2);
            if (opts != None) {
                if (tcp_md5_parseopt(&mut opts, optlen, digest_received)) {
                    let hdr_copy: tcp_hdr;
                    tcp_md5_dup_tcphdr(&hdr_copy, hdr, 1);
                    if (tcp_md5_create_digest(
                        &pcb.remote_ip,
                        &pcb.local_ip,
                        &hdr_copy,
                        info.key,
                        info.key_len,
                        digest_calculated,
                        p,
                    )) {
                        //  everything set up, compare the digests
                        if (!memcmp(digest_received, digest_calculated, LWIP_TCP_MD5_DIGEST_LEN)) {
                            //  equal
                            return Ok(());
                        }
                        //  not equal
                    }
                }
            }
        }
        //  md5 enabled on this pcb but no match or other error -> fail
        return ERR_VAL;
    }
    return Ok(());
}

//  Hook implementation for LWIP_HOOK_TCP_ADD_TX_OPTIONS
pub fn tcp_md5_add_tx_options(
    p: &mut PacketBuffer,
    hdr: &mut tcp_hdr,
    pcb: &mut TcpContext,
    opts: &mut u32,
) -> u32 {
    LWIP_ASSERT("p != NULL", p != None);
    LWIP_ASSERT("hdr != NULL", hdr != None);
    LWIP_ASSERT("pcb != NULL", pcb != None);
    LWIP_ASSERT("opts != NULL", opts != None);

    if (tcp_md5_is_enabled_on_pcb(pcb)) {
        let digest_calculated: [u8; LWIP_TCP_MD5_DIGEST_LEN];
        let opts_ret: &mut u32 = opts + 5; //  we use 20 bytes: 2 bytes padding + 18 bytes for this option
        let ptr: &mut Vec<u8> = opts;

        let info: tcp_md5_conn_info = tcp_md5_get_info(pcb, &pcb.remote_ip, pcb.remote_port);
        if (info != None) {
            let hdr_copy: tcp_hdr;
            let hdrsize: usize = TCPH_HDRLEN_BYTES(hdr);
            tcp_md5_dup_tcphdr(&hdr_copy, hdr, 0);
            //  p.payload points to the tcp header
            LWIP_ASSERT("p.payload == hdr", p.payload == hdr);
            if (!pbuf_remove_header(p, hdrsize)) {
                let ret: u8;
                if (!tcp_md5_create_digest(
                    &pcb.local_ip,
                    &pcb.remote_ip,
                    &hdr_copy,
                    info.key,
                    info.key_len,
                    digest_calculated,
                    p,
                )) {
                    info = None;
                }
                ret = pbuf_add_header_force(p, hdrsize);
                LWIP_ASSERT(
                    "tcp_md5_add_tx_options: PacketBuffer_add_header_force failed",
                    !ret,
                );
            } else {
                LWIP_ASSERT("error", 0);
            }
        }
        if (info == None) {
            //  create an invalid signature by zeroing the digest
            //memset(&digest_calculated, 0, sizeof(digest_calculated));
        }

        *ptr += 1 = LWIP_TCP_OPT_NOP;
        *ptr += 1 = LWIP_TCP_OPT_NOP;
        *ptr += 1 = LWIP_TCP_OPT_MD5;
        *ptr += 1 = LWIP_TCP_OPT_LEN_MD5;
        memcpy(ptr, digest_calculated, LWIP_TCP_MD5_DIGEST_LEN);
        ptr += LWIP_TCP_MD5_DIGEST_LEN;
        LWIP_ASSERT("ptr == opts_ret", ptr == opts_ret);
        return opts_ret;
    }
    return opts;
}

//  Hook implementation for LWIP_HOOK_SOCKETS_SETSOCKOPT
pub fn tcp_md5_setsockopt_hook(
    sock: &mut lwip_sock,
    level: i32,
    optname: i32,
    optval: &Vec<u8>,
    optlen: socklen_t,
    err: &mut i32,
) {
    LWIP_ASSERT("sock != NULL", sock != None);
    LWIP_ASSERT("err != NULL", err != None);

    if ((level == IPPROTO_TCP) && (optname == TCP_MD5SIG)) {
        let md5: &mut tcp_md5sig = optval;
        if ((optval == None) || (optlen < sizeof(tcp_md5sig))) {
            *err = EINVAL;
        } else {
            if (sock.conn
                && (NETCONNTYPE_GROUP(netconn_type(sock.conn)) == NETCONN_TCP)
                && (sock.conn.pcb.tcp != None))
            {
                if (tcp_md5_extarg_id == LWIP_TCP_PCB_NUM_EXT_ARG_ID_INVALID) {
                    //  not initialized
                    *err = EINVAL;
                } else {
                    let info: tcp_md5_conn_info = tcp_md5_conn_info_alloc();
                    if (info == None) {
                        *err = ENOMEM;
                    } else {
                        let addr_valid: i32 = 0;
                        //  OK, fill and link this request
                        memcpy(info.key, md5.tcpm_key, TCP_MD5SIG_MAXKEYLEN);
                        info.key_len = md5.tcpm_keylen;
                        //memset(&info.remote_addr, 0, sizeof(info.remote_addr));
                        if (md5.tcpm_addr.ss_family == AF_INET) {
                            let sin: &mut sockaddr_in = &md5.tcpm_addr;
                            memcpy(&info.remote_addr, &sin.sin_addr, sizeof(sin.sin_addr));
                            IP_SET_TYPE_VAL(info.remote_addr, IPADDR_TYPE_V4);
                            info.remote_port = lwip_htons(sin.sin_port);
                            addr_valid = 1;
                        } else if (md5.tcpm_addr.ss_family == AF_INET6) {
                            let sin6: &mut sockaddr_in6 = &md5.tcpm_addr;
                            memcpy(&info.remote_addr, &sin6.sin6_addr, sizeof(sin6.sin6_addr));
                            IP_SET_TYPE_VAL(info.remote_addr, IPADDR_TYPE_V6);
                            info.remote_port = lwip_htons(sin6.sin6_port);
                            addr_valid = 1;
                        }
                        if (addr_valid) {
                            //  store it
                            tcp_ext_arg_set_callbacks(
                                sock.conn.pcb.tcp,
                                tcp_md5_extarg_id,
                                &tcp_md5_ext_arg_callbacks,
                            );
                            info.next = tcp_ext_arg_get(sock.conn.pcb.tcp, tcp_md5_extarg_id);
                            tcp_ext_arg_set(sock.conn.pcb.tcp, tcp_md5_extarg_id, info);
                        } else {
                            *err = EINVAL;
                            tcp_md5_conn_info_free(&mut info);
                        }
                    }
                }
            } else {
                //  not a tcp netconn
                *err = EINVAL;
            }
        }
        return 1;
    }
    return 0;
}
