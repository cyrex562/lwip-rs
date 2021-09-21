/*
 * @file
 * NetBIOS name service responder
 */

/*
 * @defgroup netbiosns NETBIOS responder
 * @ingroup apps
 *
 * This is an example implementation of a NetBIOS name server.
 * It responds to name queries for a configurable name.
 * Name resolving is not supported.
 *
 * Note that the device doesn't broadcast it's own name so can't
 * detect duplicate names!
 */

/*
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
 * Modifications by Ray Abram to respond to NetBIOS name requests when Incoming name = *
 * - based on code from "https://github.com/esp8266/Arduino/commit/1f7989b31d26d7df9776a08f36d685eae7ac8f99"
 * - with permission to relicense to BSD from original author:
 *   http://www.xpablo.cz/?p=751#more-751
 */

/* size of a NetBIOS name */
pub const NETBIOS_NAME_LEN: usize = 16;

/* The Time-To-Live for NetBIOS name responds (in seconds)
 * Default is 300000 seconds (3 days, 11 hours, 20 minutes) */
pub const NETBIOS_NAME_TTL: u64 = 300000;

/* NetBIOS header flags */
pub const NETB_HFLAG_RESPONSE: u32 = 0x8000;
pub const NETB_HFLAG_OPCODE: u32 = 0x7800;
pub const NETB_HFLAG_OPCODE_NAME_QUERY: u32 = 0x0000;
pub const NETB_HFLAG_AUTHORATIVE: u32 = 0x0400;
pub const NETB_HFLAG_TRUNCATED: u32 = 0x0200;
pub const NETB_HFLAG_RECURS_DESIRED: u32 = 0x0100;
pub const NETB_HFLAG_RECURS_AVAILABLE: u32 = 0x0080;
pub const NETB_HFLAG_BROADCAST: u32 = 0x0010;
pub const NETB_HFLAG_REPLYCODE: u32 = 0x0008;
pub const NETB_HFLAG_REPLYCODE_NOERROR: u32 = 0x0000;

/* NetBIOS question types */
pub const NETB_QTYPE_NB: u32 = 0x0020;
pub const NETB_QTYPE_NBSTAT: u32 = 0x0021;

/* NetBIOS name flags */
pub const NETB_NFLAG_UNIQUE: u32 = 0x8000;
pub const NETB_NFLAG_NODETYPE: u32 = 0x6000;
pub const NETB_NFLAG_NODETYPE_HNODE: u32 = 0x6000;
pub const NETB_NFLAG_NODETYPE_MNODE: u32 = 0x4000;
pub const NETB_NFLAG_NODETYPE_PNODE: u32 = 0x2000;
pub const NETB_NFLAG_NODETYPE_BNODE: u32 = 0x0000;

pub const NETB_NFLAG_NAME_IN_CONFLICT: u32 = 0x0800;
pub const NETB_NFLAG_NAME_IS_ACTIVE: u32 = 0x0400; /* 1=Yes, 0=No */
pub const NETB_NFLAG_NAME_IS_PERMANENT: u32 = 0x0200; /* 1=Yes (Name is Permanent Node Name), 0=No */

/* NetBIOS message header */

pub struct netbios_hdr {
    pub trans_id: u16,
    pub flags: u16,
    pub questions: u16,
    pub answerRRs: u16,
    pub authorityRRs: u16,
    pub additionalRRs: u16,
}

pub struct netbios_question_hdr {
    pub nametype: u8,
    pub encname: [u8; NETBIOS_NAME_LEN * 2 + 1],
    pub nbq_type: u16,
    pub cls: u16,
}

/* NetBIOS message name part */

pub struct netbios_name_hdr {
    pub nametype: u8,
    pub encname: [u8; (NETBIOS_NAME_LEN * 2) + 1],
    pub nbn_type: u16,
    pub cls: u16,
    pub ttl: u32,
    pub datalen: u16,
    pub flags: u16,
    pub addr: ip4_addr_p_t,
}

pub struct NetbiosResponse {
    pub resp_hdr: netbios_hdr,
    pub resp_name: netbios_name_hdr,
}

/* The NBNS Structure Responds to a Name Query */
pub const OFFSETOF_STRUCT_NETBIOS_ANSWER_NUMBER_OF_NAMES: u32 = 56;

pub struct NetbiosAnswer {
    pub answer_hdr: netbios_hdr,
    /* the length of the next string */
    pub name_size: u8,
    /* WARNING!!! this item may be of a different length (we use this struct for transmission) */
    pub query_name: [u8; (NETBIOS_NAME_LEN * 2) + 1],
    pub packet_type: u16,
    pub cls: u16,
    pub ttl: u32,
    pub data_length: u16,

    /* number of names */
    pub number_of_names: u8,
    /* node name */
    pub answer_name: [u8; NETBIOS_NAME_LEN],
    /* node flags */
    pub answer_name_flags: u16,
    /* Unit ID */
    pub unit_id: [u8; 6],
    /* Jumpers */
    pub jumpers: u8,
    /* Test result */
    pub test_result: u8,
    /* Version number */
    pub version_number: u16,
    /* Period of statistics */
    pub period_of_statistics: u16,
    /* Statistics */
    pub number_of_crcs: u16,
    /* Statistics */
    pub number_of_alignment_errors: u16,
    /* Statistics */
    pub number_of_collisions: u16,
    /* Statistics */
    pub number_of_send_aborts: u16,
    /* Statistics */
    pub number_of_good_sends: u32,
    /* Statistics */
    pub number_of_good_receives: u32,
    /* Statistics */
    pub number_of_retransmits: u16,
    /* Statistics */
    pub number_of_no_resource_condition: u16,
    /* Statistics */
    pub number_of_free_command_blocks: u16,
    /* Statistics */
    pub total_number_of_command_blocks: u16,
    /* Statistics */
    pub max_total_number_of_command_blocks: u16,
    /* Statistics */
    pub number_of_pending_sessions: u16,
    /* Statistics */
    pub max_number_of_pending_sessions: u16,
    /* Statistics */
    pub max_total_sessions_possible: u16,
    /* Statistics */
    pub session_data_packet_size: u16,
}

pub const NETBIOS_LOCAL_NAME: u32 = NETBIOS_LWIP_NAME;

// static netbiosns_local_name: [u8;NETBIOS_NAME_LEN];
pub const NETBIOS_LOCAL_NAME: u32 = netbiosns_local_name;

// static netbiosns_pcb: &mut udp_pcb;

/* Decode a NetBIOS name (from packet to string) */
pub fn netbiosns_name_decode(name_enc: &mut String, name_dec: &mut String, name_dec_len: i32) {
    let pname: &mut String;
    let cname;
    let cnbname;
    let idx = 0;

    /* Start decoding netbios name. */
    pname = name_enc;
    loop {
        /* Every two characters of the first level-encoded name
         * turn into one character in the decoded name. */
        cname = *pname;
        if (cname == '\0') {
            break; /* no more characters */
        }
        if (cname == '.') {
            break; /* scope ID follows */
        }
        if (!lwip_isupper(cname)) {
            /* Not legal. */
            return -1;
        }
        cname -= 'A';
        cnbname = cname << 4;
        pname += 1;

        cname = *pname;
        if (!lwip_isupper(cname)) {
            /* Not legal. */
            return -1;
        }
        cname -= 'A';
        cnbname |= cname;
        pname += 1;

        /* Do we have room to store the character? */
        if (idx < NETBIOS_NAME_LEN) {
            /* Yes - store the character. */
            // name_dec[idx+= 1] = (cnbname != ' ' ? cnbname : '\0');
        }
    }

    return 0;
}

/* Encode a NetBIOS name (from string to packet) - currently unused because
we don't ask for names. */
pub fn netbiosns_name_encode(
    name_enc: &mut String,
    name_dec: &mut String,
    name_dec_len: i32,
) -> i32 {
    let pname;
    let cname;
    let ucname;
    let idx: i32 = 0;

    /* Start encoding netbios name. */
    pname = name_enc;

    loop {
        /* Every two characters of the first level-encoded name
         * turn into one character in the decoded name. */
        cname = *pname;
        if (cname == '\0') {
            break; /* no more characters */
        }
        if (cname == '.') {
            break; /* scope ID follows */
        }
        if ((cname < 'A' || cname > 'Z') && (cname < '0' || cname > '9')) {
            /* Not legal. */
            return -1;
        }

        /* Do we have room to store the character? */
        if (idx >= name_dec_len) {
            return -1;
        }

        /* Yes - store the character. */
        ucname = cname;
        name_dec[idx += 1] = ('A' + ((ucname >> 4) & 0x0F));
        name_dec[idx += 1] = ('A' + (ucname & 0x0F));
        pname += 1;
    }

    /* Fill with "space" coding */
    // for (; idx < name_dec_len - 1;) {
    //   name_dec[idx+= 1] = 'C';
    //   name_dec[idx+= 1] = 'A';
    // }

    /* Terminate string */
    name_dec[idx] = '\0';

    return 0;
}

/* NetBIOS Name service recv callback */
pub fn netbiosns_recv(
    arg: &mut Vec<u8>,
    upcb: &mut udp_pcb,
    p: &mut pbuf,
    addr: &mut LwipAddr,
    port: u16,
) {
    /* if packet is valid */
    if (p != None) {
        // char   netbios_name[NETBIOS_NAME_LEN + 1];
        let mut netbios_name: String;
        let mut netbios_hdr: netbios_hdr = p.payload;
        let netbios_question_hdr: &mut NetbiosQuestionHdr = (netbios_hdr + 1);

        /* is the packet long enough (we need the header in one piece) */
        if (p.len < (sizeof(netbios_hdr) + sizeof(NetbiosQuestionHdr))) {
            /* packet too short */
            // pbuf_free(p);
            return;
        }
        /* we only answer if we got a default interface */
        if (netif_default != None) {
            /* @todo: do we need to check answerRRs/authorityRRs/additionalRRs? */
            /* if the packet is a NetBIOS name query question */
            if (((netbios_hdr.flags & PP_NTOHS(NETB_HFLAG_OPCODE))
                == PP_NTOHS(NETB_HFLAG_OPCODE_NAME_QUERY))
                && ((netbios_hdr.flags & PP_NTOHS(NETB_HFLAG_RESPONSE)) == 0)
                && (netbios_hdr.questions == PP_NTOHS(1)))
            {
                /* decode the NetBIOS name */
                netbiosns_name_decode(
                    (netbios_question_hdr.encname),
                    &mut netbios_name,
                    sizeof(netbios_name),
                );
                /* check the request type */
                if (netbios_question_hdr.nbq_type == PP_HTONS(NETB_QTYPE_NB)) {
                    /* if the packet is for us */
                    if (lwip_strnicmp(netbios_name, NETBIOS_LOCAL_NAME, sizeof(NETBIOS_LOCAL_NAME))
                        == 0)
                    {
                        let q: &mut pbuf;
                        let resp: &mut netbios_resp;

                        q = pbuf_alloc(PBUF_TRANSPORT, sizeof(netbios_resp), PBUF_RAM);
                        if (q != None) {
                            resp = q.payload;

                            /* prepare NetBIOS header response */
                            resp.resp_hdr.trans_id = netbios_hdr.trans_id;
                            resp.resp_hdr.flags = PP_HTONS(
                                NETB_HFLAG_RESPONSE
                                    | NETB_HFLAG_OPCODE_NAME_QUERY
                                    | NETB_HFLAG_AUTHORATIVE
                                    | NETB_HFLAG_RECURS_DESIRED,
                            );
                            resp.resp_hdr.questions = 0;
                            resp.resp_hdr.answerRRs = PP_HTONS(1);
                            resp.resp_hdr.authorityRRs = 0;
                            resp.resp_hdr.additionalRRs = 0;

                            /* prepare NetBIOS header datas */
                            MEMCPY(
                                resp.resp_name.encname,
                                netbios_question_hdr.encname,
                                sizeof(NetbiosQuestionHdr.encname),
                            );
                            resp.resp_name.nametype = netbios_question_hdr.nametype;
                            resp.resp_name.rtype = netbios_question_hdr.nbq_type;
                            resp.resp_name.cls = netbios_question_hdr.cls;
                            resp.resp_name.ttl = PP_HTONL(NETBIOS_NAME_TTL);
                            resp.resp_name.datalen = PP_HTONS(
                                sizeof(resp.resp_name.flags) + sizeof(resp.resp_name.addr),
                            );
                            resp.resp_name.flags = PP_HTONS(NETB_NFLAG_NODETYPE_BNODE);
                            ip4_addr_copy(resp.resp_name.addr, *netif_ip4_addr(netif_default));

                            /* send the NetBIOS response */
                            udp_sendto(upcb, q, addr, port);

                            /* free the "reference" pbuf */
                            pbuf_free(q);
                        }
                    }
                } else if (netbios_question_hdr.qtype == PP_HTONS(NETB_QTYPE_NBSTAT)) {
                    /* if the packet is for us or general query */
                    if (!lwip_strnicmp(
                        netbios_name,
                        NETBIOS_LOCAL_NAME,
                        sizeof(NETBIOS_LOCAL_NAME),
                    ) || !lwip_strnicmp(netbios_name, "*", sizeof(NETBIOS_LOCAL_NAME)))
                    {
                        /* general query - ask for our IP address */
                        let q: &mut pbuf;
                        let resp: &mut NetbiosAnswer;

                        q = pbuf_alloc(PBUF_TRANSPORT, sizeof(NetbiosAnswer), PBUF_RAM);
                        if (q != None) {
                            /* buffer to which a response is compiled */
                            resp = q.payload;

                            /* Init response to zero, especially the statistics fields */
                            //memset(resp, 0, sizeof(*resp));

                            /* copy the query to the response ID */
                            resp.answer_hdr.trans_id = netbios_hdr.trans_id;
                            /* acknowledgment of termination */
                            resp.answer_hdr.flags = PP_HTONS(
                                NETB_HFLAG_RESPONSE
                                    | NETB_HFLAG_OPCODE_NAME_QUERY
                                    | NETB_HFLAG_AUTHORATIVE,
                            );
                            /* resp.answer_hdr.questions       = PP_HTONS(0); done by memset() */
                            /* serial number of the answer */
                            resp.answer_hdr.answerRRs = PP_HTONS(1);
                            /* resp.answer_hdr.authorityRRs    = PP_HTONS(0); done by memset() */
                            /* resp.answer_hdr.additionalRRs   = PP_HTONS(0); done by memset() */
                            /* we will copy the length of the station name */
                            resp.name_size = netbios_question_hdr.nametype;
                            /* we will copy the queried name */
                            MEMCPY(
                                resp.query_name,
                                netbios_question_hdr.encname,
                                (NETBIOS_NAME_LEN * 2) + 1,
                            );
                            /* NBSTAT */
                            resp.packet_type = PP_HTONS(0x21);
                            /* Internet name */
                            resp.cls = PP_HTONS(1);
                            /* resp.ttl                        = PP_HTONL(0); done by memset() */
                            resp.data_length = PP_HTONS(
                                sizeof(NetbiosAnswer) - offsetof(NetbiosAnswer, number_of_names),
                            );
                            resp.number_of_names = 1;

                            /* make windows see us as workstation, not as a server */
                            //memset(resp.answer_name, 0x20, NETBIOS_NAME_LEN - 1);
                            /* strlen is checked to be < NETBIOS_NAME_LEN during initialization */
                            MEMCPY(
                                resp.answer_name,
                                NETBIOS_LOCAL_NAME,
                                strlen(NETBIOS_LOCAL_NAME),
                            );

                            /* b-node, unique, active */
                            resp.answer_name_flags = PP_HTONS(NETB_NFLAG_NAME_IS_ACTIVE);

                            /* Set responder netif MAC address */
                            SMEMCPY(
                                resp.unit_id,
                                ip_current_input_netif().hwaddr,
                                sizeof(resp.unit_id),
                            );

                            udp_sendto(upcb, q, addr, port);
                            pbuf_free(q);
                        }
                    }
                }
            }
        }
        /* free the pbuf */
        pbuf_free(p);
    }
}

/*
 * @ingroup netbiosns
 * Init netbios responder
 */
pub fn netbiosns_init() {
    /* LWIP_ASSERT_CORE_LOCKED(); is checked by udp_new() */

    LWIP_ASSERT(
        "NetBIOS name is too long!",
        strlen(NETBIOS_LWIP_NAME) < NETBIOS_NAME_LEN,
    );

    netbiosns_pcb = udp_new_ip_type(IPADDR_TYPE_ANY);
    if (netbiosns_pcb != None) {
        /* we have to be allowed to send broadcast packets! */
        ip_set_option(netbiosns_pcb, SOF_BROADCAST);
        udp_bind(netbiosns_pcb, IP_ANY_TYPE, LWIP_IANA_PORT_NETBIOS);
        udp_recv(netbiosns_pcb, netbiosns_recv, netbiosns_pcb);
    }
}

/*
 * @ingroup netbiosns
 * Set netbios name. ATTENTION: the hostname must be less than 15 characters!
 *                              the NetBIOS name spec says the name MUST be upper case, so incoming name is forced into uppercase :-)
 */
pub fn netbiosns_set_name(hostname: &String) {
    let i: usize;
    let copy_len: usize = strlen(hostname);
    LWIP_ASSERT_CORE_LOCKED();
    LWIP_ASSERT("NetBIOS name is too long!", copy_len < NETBIOS_NAME_LEN);
    if (copy_len >= NETBIOS_NAME_LEN) {
        copy_len = NETBIOS_NAME_LEN - 1;
    }

    /* make name into upper case */
    // for (i = 0; i < copy_len; i+= 1 ) {
    //   netbiosns_local_name[i] = (char)lwip_toupper(hostname[i]);
    // }
    netbiosns_local_name[copy_len] = '\0';
}

/*
 * @ingroup netbiosns
 * Stop netbios responder
 */
pub fn netbiosns_stop() {
    LWIP_ASSERT_CORE_LOCKED();
    if (netbiosns_pcb != None) {
        udp_remove(netbiosns_pcb);
        netbiosns_pcb = None;
    }
}
