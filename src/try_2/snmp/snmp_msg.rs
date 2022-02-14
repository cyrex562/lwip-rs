use crate::snmp::snmp_h::snmp_varbind;

/*
 * @file
 * SNMP message processing (RFC1157).
 */

/*
 * Copyright (c) 2006 Axon Digital Design B.V., The Netherlands.
 * Copyright (c) 2016 Elias Oenal.
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
 * Author: Christiaan Simons <christiaan.simons@axon.tv>
 *         Martin Hentschel <info@cl-soft.de>
 *         Elias Oenal <lwip@eliasoenal.com>
 */

pub const SNMP_V3_AUTH_FLAG: u32 = 0x01;
pub const SNMP_V3_PRIV_FLAG: u32 = 0x02;

//  Security levels 
pub const SNMP_V3_NOAUTHNOPRIV: u32 = 0x00;
pub const SNMP_V3_AUTHNOPRIV: u32 = SNMP_V3_AUTH_FLAG;
pub const SNMP_V3_AUTHPRIV: u32 = (SNMP_V3_AUTH_FLAG | SNMP_V3_PRIV_FLAG);

//  public (non-static) constants 
//  SNMP community string 
// snmp_community: &String = SNMP_COMMUNITY;
// //  SNMP community string for write access 
// snmp_community_write: &String = SNMP_COMMUNITY_WRITE;
// //  SNMP community string for sending traps 
// snmp_community_trap: &String = SNMP_COMMUNITY_TRAP;

// snmp_write_callback_fct snmp_write_callback     = None;
// pub fn                    *snmp_write_callback_arg = None;

static v1_enabled: u8 = 1;
static v2c_enabled: u8 = 1;
static v3_enabled: u8 = 1;

pub fn snmp_version_enabled(version: u8) {
    if (version == SNMP_VERSION_1) {
        return v1_enabled;
    } else if (version == SNMP_VERSION_2c) {
        return v2c_enabled;
    } else if (version == SNMP_VERSION_3) {
        return v3_enabled;
    } else {
        LWIP_ASSERT("Invalid SNMP version", 0);
        return 0;
    }
}

pub fn snmp_v1_enabled() -> u8 {
    return snmp_version_enabled(SNMP_VERSION_1);
}

pub fn snmp_v2c_enabled() -> u8 {
    return snmp_version_enabled(SNMP_VERSION_2c);
}

pub fn snmp_v3_enabled() -> u8 {
    return snmp_version_enabled(SNMP_VERSION_3);
}

pub fn snmp_version_enable(version: u8, enable: u8) {
    if (version == SNMP_VERSION_1) {
        v1_enabled = enable;
    } else if (version == SNMP_VERSION_2c) {
        v2c_enabled = enable;
    } else if (version == SNMP_VERSION_3) {
        v3_enabled = enable;
    } else {
        LWIP_ASSERT("Invalid SNMP version", 0);
    }
}

pub fn snmp_v1_enable(enable: u8) {
    snmp_version_enable(SNMP_VERSION_1, enable);
}

pub fn snmp_v2c_enable(enable: u8) {
    snmp_version_enable(SNMP_VERSION_2c, enable);
}

pub fn snmp_v3_enable(enable: u8) {
    snmp_version_enable(SNMP_VERSION_3, enable);
}

/*
 * @ingroup snmp_core
 * Returns current SNMP community string.
 * @return current SNMP community string
 */
pub fn snmp_get_community() -> String {
    return snmp_community;
}

/*
 * @ingroup snmp_core
 * Sets SNMP community string.
 * The string itself (its storage) must be valid throughout the whole life of
 * program (or until it is changed to sth else).
 *
 * @param community is a pointer to new community string
 */
pub fn snmp_set_community(community: &str) {
    LWIP_ASSERT_CORE_LOCKED();
    LWIP_ASSERT(
        "community string is too long!",
        strlen(community) <= SNMP_MAX_COMMUNITY_STR_LEN,
    );
    snmp_community = community;
}

/*
 * @ingroup snmp_core
 * Returns current SNMP write-access community string.
 * @return current SNMP write-access community string
 */
pub fn snmp_get_community_write() -> String {
    return snmp_community_write;
}

/*
 * @ingroup snmp_traps
 * Returns current SNMP community string used for sending traps.
 * @return current SNMP community string used for sending traps
 */
pub fn snmp_get_community_trap() -> String {
    return snmp_community_trap;
}

/*
 * @ingroup snmp_core
 * Sets SNMP community string for write-access.
 * The string itself (its storage) must be valid throughout the whole life of
 * program (or until it is changed to sth else).
 *
 * @param community is a pointer to new write-access community string
 */
pub fn snmp_set_community_write(community: &str) {
    LWIP_ASSERT_CORE_LOCKED();
    LWIP_ASSERT("community string must not be NULL", community != None);
    LWIP_ASSERT(
        "community string is too long!",
        strlen(community) <= SNMP_MAX_COMMUNITY_STR_LEN,
    );
    snmp_community_write = community;
}

/*
 * @ingroup snmp_traps
 * Sets SNMP community string used for sending traps.
 * The string itself (its storage) must be valid throughout the whole life of
 * program (or until it is changed to sth else).
 *
 * @param community is a pointer to new trap community string
 */
pub fn snmp_set_community_trap(community: &str) {
    LWIP_ASSERT_CORE_LOCKED();
    LWIP_ASSERT(
        "community string is too long!",
        strlen(community) <= SNMP_MAX_COMMUNITY_STR_LEN,
    );
    snmp_community_trap = community;
}

/*
 * @ingroup snmp_core
 * Callback fired on every successful write access
 */
pub fn snmp_set_write_callback(
    write_callback: snmp_write_callback_fct,
    callback_arg: &mut Vec<u8>,
) {
    LWIP_ASSERT_CORE_LOCKED();
    snmp_write_callback = write_callback;
    snmp_write_callback_arg = callback_arg;
}

//  ----------------------------------------------------------------------- 
//  forward declarations 
//  ----------------------------------------------------------------------- 

// pub fn snmp_process_get_request(request: &mut snmp_request) -> Result<(), LwipError>;pub fn snmp_process_get_request(request: &mut snmp_request) -> Result<(), LwipError>pub fn snmp_process_get_request(request: &mut snmp_request) -> Result<(), LwipError>pub fn snmp_process_get_request(request: &mut snmp_request) -> Result<(), LwipError>
// static snmp_process_getnext_request: err_t(request: &mut snmp_request);
// static snmp_process_getbulk_request: err_t(request: &mut snmp_request);
// static snmp_process_set_request: err_t(request: &mut snmp_request);

// pub fn snmp_parse_inbound_frame(request: &mut snmp_request) -> Result<(), LwipError>;pub fn snmp_parse_inbound_frame(request: &mut snmp_request) -> Result<(), LwipError>pub fn snmp_parse_inbound_frame(request: &mut snmp_request) -> Result<(), LwipError>
// static snmp_prepare_outbound_frame: err_t(request: &mut snmp_request);
// static snmp_complete_outbound_frame: err_t(request: &mut snmp_request);
// pub fn snmp_execute_write_callbacks(request: &mut snmp_request);

//  ----------------------------------------------------------------------- 
//  implementation 
//  ----------------------------------------------------------------------- 

pub fn snmp_receive(handle: &mut Vec<u8>, p: &mut PacketBuffer, source_ip: &mut LwipAddr, port: u16) {
    let err: err_t;
    let request: snmp_request;

    //memset(&request, 0, sizeof(request));
    request.handle = handle;
    request.source_ip = source_ip;
    request.source_port = port;
    request.inbound_pbuf = p;

    snmp_stats.inpkts += 1;

    err = snmp_parse_inbound_frame(&request);
    if (err == ERR_OK) {
        err = snmp_prepare_outbound_frame(&request);
        if (err == ERR_OK) {
            if (request.error_status == SNMP_ERR_NOERROR) {
                //  only process frame if we do not already have an error to return (e.g. all readonly) 
                if (request.request_type == SNMP_ASN1_CONTEXT_PDU_GET_REQ) {
                    err = snmp_process_get_request(&request);
                } else if (request.request_type == SNMP_ASN1_CONTEXT_PDU_GET_NEXT_REQ) {
                    err = snmp_process_getnext_request(&request);
                } else if (request.request_type == SNMP_ASN1_CONTEXT_PDU_GET_BULK_REQ) {
                    err = snmp_process_getbulk_request(&request);
                } else if (request.request_type == SNMP_ASN1_CONTEXT_PDU_SET_REQ) {
                    err = snmp_process_set_request(&request);
                }
            } else {
                let vb: snmp_varbind;

                // vb.next = None;
                vb.prev = None;
                vb.asn1_type = SNMP_ASN1_TYPE_COUNTER32;
                vb.value_len = sizeof;

                match (request.error_status) {
                    SNMP_ERR_AUTHORIZATIONERROR => {
                        let oid: [u32; 11] = [1, 3, 6, 1, 6, 3, 15, 1, 1, 5, 0];
                        snmp_oid_assign(&vb.oid, oid, LWIP_ARRAYSIZE(oid));
                        vb.value = &snmp_stats.wrongdigests;
                    }
                    // break;
                    SNMP_ERR_UNKNOWN_ENGINEID => {
                        let oid: [u32; 11] = [1, 3, 6, 1, 6, 3, 15, 1, 1, 4, 0];
                        snmp_oid_assign(&vb.oid, oid, LWIP_ARRAYSIZE(oid));
                        vb.value = &snmp_stats.unknownengineids;
                    }
                    // break;
                    SNMP_ERR_UNKNOWN_SECURITYNAME => {
                        let oid: [u32; 11] = [1, 3, 6, 1, 6, 3, 15, 1, 1, 3, 0];
                        snmp_oid_assign(&vb.oid, oid, LWIP_ARRAYSIZE(oid));
                        vb.value = &snmp_stats.unknownusernames;
                    }
                    // break;
                    SNMP_ERR_UNSUPPORTED_SECLEVEL => {
                        let oid: [u32; 11] = [1, 3, 6, 1, 6, 3, 15, 1, 1, 1, 0];
                        snmp_oid_assign(&vb.oid, oid, LWIP_ARRAYSIZE(oid));
                        vb.value = &snmp_stats.unsupportedseclevels;
                    }
                    // break;
                    SNMP_ERR_NOTINTIMEWINDOW => {
                        let oid: [u32; 11] = [1, 3, 6, 1, 6, 3, 15, 1, 1, 2, 0];
                        snmp_oid_assign(&vb.oid, oid, LWIP_ARRAYSIZE(oid));
                        vb.value = &snmp_stats.notintimewindows;
                    }
                    // break;
                    SNMP_ERR_DECRYIPTION_ERROR => {
                        let oid: [u32; 11] = [1, 3, 6, 1, 6, 3, 15, 1, 1, 6, 0];
                        snmp_oid_assign(&vb.oid, oid, LWIP_ARRAYSIZE(oid));
                        vb.value = &snmp_stats.decryptionerrors;
                    }
                    // break;
                    _ => {
                        //  Unknown or unhandled error_status 
                        err = ERR_ARG;
                    }
                }

                if (err == ERR_OK) {
                    snmp_append_outbound_varbind(&(request.outbound_pbuf_stream), &vb);
                    request.error_status = SNMP_ERR_NOERROR;
                }

                request.request_out_type = (SNMP_ASN1_CLASS_CONTEXT
                    | SNMP_ASN1_CONTENTTYPE_CONSTRUCTED
                    | SNMP_ASN1_CONTEXT_PDU_REPORT);
                request.request_id = request.msg_id;
            }

            if (err == ERR_OK) {
                err = snmp_complete_outbound_frame(&request);

                if (err == ERR_OK) {
                    err = snmp_sendto(
                        request.handle,
                        request.outbound_pbuf,
                        request.source_ip,
                        request.source_port,
                    );

                    if ((request.request_type == SNMP_ASN1_CONTEXT_PDU_SET_REQ)
                        && (request.error_status == SNMP_ERR_NOERROR)
                        && (snmp_write_callback != None))
                    {
                        //  raise write notification for all written objects 
                        snmp_execute_write_callbacks(&request);
                    }
                }
            }
        }

        if (request.outbound_pbuf != None) {
            pbuf_free(request.outbound_pbuf);
        }
    }
}

pub fn snmp_msg_getnext_validate_node_inst(
    node_instance: &mut snmp_node_instance,
    validate_arg: &mut Vec<u8>,
) {
    if (((node_instance.access & SNMP_NODE_INSTANCE_ACCESS_READ) != SNMP_NODE_INSTANCE_ACCESS_READ)
        || (node_instance.get_value == None))
    {
        return SNMP_ERR_NOSUCHINSTANCE;
    }

    if ((node_instance.asn1_type == SNMP_ASN1_TYPE_COUNTER64)
        && ((validate_arg).version == SNMP_VERSION_1))
    {
        //  according to RFC 2089 skip Counter64 objects in GetNext requests from v1 clients 
        return SNMP_ERR_NOSUCHINSTANCE;
    }

    return SNMP_ERR_NOERROR;
}

pub fn snmp_process_varbind(request: &mut snmp_request, vb: &mut snmp_varbind, get_next: u8) {
    let err: err_t;
    let node_instance: snmp_node_instance;
    //memset(&node_instance, 0, sizeof(node_instance));

    if (get_next) {
        let result_oid: snmp_obj_id;
        request.error_status = snmp_get_next_node_instance_from_oid(
            vb.oid.id,
            vb.oid.len,
            snmp_msg_getnext_validate_node_inst,
            request,
            &result_oid,
            &node_instance,
        );

        if (request.error_status == SNMP_ERR_NOERROR) {
            snmp_oid_assign(&vb.oid, result_oid.id, result_oid.len);
        }
    } else {
        request.error_status =
            snmp_get_node_instance_from_oid(vb.oid.id, vb.oid.len, &node_instance);

        if (request.error_status == SNMP_ERR_NOERROR) {
            //  use 'getnext_validate' method for validation to avoid code duplication (some checks have to be executed here) 
            request.error_status = snmp_msg_getnext_validate_node_inst(&node_instance, request);

            if (request.error_status != SNMP_ERR_NOERROR) {
                if (node_instance.release_instance != None) {
                    node_instance.release_instance(&node_instance);
                }
            }
        }
    }

    if (request.error_status != SNMP_ERR_NOERROR) {
        if (request.error_status >= SNMP_VARBIND_EXCEPTION_OFFSET) {
            if ((request.version == SNMP_VERSION_2c) || request.version == SNMP_VERSION_3) {
                //  in SNMP v2c a varbind related exception is stored in varbind and not in frame header 
                vb.asn1_type = (SNMP_ASN1_CONTENTTYPE_PRIMITIVE
                    | SNMP_ASN1_CLASS_CONTEXT
                    | (request.error_status & SNMP_VARBIND_EXCEPTION_MASK));
                vb.value_len = 0;

                err = snmp_append_outbound_varbind(&(request.outbound_pbuf_stream), vb);
                if (err == ERR_OK) {
                    //  we stored the exception in varbind -> go on 
                    request.error_status = SNMP_ERR_NOERROR;
                } else if (err == ERR_BUF) {
                    request.error_status = SNMP_ERR_TOOBIG;
                } else {
                    request.error_status = SNMP_ERR_GENERROR;
                }
            }
        } else {
            //  according to RFC 1157/1905, all other errors only return genError 
            request.error_status = SNMP_ERR_GENERROR;
        }
    } else {
        let len: i16 = node_instance.get_value(&node_instance, vb.value);

        if (len >= 0) {
            vb.value_len = len; //  cast is OK because we checked >= 0 above 
            vb.asn1_type = node_instance.asn1_type;

            LWIP_ASSERT(
                "SNMP_MAX_VALUE_SIZE is configured too low",
                (vb.value_len & !SNMP_GET_VALUE_RAW_DATA) <= SNMP_MAX_VALUE_SIZE,
            );
            err = snmp_append_outbound_varbind(&request.outbound_pbuf_stream, vb);

            if (err == ERR_BUF) {
                request.error_status = SNMP_ERR_TOOBIG;
            } else if (err != ERR_OK) {
                request.error_status = SNMP_ERR_GENERROR;
            }
        } else {
            request.error_status = SNMP_ERR_GENERROR;
        }

        if (node_instance.release_instance != None) {
            node_instance.release_instance(&node_instance);
        }
    }
}

/*
 * Service an internal or external event for SNMP GET.
 *
 * @param request points to the associated message process state
 */
pub fn snmp_process_get_request(request: &mut snmp_request) -> Result<(), LwipError> {
    let err: err_t;
    let vb: snmp_varbind;
    vb.value = request.value_buffer;

    //  LWIP_DEBUGF(SNMP_DEBUG, ("SNMP get request\n"));

    while (request.error_status == SNMP_ERR_NOERROR) {
        err = snmp_vb_enumerator_get_next(&request.inbound_varbind_enumerator, &vb);
        if (err == SNMP_VB_ENUMERATOR_ERR_OK) {
            if ((vb.asn_1type == SNMP_ASN1_TYPE_None) && (vb.value_len == 0)) {
                snmp_process_varbind(request, &vb, 0);
            } else {
                request.error_status = SNMP_ERR_GENERROR;
            }
        } else if (err == SNMP_VB_ENUMERATOR_ERR_EOVB) {
            //  no more varbinds in request 
            break;
        } else if (err == SNMP_VB_ENUMERATOR_ERR_ASN1ERROR) {
            //  malformed ASN.1, don't answer 
            return ERR_ARG;
        } else {
            request.error_status = SNMP_ERR_GENERROR;
        }
    }

    return Ok(());
}

/*
 * Service an internal or external event for SNMP GET.
 *
 * @param request points to the associated message process state
 */
pub fn snmp_process_getnext_request(request: &mut snmp_request) -> Result<(), LwipError> {
    let err: err_t;
    let vb: snmp_varbind;
    vb.value = request.value_buffer;

    //  LWIP_DEBUGF(SNMP_DEBUG, ("SNMP get-next request\n"));

    while (request.error_status == SNMP_ERR_NOERROR) {
        err = snmp_vb_enumerator_get_next(&request.inbound_varbind_enumerator, &vb);
        if (err == SNMP_VB_ENUMERATOR_ERR_OK) {
            if ((vb.asn1_type == SNMP_ASN1_TYPE_None) && (vb.value_len == 0)) {
                snmp_process_varbind(request, &vb, 1);
            } else {
                request.error_status = SNMP_ERR_GENERROR;
            }
        } else if (err == SNMP_VB_ENUMERATOR_ERR_EOVB) {
            //  no more varbinds in request 
            break;
        } else if (err == SNMP_VB_ENUMERATOR_ERR_ASN1ERROR) {
            //  malformed ASN.1, don't answer 
            return ERR_ARG;
        } else {
            request.error_status = SNMP_ERR_GENERROR;
        }
    }

    return Ok(());
}

/*
 * Service an internal or external event for SNMP GETBULKT.
 *
 * @param request points to the associated message process state
 */
pub fn snmp_process_getbulk_request(request: &mut snmp_request) -> Result<(), LwipError> {
    let err: err_t;
    let non_repeaters: i32 = request.non_repeaters;
    let letrepetitions: i32;
    let repetition_offset: u16 = 0;
    let repetition_varbind_enumerator: snmp_varbind_enumerator;
    let vb: snmp_varbind;
    vb.value = request.value_buffer;

    if (SNMP_LWIP_GETBULK_MAX_REPETITIONS > 0) {
        repetitions = LWIP_MIN(request.max_repetitions, SNMP_LWIP_GETBULK_MAX_REPETITIONS);
    } else {
        repetitions = request.max_repetitions;
    }

    //  LWIP_DEBUGF(SNMP_DEBUG, ("SNMP get-bulk request\n"));

    //  process non repeaters and first repetition 
    while (request.error_status == SNMP_ERR_NOERROR) {
        if (non_repeaters == 0) {
            repetition_offset = request.outbound_pbuf_stream.offset;

            if (repetitions == 0) {
                //  do not resolve repeaters when repetitions is set to 0 
                break;
            }
            repetitions -= 1;
        }

        err = snmp_vb_enumerator_get_next(&request.inbound_varbind_enumerator, &vb);
        if (err == SNMP_VB_ENUMERATOR_ERR_EOVB) {
            //  no more varbinds in request 
            break;
        } else if (err == SNMP_VB_ENUMERATOR_ERR_ASN1ERROR) {
            //  malformed ASN.1, don't answer 
            return ERR_ARG;
        } else if ((err != SNMP_VB_ENUMERATOR_ERR_OK)
            || (vb.asn1_type != SNMP_ASN1_TYPE_None)
            || (vb.value_len != 0))
        {
            request.error_status = SNMP_ERR_GENERROR;
        } else {
            snmp_process_varbind(request, &vb, 1);
            non_repeaters -= 1;
        }
    }

    //  process repetitions > 1 
    while ((request.error_status == SNMP_ERR_NOERROR)
        && (repetitions > 0)
        && (request.outbound_pbuf_stream.offset != repetition_offset))
    {
        let all_endofmibview: u8 = 1;
        snmp_vb_enumerator_init(
            &repetition_varbind_enumerator,
            request.outbound_pbuf,
            repetition_offset,
            request.outbound_pbuf_stream.offset - repetition_offset,
        );
        repetition_offset = request.outbound_pbuf_stream.offset; //  for next loop 

        while (request.error_status == SNMP_ERR_NOERROR) {
            vb.value = None; //  do NOT decode value (we enumerate outbound buffer here, so all varbinds have values assigned) 
            err = snmp_vb_enumerator_get_next(&repetition_varbind_enumerator, &vb);
            if (err == SNMP_VB_ENUMERATOR_ERR_OK) {
                vb.value = request.value_buffer;
                snmp_process_varbind(request, &vb, 1);

                if (request.error_status != SNMP_ERR_NOERROR) {
                    //  already set correct error-index (here it cannot be taken from inbound varbind enumerator) 
                    request.error_index =
                        request.non_repeaters + repetition_varbind_enumerator.varbind_count;
                } else if (vb.asn1_type
                    != (SNMP_ASN1_CONTENTTYPE_PRIMITIVE
                        | SNMP_ASN1_CLASS_CONTEXT
                        | SNMP_ASN1_CONTEXT_VARBIND_END_OF_MIB_VIEW))
                {
                    all_endofmibview = 0;
                }
            } else if (err == SNMP_VB_ENUMERATOR_ERR_EOVB) {
                //  no more varbinds in request 
                break;
            } else {
                //        LWIP_DEBUGF(SNMP_DEBUG, ("Very strange, we cannot parse the varbind output that we created just before!"));
                request.error_status = SNMP_ERR_GENERROR;
                request.error_index =
                    request.non_repeaters + repetition_varbind_enumerator.varbind_count;
            }
        }

        if ((request.error_status == SNMP_ERR_NOERROR) && all_endofmibview) {
            //  stop when all varbinds in a loop return EndOfMibView 
            break;
        }

        repetitions -= 1;
    }

    if (request.error_status == SNMP_ERR_TOOBIG) {
        //  for GetBulk it is ok, if not all requested variables fit into the response -> just return the varbinds added so far 
        request.error_status = SNMP_ERR_NOERROR;
    }

    return Ok(());
}

/*
 * Service an internal or external event for SNMP SET.
 *
 * @param request points to the associated message process state
 */
pub fn snmp_process_set_request(request: &mut snmp_request) -> Result<(), LwipError> {
    let err: err_t;
    let vb: snmp_varbind;
    vb.value = request.value_buffer;

    //  LWIP_DEBUGF(SNMP_DEBUG, ("SNMP set request\n"));

    //  perform set test on all objects 
    while (request.error_status == SNMP_ERR_NOERROR) {
        err = snmp_vb_enumerator_get_next(&request.inbound_varbind_enumerator, &vb);
        if (err == SNMP_VB_ENUMERATOR_ERR_OK) {
            let node_instance: snmp_node_instance;
            //memset(&node_instance, 0, sizeof(node_instance));

            request.error_status =
                snmp_get_node_instance_from_oid(vb.oid.id, vb.oid.len, &node_instance);
            if (request.error_status == SNMP_ERR_NOERROR) {
                if (node_instance.asn1_type != vb.asn1_type) {
                    request.error_status = SNMP_ERR_WRONGTYPE;
                } else if (((node_instance.access & SNMP_NODE_INSTANCE_ACCESS_WRITE)
                    != SNMP_NODE_INSTANCE_ACCESS_WRITE)
                    || (node_instance.set_value == None))
                {
                    request.error_status = SNMP_ERR_NOTWRITABLE;
                } else {
                    if (node_instance.set_test != None) {
                        request.error_status =
                            node_instance.set_test(&node_instance, vb.value_len, vb.value);
                    }
                }

                if (node_instance.release_instance != None) {
                    node_instance.release_instance(&node_instance);
                }
            }
        } else if (err == SNMP_VB_ENUMERATOR_ERR_EOVB) {
            //  no more varbinds in request 
            break;
        } else if (err == SNMP_VB_ENUMERATOR_ERR_INVALIDLENGTH) {
            request.error_status = SNMP_ERR_WRONGLENGTH;
        } else if (err == SNMP_VB_ENUMERATOR_ERR_ASN1ERROR) {
            //  malformed ASN.1, don't answer 
            return ERR_ARG;
        } else {
            request.error_status = SNMP_ERR_GENERROR;
        }
    }

    //  perform real set operation on all objects 
    if (request.error_status == SNMP_ERR_NOERROR) {
        snmp_vb_enumerator_init(
            &request.inbound_varbind_enumerator,
            request.inbound_pbuf,
            request.inbound_varbind_offset,
            request.inbound_varbind_len,
        );
        while (request.error_status == SNMP_ERR_NOERROR) {
            err = snmp_vb_enumerator_get_next(&request.inbound_varbind_enumerator, &vb);
            if (err == SNMP_VB_ENUMERATOR_ERR_OK) {
                let node_instance: snmp_node_instance;
                //memset(&node_instance, 0, sizeof(node_instance));
                request.error_status =
                    snmp_get_node_instance_from_oid(vb.oid.id, vb.oid.len, &node_instance);
                if (request.error_status == SNMP_ERR_NOERROR) {
                    if (node_instance.set_value(&node_instance, vb.value_len, vb.value)
                        != SNMP_ERR_NOERROR)
                    {
                        if (request.inbound_varbind_enumerator.varbind_count == 1) {
                            request.error_status = SNMP_ERR_COMMITFAILED;
                        } else {
                            //  we cannot undo the set operations done so far 
                            request.error_status = SNMP_ERR_UNDOFAILED;
                        }
                    }

                    if (node_instance.release_instance != None) {
                        node_instance.release_instance(&node_instance);
                    }
                }
            } else if (err == SNMP_VB_ENUMERATOR_ERR_EOVB) {
                //  no more varbinds in request 
                break;
            } else {
                //  first time enumerating varbinds work but second time not, although nothing should have changed in between ??? 
                request.error_status = SNMP_ERR_GENERROR;
            }
        }
    }

    return Ok(());
}

// #define PARSE_EXEC(code, retValue) \
//   if ((code) != ERR_OK) { \
// //    LWIP_DEBUGF(SNMP_DEBUG, ("Malformed ASN.1 detected.\n")); \
//     snmp_stats.inasnparseerrs+= 1; \
//     return retValue; \
//   }

// #define PARSE_ASSERT(cond, retValue) \
//   if (!(cond)) { \
// //    LWIP_DEBUGF(SNMP_DEBUG, ("SNMP parse assertion failed!: " # cond)); \
//     snmp_stats.inasnparseerrs+= 1; \
//     return retValue; \
//   }

// #define BUILD_EXEC(code, retValue) \
//   if ((code) != ERR_OK) { \
// //    LWIP_DEBUGF(SNMP_DEBUG, ("SNMP error during creation of outbound frame!: " # code)); \
//     return retValue; \
//   }

// #define IF_PARSE_EXEC(code)   PARSE_EXEC(code, ERR_ARG)
// #define IF_PARSE_ASSERT(code) PARSE_ASSERT(code, ERR_ARG)

/*
 * Checks and decodes incoming SNMP message header, logs header errors.
 *
 * @param request points to the current message request state return
 * @return
 * - ERR_OK SNMP header is sane and accepted
 * - ERR_VAL SNMP header is either malformed or rejected
 */
pub fn snmp_parse_inbound_frame(request: &mut snmp_request) -> Result<(), LwipError> {
    let pbuf_stream: snmp_pbuf_stream;
    let tlv: snmp_asn1_tlv;
    let letparent_tlv_value_len: i32;
    let lets32_value: i32;
    let err: err_t;

    let auth: snmpv3_auth_algo_t;
    let priv_algo: snmpv3_priv_algo_t;

    IF_PARSE_EXEC(snmp_pbuf_stream_init(
        &pbuf_stream,
        request.inbound_pbuf,
        0,
        request.inbound_pbuf.tot_len,
    ));

    //  decode main container consisting of version, community and PDU 
    IF_PARSE_EXEC(snmp_asn1_dec_tlv(&pbuf_stream, &tlv));
    IF_PARSE_ASSERT(
        (tlv.asn1_type == SNMP_ASN1_TYPE_SEQUENCE) && (tlv.value_len == pbuf_stream.length),
    );
    parent_tlv_value_len = tlv.value_len;

    //  decode version 
    IF_PARSE_EXEC(snmp_asn1_dec_tlv(&pbuf_stream, &tlv));
    IF_PARSE_ASSERT(tlv.asn1_type == SNMP_ASN1_TYPE_INTEGER);
    parent_tlv_value_len -= SNMP_ASN1_TLV_LENGTH(tlv);
    IF_PARSE_ASSERT(parent_tlv_value_len > 0);

    IF_PARSE_EXEC(snmp_asn1_dec_s32t(&pbuf_stream, tlv.value_len, &s32_value));

    if (((s32_value != SNMP_VERSION_1)
        && (s32_value != SNMP_VERSION_2c)
        && (s32_value != SNMP_VERSION_3))
        || (!snmp_version_enabled(s32_value)))
    {
        //  unsupported SNMP version 
        snmp_stats.inbadversions += 1;
        return ERR_ARG;
    }
    request.version = s32_value;

    if (request.version == SNMP_VERSION_3) {
        let u16_value: u16;
        let inbound_msgAuthenticationParameters_offset: u16;

        //  SNMPv3 doesn't use communities 
        //  @todo: Differentiate read/write access 
        strncpy(
            request.community,
            snmp_community,
            SNMP_MAX_COMMUNITY_STR_LEN,
        );
        request.community[SNMP_MAX_COMMUNITY_STR_LEN] = 0; //  ensure NULL termination (strncpy does NOT guarantee it!) 
        request.community_strlen = strnlen(request.community, SNMP_MAX_COMMUNITY_STR_LEN);

        //  RFC3414 globalData 
        IF_PARSE_EXEC(snmp_asn1_dec_tlv(&pbuf_stream, &tlv));
        IF_PARSE_ASSERT(tlv.asn1_type == SNMP_ASN1_TYPE_SEQUENCE);
        parent_tlv_value_len -= SNMP_ASN1_TLV_HDR_LENGTH(tlv);
        IF_PARSE_ASSERT(parent_tlv_value_len > 0);

        //  decode msgID 
        IF_PARSE_EXEC(snmp_asn1_dec_tlv(&pbuf_stream, &tlv));
        IF_PARSE_ASSERT(tlv.asn1_type == SNMP_ASN1_TYPE_INTEGER);
        parent_tlv_value_len -= SNMP_ASN1_TLV_LENGTH(tlv);
        IF_PARSE_ASSERT(parent_tlv_value_len > 0);

        IF_PARSE_EXEC(snmp_asn1_dec_s32t(&pbuf_stream, tlv.value_len, &s32_value));
        request.msg_id = s32_value;

        //  decode msgMaxSize 
        IF_PARSE_EXEC(snmp_asn1_dec_tlv(&pbuf_stream, &tlv));
        IF_PARSE_ASSERT(tlv.asn1_type == SNMP_ASN1_TYPE_INTEGER);
        parent_tlv_value_len -= SNMP_ASN1_TLV_LENGTH(tlv);
        IF_PARSE_ASSERT(parent_tlv_value_len > 0);

        IF_PARSE_EXEC(snmp_asn1_dec_s32t(&pbuf_stream, tlv.value_len, &s32_value));
        request.msg_max_size = s32_value;

        //  decode msgFlags 
        IF_PARSE_EXEC(snmp_asn1_dec_tlv(&pbuf_stream, &tlv));
        IF_PARSE_ASSERT(tlv.asn1_type == SNMP_ASN1_TYPE_OCTET_STRING);
        parent_tlv_value_len -= SNMP_ASN1_TLV_LENGTH(tlv);
        IF_PARSE_ASSERT(parent_tlv_value_len > 0);

        IF_PARSE_EXEC(snmp_asn1_dec_s32t(&pbuf_stream, tlv.value_len, &s32_value));
        request.msg_flags = s32_value;

        //  decode msgSecurityModel 
        IF_PARSE_EXEC(snmp_asn1_dec_tlv(&pbuf_stream, &tlv));
        IF_PARSE_ASSERT(tlv.asn1_type == SNMP_ASN1_TYPE_INTEGER);
        parent_tlv_value_len -= SNMP_ASN1_TLV_LENGTH(tlv);
        IF_PARSE_ASSERT(parent_tlv_value_len > 0);

        IF_PARSE_EXEC(snmp_asn1_dec_s32t(&pbuf_stream, tlv.value_len, &s32_value));
        request.msg_security_model = s32_value;

        /* RFC3414 msgSecurityParameters
         * The User-based Security Model defines the contents of the OCTET
         * STRING as a SEQUENCE.
         *
         * We skip the protective dummy OCTET STRING header
         * to access the SEQUENCE header.
         */
        IF_PARSE_EXEC(snmp_asn1_dec_tlv(&pbuf_stream, &tlv));
        IF_PARSE_ASSERT(tlv.asn1_type == SNMP_ASN1_TYPE_OCTET_STRING);
        parent_tlv_value_len -= SNMP_ASN1_TLV_HDR_LENGTH(tlv);
        IF_PARSE_ASSERT(parent_tlv_value_len > 0);

        //  msgSecurityParameters SEQUENCE header 
        IF_PARSE_EXEC(snmp_asn1_dec_tlv(&pbuf_stream, &tlv));
        IF_PARSE_ASSERT(tlv.asn1_type == SNMP_ASN1_TYPE_SEQUENCE);
        parent_tlv_value_len -= SNMP_ASN1_TLV_HDR_LENGTH(tlv);
        IF_PARSE_ASSERT(parent_tlv_value_len > 0);

        //  decode msgAuthoritativeEngineID 
        IF_PARSE_EXEC(snmp_asn1_dec_tlv(&pbuf_stream, &tlv));
        IF_PARSE_ASSERT(tlv.asn1_type == SNMP_ASN1_TYPE_OCTET_STRING);
        parent_tlv_value_len -= SNMP_ASN1_TLV_LENGTH(tlv);
        IF_PARSE_ASSERT(parent_tlv_value_len > 0);

        IF_PARSE_EXEC(snmp_asn1_dec_raw(
            &pbuf_stream,
            tlv.value_len,
            request.msg_authoritative_engine_id,
            &u16_value,
            SNMP_V3_MAX_ENGINE_ID_LENGTH,
        ));
        request.msg_authoritative_engine_id_len = u16_value;

        //  msgAuthoritativeEngineBoots 
        IF_PARSE_EXEC(snmp_asn1_dec_tlv(&pbuf_stream, &tlv));
        IF_PARSE_ASSERT(tlv.asn1_type == SNMP_ASN1_TYPE_INTEGER);
        parent_tlv_value_len -= SNMP_ASN1_TLV_LENGTH(tlv);
        IF_PARSE_ASSERT(parent_tlv_value_len > 0);
        IF_PARSE_EXEC(snmp_asn1_dec_s32t(
            &pbuf_stream,
            tlv.value_len,
            &request.msg_authoritative_engine_boots,
        ));

        //  msgAuthoritativeEngineTime 
        IF_PARSE_EXEC(snmp_asn1_dec_tlv(&pbuf_stream, &tlv));
        IF_PARSE_ASSERT(tlv.asn1_type == SNMP_ASN1_TYPE_INTEGER);
        parent_tlv_value_len -= SNMP_ASN1_TLV_LENGTH(tlv);
        IF_PARSE_ASSERT(parent_tlv_value_len > 0);
        IF_PARSE_EXEC(snmp_asn1_dec_s32t(
            &pbuf_stream,
            tlv.value_len,
            &request.msg_authoritative_engine_time,
        ));

        //  msgUserName 
        IF_PARSE_EXEC(snmp_asn1_dec_tlv(&pbuf_stream, &tlv));
        IF_PARSE_ASSERT(tlv.asn1_type == SNMP_ASN1_TYPE_OCTET_STRING);
        parent_tlv_value_len -= SNMP_ASN1_TLV_LENGTH(tlv);
        IF_PARSE_ASSERT(parent_tlv_value_len > 0);

        IF_PARSE_EXEC(snmp_asn1_dec_raw(
            &pbuf_stream,
            tlv.value_len,
            request.msg_user_name,
            &u16_value,
            SNMP_V3_MAX_USER_LENGTH,
        ));
        request.msg_user_name_len = u16_value;

        //  msgAuthenticationParameters 
        //memset(request.msg_authentication_parameters, 0, SNMP_V3_MAX_AUTH_PARAM_LENGTH);
        IF_PARSE_EXEC(snmp_asn1_dec_tlv(&pbuf_stream, &tlv));
        IF_PARSE_ASSERT(tlv.asn1_type == SNMP_ASN1_TYPE_OCTET_STRING);
        parent_tlv_value_len -= SNMP_ASN1_TLV_LENGTH(tlv);
        IF_PARSE_ASSERT(parent_tlv_value_len > 0);
        //  Remember position 
        inbound_msgAuthenticationParameters_offset = pbuf_stream.offset;

        //  Read auth parameters 
        //  IF_PARSE_ASSERT(tlv.value_len <= SNMP_V3_MAX_AUTH_PARAM_LENGTH); 
        IF_PARSE_EXEC(snmp_asn1_dec_raw(
            &pbuf_stream,
            tlv.value_len,
            request.msg_authentication_parameters,
            &u16_value,
            tlv.value_len,
        ));
        request.msg_authentication_parameters_len = u16_value;

        //  msgPrivacyParameters 
        //memset(request.msg_privacy_parameters, 0, SNMP_V3_MAX_PRIV_PARAM_LENGTH);
        IF_PARSE_EXEC(snmp_asn1_dec_tlv(&pbuf_stream, &tlv));
        IF_PARSE_ASSERT(tlv.asn1_type == SNMP_ASN1_TYPE_OCTET_STRING);
        parent_tlv_value_len -= SNMP_ASN1_TLV_LENGTH(tlv);
        IF_PARSE_ASSERT(parent_tlv_value_len > 0);

        IF_PARSE_EXEC(snmp_asn1_dec_raw(
            &pbuf_stream,
            tlv.value_len,
            request.msg_privacy_parameters,
            &u16_value,
            SNMP_V3_MAX_PRIV_PARAM_LENGTH,
        ));
        request.msg_privacy_parameters_len = u16_value;

        /* validate securityParameters here (do this after decoding because we don't want to increase other counters for wrong frames)
        * 1) securityParameters was correctly serialized if we reach here.
        * 2) securityParameters are already cached.
        * 3) if msgAuthoritativeEngineID is unknown, zero-length or too long:
            b) https://tools.ietf.org/html/rfc3414#section-7
        */
        {
            let eid: String;
            let eid_len: u8;

            snmpv3_get_engine_id(&eid, &eid_len);

            if ((request.msg_authoritative_engine_id_len == 0)
                || (request.msg_authoritative_engine_id_len != eid_len)
                || (memcmp(eid, request.msg_authoritative_engine_id, eid_len) != 0))
            {
                snmp_stats.unknownengineids += 1;
                request.msg_flags = 0; //  noauthnopriv 
                request.error_status = SNMP_ERR_UNKNOWN_ENGINEID;
                return Ok(());
            }
        }

        //  4) verify username 
        if (snmpv3_get_user(request.msg_user_name, &auth, None, &priv_algo, None)) {
            snmp_stats.unknownusernames += 1;
            request.msg_flags = 0; //  noauthnopriv 
            request.error_status = SNMP_ERR_UNKNOWN_SECURITYNAME;
            return Ok(());
        }

        //  5) verify security level 
        match (request.msg_flags & (SNMP_V3_AUTH_FLAG | SNMP_V3_PRIV_FLAG)) {
            SNMP_V3_NOAUTHNOPRIV => {
                if ((auth != SNMP_V3_AUTH_ALGO_INVAL) || (priv_algo != SNMP_V3_PRIV_ALGO_INVAL)) {
                    //  Invalid security level for user 
                    snmp_stats.unsupportedseclevels += 1;
                    request.msg_flags = SNMP_V3_NOAUTHNOPRIV;
                    request.error_status = SNMP_ERR_UNSUPPORTED_SECLEVEL;
                    return Ok(());
                }
            }
            // break;
            SNMP_V3_AUTHNOPRIV => {
                if ((auth == SNMP_V3_AUTH_ALGO_INVAL) || (priv_algo != SNMP_V3_PRIV_ALGO_INVAL)) {
                    //  Invalid security level for user 
                    snmp_stats.unsupportedseclevels += 1;
                    request.msg_flags = SNMP_V3_NOAUTHNOPRIV;
                    request.error_status = SNMP_ERR_UNSUPPORTED_SECLEVEL;
                    return Ok(());
                }
            }
            // break;
            SNMP_V3_AUTHPRIV => {
                if ((auth == SNMP_V3_AUTH_ALGO_INVAL) || (priv_algo == SNMP_V3_PRIV_ALGO_INVAL)) {
                    //  Invalid security level for user 
                    snmp_stats.unsupportedseclevels += 1;
                    request.msg_flags = SNMP_V3_NOAUTHNOPRIV;
                    request.error_status = SNMP_ERR_UNSUPPORTED_SECLEVEL;
                    return Ok(());
                }
            }
            // break;
            _ => {
                snmp_stats.unsupportedseclevels += 1;
                request.msg_flags = SNMP_V3_NOAUTHNOPRIV;
                request.error_status = SNMP_ERR_UNSUPPORTED_SECLEVEL;
                return Ok(());
            }
        }

        //  6) if securitylevel specifies authentication, authenticate message. 

        if (request.msg_flags & SNMP_V3_AUTH_FLAG) {
            let zero_arr: [u8; SNMP_V3_MAX_AUTH_PARAM_LENGTH] = [0; SNMP_V3_MAX_AUTH_PARAM_LENGTH];
            let key: [u8; 20];
            let hmac: [u8; LWIP_MAX(SNMP_V3_SHA_LEN, SNMP_V3_MD5_LEN)];
            let auth_stream: snmp_pbuf_stream;

            if (request.msg_authentication_parameters_len > SNMP_V3_MAX_AUTH_PARAM_LENGTH) {
                snmp_stats.wrongdigests += 1;
                request.msg_flags = SNMP_V3_NOAUTHNOPRIV;
                request.error_status = SNMP_ERR_AUTHORIZATIONERROR;
                return Ok(());
            }

            //  Rewind stream 
            IF_PARSE_EXEC(snmp_pbuf_stream_init(
                &auth_stream,
                request.inbound_pbuf,
                0,
                request.inbound_pbuf.tot_len,
            ));
            IF_PARSE_EXEC(snmp_pbuf_stream_seek_abs(
                &auth_stream,
                inbound_msgAuthenticationParameters_offset,
            ));
            //  Set auth parameters to zero for verification 
            IF_PARSE_EXEC(snmp_asn1_enc_raw(
                &auth_stream,
                zero_arr,
                request.msg_authentication_parameters_len,
            ));

            //  Verify authentication 
            IF_PARSE_EXEC(snmp_pbuf_stream_init(
                &auth_stream,
                request.inbound_pbuf,
                0,
                request.inbound_pbuf.tot_len,
            ));

            IF_PARSE_EXEC(snmpv3_get_user(
                request.msg_user_name,
                &auth,
                key,
                None,
                None,
            ));
            IF_PARSE_EXEC(snmpv3_auth(
                &auth_stream,
                request.inbound_pbuf.tot_len,
                key,
                auth,
                hmac,
            ));

            if (memcmp(
                request.msg_authentication_parameters,
                hmac,
                SNMP_V3_MAX_AUTH_PARAM_LENGTH,
            )) {
                snmp_stats.wrongdigests += 1;
                request.msg_flags = SNMP_V3_NOAUTHNOPRIV;
                request.error_status = SNMP_ERR_AUTHORIZATIONERROR;
                return Ok(());
            }

            //  7) if securitylevel specifies authentication, verify engineboots, enginetime and lastenginetime 
            {
                let boots: i32 = snmpv3_get_engine_boots_internal();
                if ((request.msg_authoritative_engine_boots != boots) || (boots == 2147483647)) {
                    snmp_stats.notintimewindows += 1;
                    request.msg_flags = SNMP_V3_AUTHNOPRIV;
                    request.error_status = SNMP_ERR_NOTINTIMEWINDOW;
                    return Ok(());
                }
            }
            {
                let time: i32 = snmpv3_get_engine_time_internal();
                if (request.msg_authoritative_engine_time > (time + 150)) {
                    snmp_stats.notintimewindows += 1;
                    request.msg_flags = SNMP_V3_AUTHNOPRIV;
                    request.error_status = SNMP_ERR_NOTINTIMEWINDOW;
                    return Ok(());
                } else if (time > 150) {
                    if (request.msg_authoritative_engine_time < (time - 150)) {
                        snmp_stats.notintimewindows += 1;
                        request.msg_flags = SNMP_V3_AUTHNOPRIV;
                        request.error_status = SNMP_ERR_NOTINTIMEWINDOW;
                        return Ok(());
                    }
                }
            }
        }

        //  8) if securitylevel specifies privacy, decrypt message. 

        if (request.msg_flags & SNMP_V3_PRIV_FLAG) {
            //  Decrypt message 

            let key: [u8; 20];

            IF_PARSE_EXEC(snmp_asn1_dec_tlv(&pbuf_stream, &tlv));
            IF_PARSE_ASSERT(tlv.asn1_type == SNMP_ASN1_TYPE_OCTET_STRING);
            parent_tlv_value_len -= SNMP_ASN1_TLV_HDR_LENGTH(tlv);
            IF_PARSE_ASSERT(parent_tlv_value_len > 0);

            IF_PARSE_EXEC(snmpv3_get_user(
                request.msg_user_name,
                None,
                None,
                &priv_algo,
                key,
            ));
            if (snmpv3_crypt(
                &pbuf_stream,
                tlv.value_len,
                key,
                request.msg_privacy_parameters,
                request.msg_authoritative_engine_boots,
                request.msg_authoritative_engine_time,
                priv_algo,
                SNMP_V3_PRIV_MODE_DECRYPT,
            ) != ERR_OK)
            {
                snmp_stats.decryptionerrors += 1;
                request.msg_flags = SNMP_V3_AUTHNOPRIV;
                request.error_status = SNMP_ERR_DECRYIPTION_ERROR;
                return Ok(());
            }
        }

        /* 9) calculate max size of scoped pdu?
         * 10) securityname for user is retrieved from usertable?
         * 11) security data is cached?
         * 12)
         */

        /* Scoped PDU
         * Encryption context
         */
        IF_PARSE_EXEC(snmp_asn1_dec_tlv(&pbuf_stream, &tlv));
        IF_PARSE_ASSERT(tlv.asn1_type == SNMP_ASN1_TYPE_SEQUENCE);
        parent_tlv_value_len -= SNMP_ASN1_TLV_HDR_LENGTH(tlv);
        IF_PARSE_ASSERT(parent_tlv_value_len > 0);

        //  contextEngineID 
        IF_PARSE_EXEC(snmp_asn1_dec_tlv(&pbuf_stream, &tlv));
        IF_PARSE_ASSERT(tlv.asn1_type == SNMP_ASN1_TYPE_OCTET_STRING);
        parent_tlv_value_len -= SNMP_ASN1_TLV_LENGTH(tlv);
        IF_PARSE_ASSERT(parent_tlv_value_len > 0);

        IF_PARSE_EXEC(snmp_asn1_dec_raw(
            &pbuf_stream,
            tlv.value_len,
            request.context_engine_id,
            &u16_value,
            SNMP_V3_MAX_ENGINE_ID_LENGTH,
        ));
        request.context_engine_id_len = u16_value;
        //  TODO: do we need to verify this contextengineid too? 

        //  contextName 
        IF_PARSE_EXEC(snmp_asn1_dec_tlv(&pbuf_stream, &tlv));
        IF_PARSE_ASSERT(tlv.asn1_type == SNMP_ASN1_TYPE_OCTET_STRING);
        parent_tlv_value_len -= SNMP_ASN1_TLV_LENGTH(tlv);
        IF_PARSE_ASSERT(parent_tlv_value_len > 0);

        IF_PARSE_EXEC(snmp_asn1_dec_raw(
            &pbuf_stream,
            tlv.value_len,
            request.context_name,
            &u16_value,
            SNMP_V3_MAX_ENGINE_ID_LENGTH,
        ));
        request.context_name_len = u16_value;
        //  TODO: do we need to verify this contextname too? 
    } else {
        //  decode community 
        IF_PARSE_EXEC(snmp_asn1_dec_tlv(&pbuf_stream, &tlv));
        IF_PARSE_ASSERT(tlv.asn1_type == SNMP_ASN1_TYPE_OCTET_STRING);
        parent_tlv_value_len -= SNMP_ASN1_TLV_LENGTH(tlv);
        IF_PARSE_ASSERT(parent_tlv_value_len > 0);

        err = snmp_asn1_dec_raw(
            &pbuf_stream,
            tlv.value_len,
            request.community,
            &request.community_strlen,
            SNMP_MAX_COMMUNITY_STR_LEN,
        );
        if (err == ERR_MEM) {
            //  community string does not fit in our buffer -> its too long -> its invalid 
            request.community_strlen = 0;
            snmp_pbuf_stream_seek(&pbuf_stream, tlv.value_len);
        } else {
            IF_PARSE_ASSERT(err == ERR_OK);
        }
        //  add zero terminator 
        request.community[request.community_strlen] = 0;
    }

    //  decode PDU type (next container level) 
    IF_PARSE_EXEC(snmp_asn1_dec_tlv(&pbuf_stream, &tlv));
    IF_PARSE_ASSERT(tlv.value_len <= pbuf_stream.length);
    request.inbound_padding_len = pbuf_stream.length - tlv.value_len;
    parent_tlv_value_len = tlv.value_len;

    //  validate PDU type 
    //   match (tlv.asn1_type) {
    //     SNMP_ASN1_CLASS_CONTEXT | SNMP_ASN1_CONTENTTYPE_CONSTRUCTED | SNMP_ASN1_CONTEXT_PDU_GET_REQ):
    //       //  GetRequest PDU 
    //       snmp_stats.ingetrequests+= 1;
    //       break;
    //     case (SNMP_ASN1_CLASS_CONTEXT | SNMP_ASN1_CONTENTTYPE_CONSTRUCTED | SNMP_ASN1_CONTEXT_PDU_GET_NEXT_REQ):
    //       //  GetNextRequest PDU 
    //       snmp_stats.ingetnexts+= 1;
    //       break;
    //     case (SNMP_ASN1_CLASS_CONTEXT | SNMP_ASN1_CONTENTTYPE_CONSTRUCTED | SNMP_ASN1_CONTEXT_PDU_GET_BULK_REQ):
    //       //  GetBulkRequest PDU 
    //       if (request.version < SNMP_VERSION_2c) {
    //         //  RFC2089: invalid, drop packet 
    //         return ERR_ARG;
    //       }
    //       break;
    //     case (SNMP_ASN1_CLASS_CONTEXT | SNMP_ASN1_CONTENTTYPE_CONSTRUCTED | SNMP_ASN1_CONTEXT_PDU_SET_REQ):
    //       //  SetRequest PDU 
    //       snmp_stats.insetrequests+= 1;
    //       break;
    //     _ =>
    //       //  unsupported input PDU for this agent (no parse error) 
    // //      LWIP_DEBUGF(SNMP_DEBUG, ("Unknown/Invalid SNMP PDU type received: %d", tlv.asn1_type)); \
    //       return ERR_ARG;
    //   }
    request.request_type = tlv.asn1_type & SNMP_ASN1_DATATYPE_MASK;
    request.request_out_type = (SNMP_ASN1_CLASS_CONTEXT
        | SNMP_ASN1_CONTENTTYPE_CONSTRUCTED
        | SNMP_ASN1_CONTEXT_PDU_GET_RESP);

    //  validate community (do this after decoding PDU type because we don't want to increase 'inbadcommunitynames' for wrong frame types 
    if (request.community_strlen == 0) {
        //  community string was too long or really empty
        snmp_stats.inbadcommunitynames += 1;
        snmp_authfail_trap();
        return ERR_ARG;
    } else if (request.request_type == SNMP_ASN1_CONTEXT_PDU_SET_REQ) {
        if (snmp_community_write[0] == 0) {
            //  our write community is empty, that means all our objects are readonly 
            request.error_status = SNMP_ERR_NOTWRITABLE;
            request.error_index = 1;
        } else if (strncmp(
            snmp_community_write,
            request.community,
            SNMP_MAX_COMMUNITY_STR_LEN,
        ) != 0)
        {
            //  community name does not match 
            snmp_stats.inbadcommunitynames += 1;
            snmp_authfail_trap();
            return ERR_ARG;
        }
    } else {
        if (strncmp(
            snmp_community,
            request.community,
            SNMP_MAX_COMMUNITY_STR_LEN,
        ) != 0)
        {
            //  community name does not match 
            snmp_stats.inbadcommunitynames += 1;
            snmp_authfail_trap();
            return ERR_ARG;
        }
    }

    //  decode request ID 
    IF_PARSE_EXEC(snmp_asn1_dec_tlv(&pbuf_stream, &tlv));
    IF_PARSE_ASSERT(tlv.asn1_type == SNMP_ASN1_TYPE_INTEGER);
    parent_tlv_value_len -= SNMP_ASN1_TLV_LENGTH(tlv);
    IF_PARSE_ASSERT(parent_tlv_value_len > 0);

    IF_PARSE_EXEC(snmp_asn1_dec_s32t(
        &pbuf_stream,
        tlv.value_len,
        &request.request_id,
    ));

    //  decode error status / non-repeaters 
    IF_PARSE_EXEC(snmp_asn1_dec_tlv(&pbuf_stream, &tlv));
    IF_PARSE_ASSERT(tlv.asn1_type == SNMP_ASN1_TYPE_INTEGER);
    parent_tlv_value_len -= SNMP_ASN1_TLV_LENGTH(tlv);
    IF_PARSE_ASSERT(parent_tlv_value_len > 0);

    if (request.request_type == SNMP_ASN1_CONTEXT_PDU_GET_BULK_REQ) {
        IF_PARSE_EXEC(snmp_asn1_dec_s32t(
            &pbuf_stream,
            tlv.value_len,
            &request.non_repeaters,
        ));
        if (request.non_repeaters < 0) {
            //  RFC 1905, 4.2.3 
            request.non_repeaters = 0;
        }
    } else {
        //  only check valid value, don't touch 'request.error_status', maybe a response error status was already set to above; 
        IF_PARSE_EXEC(snmp_asn1_dec_s32t(&pbuf_stream, tlv.value_len, &s32_value));
        IF_PARSE_ASSERT(s32_value == SNMP_ERR_NOERROR);
    }

    //  decode error index / max-repetitions 
    IF_PARSE_EXEC(snmp_asn1_dec_tlv(&pbuf_stream, &tlv));
    IF_PARSE_ASSERT(tlv.asn1_type == SNMP_ASN1_TYPE_INTEGER);
    parent_tlv_value_len -= SNMP_ASN1_TLV_LENGTH(tlv);
    IF_PARSE_ASSERT(parent_tlv_value_len > 0);

    if (request.request_type == SNMP_ASN1_CONTEXT_PDU_GET_BULK_REQ) {
        IF_PARSE_EXEC(snmp_asn1_dec_s32t(
            &pbuf_stream,
            tlv.value_len,
            &request.max_repetitions,
        ));
        if (request.max_repetitions < 0) {
            //  RFC 1905, 4.2.3 
            request.max_repetitions = 0;
        }
    } else {
        IF_PARSE_EXEC(snmp_asn1_dec_s32t(
            &pbuf_stream,
            tlv.value_len,
            &request.error_index,
        ));
        IF_PARSE_ASSERT(s32_value == 0);
    }

    //  decode varbind-list type (next container level) 
    IF_PARSE_EXEC(snmp_asn1_dec_tlv(&pbuf_stream, &tlv));
    IF_PARSE_ASSERT(
        (tlv.asn1_type == SNMP_ASN1_TYPE_SEQUENCE) && (tlv.value_len <= pbuf_stream.length),
    );

    request.inbound_varbind_offset = pbuf_stream.offset;
    request.inbound_varbind_len = pbuf_stream.length - request.inbound_padding_len;
    snmp_vb_enumerator_init(
        &(request.inbound_varbind_enumerator),
        request.inbound_pbuf,
        request.inbound_varbind_offset,
        request.inbound_varbind_len,
    );

    return Ok(());
}

// #define OF_BUILD_EXEC(code) BUILD_EXEC(code, ERR_ARG)

pub fn snmp_prepare_outbound_frame(request: &mut snmp_request) -> Result<(), LwipError> {
    let tlv: snmp_asn1_tlv;
    let pbuf_stream: &mut snmp_pbuf_stream = &(request.outbound_pbuf_stream);

    //  try allocating pbuf(s) for maximum response size 
    request.outbound_pbuf = pbuf_alloc(PBUF_TRANSPORT, 1472, PBUF_RAM);
    if (request.outbound_pbuf == None) {
        return ERR_MEM;
    }

    snmp_pbuf_stream_init(
        pbuf_stream,
        request.outbound_pbuf,
        0,
        request.outbound_pbuf.tot_len,
    );

    //  'Message' sequence 
    SNMP_ASN1_SET_TLV_PARAMS(tlv, SNMP_ASN1_TYPE_SEQUENCE, 3, 0);
    OF_BUILD_EXEC(snmp_ans1_enc_tlv(pbuf_stream, &tlv));

    //  version 
    SNMP_ASN1_SET_TLV_PARAMS(tlv, SNMP_ASN1_TYPE_INTEGER, 0, 0);
    snmp_asn1_enc_s32t_cnt(request.version, &tlv.value_len);
    OF_BUILD_EXEC(snmp_ans1_enc_tlv(pbuf_stream, &tlv));
    OF_BUILD_EXEC(snmp_asn1_enc_s32t(
        pbuf_stream,
        tlv.value_len,
        request.version,
    ));

    if (request.version < SNMP_VERSION_3) {
        //  community 
        SNMP_ASN1_SET_TLV_PARAMS(
            tlv,
            SNMP_ASN1_TYPE_OCTET_STRING,
            0,
            request.community_strlen,
        );
        OF_BUILD_EXEC(snmp_ans1_enc_tlv(pbuf_stream, &tlv));
        OF_BUILD_EXEC(snmp_asn1_enc_raw(
            pbuf_stream,
            request.community,
            request.community_strlen,
        ));
    } else {
        let id: String;

        //  globalData 
        request.outbound_msg_global_data_offset = pbuf_stream.offset;
        SNMP_ASN1_SET_TLV_PARAMS(tlv, SNMP_ASN1_TYPE_SEQUENCE, 1, 0);
        OF_BUILD_EXEC(snmp_ans1_enc_tlv(pbuf_stream, &tlv));

        //  msgID 
        SNMP_ASN1_SET_TLV_PARAMS(tlv, SNMP_ASN1_TYPE_INTEGER, 0, 1);
        snmp_asn1_enc_s32t_cnt(request.msg_id, &tlv.value_len);
        OF_BUILD_EXEC(snmp_ans1_enc_tlv(pbuf_stream, &tlv));
        OF_BUILD_EXEC(snmp_asn1_enc_s32t(
            pbuf_stream,
            tlv.value_len,
            request.msg_id,
        ));

        //  msgMaxSize 
        SNMP_ASN1_SET_TLV_PARAMS(tlv, SNMP_ASN1_TYPE_INTEGER, 0, 1);
        snmp_asn1_enc_s32t_cnt(request.msg_max_size, &tlv.value_len);
        OF_BUILD_EXEC(snmp_ans1_enc_tlv(pbuf_stream, &tlv));
        OF_BUILD_EXEC(snmp_asn1_enc_s32t(
            pbuf_stream,
            tlv.value_len,
            request.msg_max_size,
        ));

        //  msgFlags 
        SNMP_ASN1_SET_TLV_PARAMS(tlv, SNMP_ASN1_TYPE_OCTET_STRING, 0, 1);
        OF_BUILD_EXEC(snmp_ans1_enc_tlv(pbuf_stream, &tlv));
        OF_BUILD_EXEC(snmp_asn1_enc_raw(pbuf_stream, &request.msg_flags, 1));

        //  msgSecurityModel 
        SNMP_ASN1_SET_TLV_PARAMS(tlv, SNMP_ASN1_TYPE_INTEGER, 0, 1);
        snmp_asn1_enc_s32t_cnt(request.msg_security_model, &tlv.value_len);
        OF_BUILD_EXEC(snmp_ans1_enc_tlv(pbuf_stream, &tlv));
        OF_BUILD_EXEC(snmp_asn1_enc_s32t(
            pbuf_stream,
            tlv.value_len,
            request.msg_security_model,
        ));

        //  end of msgGlobalData 
        request.outbound_msg_global_data_end = pbuf_stream.offset;

        //  msgSecurityParameters 
        request.outbound_msg_security_parameters_str_offset = pbuf_stream.offset;
        SNMP_ASN1_SET_TLV_PARAMS(tlv, SNMP_ASN1_TYPE_OCTET_STRING, 1, 0);
        OF_BUILD_EXEC(snmp_ans1_enc_tlv(pbuf_stream, &tlv));

        request.outbound_msg_security_parameters_seq_offset = pbuf_stream.offset;
        SNMP_ASN1_SET_TLV_PARAMS(tlv, SNMP_ASN1_TYPE_SEQUENCE, 1, 0);
        OF_BUILD_EXEC(snmp_ans1_enc_tlv(pbuf_stream, &tlv));

        //  msgAuthoritativeEngineID 
        snmpv3_get_engine_id(&id, &request.msg_authoritative_engine_id_len);
        MEMCPY(
            request.msg_authoritative_engine_id,
            id,
            request.msg_authoritative_engine_id_len,
        );
        SNMP_ASN1_SET_TLV_PARAMS(
            tlv,
            SNMP_ASN1_TYPE_OCTET_STRING,
            0,
            request.msg_authoritative_engine_id_len,
        );
        OF_BUILD_EXEC(snmp_ans1_enc_tlv(pbuf_stream, &tlv));
        OF_BUILD_EXEC(snmp_asn1_enc_raw(
            pbuf_stream,
            request.msg_authoritative_engine_id,
            request.msg_authoritative_engine_id_len,
        ));

        request.msg_authoritative_engine_time = snmpv3_get_engine_time();
        request.msg_authoritative_engine_boots = snmpv3_get_engine_boots();

        //  msgAuthoritativeEngineBoots 
        SNMP_ASN1_SET_TLV_PARAMS(tlv, SNMP_ASN1_TYPE_INTEGER, 0, 0);
        snmp_asn1_enc_s32t_cnt(request.msg_authoritative_engine_boots, &tlv.value_len);
        OF_BUILD_EXEC(snmp_ans1_enc_tlv(pbuf_stream, &tlv));
        OF_BUILD_EXEC(snmp_asn1_enc_s32t(
            pbuf_stream,
            tlv.value_len,
            request.msg_authoritative_engine_boots,
        ));

        //  msgAuthoritativeEngineTime 
        SNMP_ASN1_SET_TLV_PARAMS(tlv, SNMP_ASN1_TYPE_INTEGER, 0, 0);
        snmp_asn1_enc_s32t_cnt(request.msg_authoritative_engine_time, &tlv.value_len);
        OF_BUILD_EXEC(snmp_ans1_enc_tlv(pbuf_stream, &tlv));
        OF_BUILD_EXEC(snmp_asn1_enc_s32t(
            pbuf_stream,
            tlv.value_len,
            request.msg_authoritative_engine_time,
        ));

        //  msgUserName 
        SNMP_ASN1_SET_TLV_PARAMS(
            tlv,
            SNMP_ASN1_TYPE_OCTET_STRING,
            0,
            request.msg_user_name_len,
        );
        OF_BUILD_EXEC(snmp_ans1_enc_tlv(pbuf_stream, &tlv));
        OF_BUILD_EXEC(snmp_asn1_enc_raw(
            pbuf_stream,
            request.msg_user_name,
            request.msg_user_name_len,
        ));

        //  msgAuthenticationParameters 
        if (request.msg_flags & SNMP_V3_AUTH_FLAG) {
            //memset(request.msg_authentication_parameters, 0, SNMP_V3_MAX_AUTH_PARAM_LENGTH);
            request.outbound_msg_authentication_parameters_offset = pbuf_stream.offset;
            SNMP_ASN1_SET_TLV_PARAMS(
                tlv,
                SNMP_ASN1_TYPE_OCTET_STRING,
                1,
                SNMP_V3_MAX_AUTH_PARAM_LENGTH,
            );
            OF_BUILD_EXEC(snmp_ans1_enc_tlv(pbuf_stream, &tlv));
            OF_BUILD_EXEC(snmp_asn1_enc_raw(
                pbuf_stream,
                request.msg_authentication_parameters,
                SNMP_V3_MAX_AUTH_PARAM_LENGTH,
            ));
        } else {
            SNMP_ASN1_SET_TLV_PARAMS(tlv, SNMP_ASN1_TYPE_OCTET_STRING, 0, 0);
            OF_BUILD_EXEC(snmp_ans1_enc_tlv(pbuf_stream, &tlv));
        }

        //  msgPrivacyParameters 
        if (request.msg_flags & SNMP_V3_PRIV_FLAG) {
            snmpv3_build_priv_param(request.msg_privacy_parameters);

            SNMP_ASN1_SET_TLV_PARAMS(
                tlv,
                SNMP_ASN1_TYPE_OCTET_STRING,
                0,
                SNMP_V3_MAX_PRIV_PARAM_LENGTH,
            );
            OF_BUILD_EXEC(snmp_ans1_enc_tlv(pbuf_stream, &tlv));
            OF_BUILD_EXEC(snmp_asn1_enc_raw(
                pbuf_stream,
                request.msg_privacy_parameters,
                SNMP_V3_MAX_PRIV_PARAM_LENGTH,
            ));
        } else {
            SNMP_ASN1_SET_TLV_PARAMS(tlv, SNMP_ASN1_TYPE_OCTET_STRING, 0, 0);
            OF_BUILD_EXEC(snmp_ans1_enc_tlv(pbuf_stream, &tlv));
        }

        //  End of msgSecurityParameters, so we can calculate the length of this sequence later 
        request.outbound_msg_security_parameters_end = pbuf_stream.offset;

        //  For encryption we have to encapsulate the payload in an octet string 
        if (request.msg_flags & SNMP_V3_PRIV_FLAG) {
            request.outbound_scoped_pdu_string_offset = pbuf_stream.offset;
            SNMP_ASN1_SET_TLV_PARAMS(tlv, SNMP_ASN1_TYPE_OCTET_STRING, 3, 0);
            OF_BUILD_EXEC(snmp_ans1_enc_tlv(pbuf_stream, &tlv));
        }

        /* Scoped PDU
         * Encryption context
         */
        request.outbound_scoped_pdu_seq_offset = pbuf_stream.offset;
        SNMP_ASN1_SET_TLV_PARAMS(tlv, SNMP_ASN1_TYPE_SEQUENCE, 3, 0);
        OF_BUILD_EXEC(snmp_ans1_enc_tlv(pbuf_stream, &tlv));

        //  contextEngineID 
        snmpv3_get_engine_id(&id, &request.context_engine_id_len);
        MEMCPY(request.context_engine_id, id, request.context_engine_id_len);
        SNMP_ASN1_SET_TLV_PARAMS(
            tlv,
            SNMP_ASN1_TYPE_OCTET_STRING,
            0,
            request.context_engine_id_len,
        );
        OF_BUILD_EXEC(snmp_ans1_enc_tlv(pbuf_stream, &tlv));
        OF_BUILD_EXEC(snmp_asn1_enc_raw(
            pbuf_stream,
            request.context_engine_id,
            request.context_engine_id_len,
        ));

        //  contextName 
        SNMP_ASN1_SET_TLV_PARAMS(
            tlv,
            SNMP_ASN1_TYPE_OCTET_STRING,
            0,
            request.context_name_len,
        );
        OF_BUILD_EXEC(snmp_ans1_enc_tlv(pbuf_stream, &tlv));
        OF_BUILD_EXEC(snmp_asn1_enc_raw(
            pbuf_stream,
            request.context_name,
            request.context_name_len,
        ));
    }

    //  'PDU' sequence 
    request.outbound_pdu_offset = pbuf_stream.offset;
    SNMP_ASN1_SET_TLV_PARAMS(tlv, request.request_out_type, 3, 0);
    OF_BUILD_EXEC(snmp_ans1_enc_tlv(pbuf_stream, &tlv));

    //  request ID 
    SNMP_ASN1_SET_TLV_PARAMS(tlv, SNMP_ASN1_TYPE_INTEGER, 0, 0);
    snmp_asn1_enc_s32t_cnt(request.request_id, &tlv.value_len);
    OF_BUILD_EXEC(snmp_ans1_enc_tlv(pbuf_stream, &tlv));
    OF_BUILD_EXEC(snmp_asn1_enc_s32t(
        pbuf_stream,
        tlv.value_len,
        request.request_id,
    ));

    //  error status 
    SNMP_ASN1_SET_TLV_PARAMS(tlv, SNMP_ASN1_TYPE_INTEGER, 0, 1);
    OF_BUILD_EXEC(snmp_ans1_enc_tlv(pbuf_stream, &tlv));
    request.outbound_error_status_offset = pbuf_stream.offset;
    OF_BUILD_EXEC(snmp_pbuf_stream_write(pbuf_stream, 0));

    //  error index 
    SNMP_ASN1_SET_TLV_PARAMS(tlv, SNMP_ASN1_TYPE_INTEGER, 0, 1);
    OF_BUILD_EXEC(snmp_ans1_enc_tlv(pbuf_stream, &tlv));
    request.outbound_error_index_offset = pbuf_stream.offset;
    OF_BUILD_EXEC(snmp_pbuf_stream_write(pbuf_stream, 0));

    //  'VarBindList' sequence 
    SNMP_ASN1_SET_TLV_PARAMS(tlv, SNMP_ASN1_TYPE_SEQUENCE, 3, 0);
    OF_BUILD_EXEC(snmp_ans1_enc_tlv(pbuf_stream, &tlv));

    request.outbound_varbind_offset = pbuf_stream.offset;

    return Ok(());
}

//  Calculate the length of a varbind list 
pub fn snmp_varbind_length(varbind: &mut snmp_varbind, len: &mut snmp_varbind_len) {
    //  calculate required lengths 
    snmp_asn1_enc_oid_cnt(varbind.oid.id, varbind.oid.len, &len.oid_value_len);
    snmp_asn1_enc_length_cnt(len.oid_value_len, &len.oid_len_len);

    if (varbind.value_len == 0) {
        len.value_value_len = 0;
    } else if (varbind.value_len & SNMP_GET_VALUE_RAW_DATA) {
        len.value_value_len = varbind.value_len & (!SNMP_GET_VALUE_RAW_DATA);
    } else {
        match (varbind.asn1_type) {
            SNMP_ASN1_TYPE_INTEGER => {
                if (varbind.value_len != sizeof) {
                    return ERR_VAL;
                }
                snmp_asn1_enc_s32t_cnt(*(varbind.value), &len.value_value_len);
            }

            SNMP_ASN1_TYPE_COUNTER | SNMP_ASN1_TYPE_GAUGE | SNMP_ASN1_TYPE_TIMETICKS => {
                if (varbind.value_len != sizeof) {
                    return ERR_VAL;
                }
                snmp_asn1_enc_u32t_cnt(*(varbind.value), &len.value_value_len);
            }

            SNMP_ASN1_TYPE_OCTET_STRING | SNMP_ASN1_TYPE_IPADDR | SNMP_ASN1_TYPE_OPAQUE => {
                len.value_value_len = varbind.value_len;
            }

            SNMP_ASN1_TYPE_NONE => {
                if (varbind.value_len != 0) {
                    return ERR_VAL;
                }
                len.value_value_len = 0;
            }

            SNMP_ASN1_TYPE_OBJECT_ID => {
                if ((varbind.value_len & 0x03) != 0) {
                    return ERR_VAL;
                }
                snmp_asn1_enc_oid_cnt(varbind.value, varbind.value_len >> 2, &len.value_value_len);
            }

            SNMP_ASN1_TYPE_COUNTER64 => {
                if (varbind.value_len != sizeof) {
                    return ERR_VAL;
                }
                snmp_asn1_enc_u64t_cnt(*varbind.value, &len.value_value_len);
            }

            _ => {
                //  unsupported type 
                return ERR_VAL;
            }
        }
    }
    snmp_asn1_enc_length_cnt(len.value_value_len, &len.value_len_len);

    len.vb_value_len =
        1 + len.oid_len_len + len.oid_value_len + 1 + len.value_len_len + len.value_value_len;
    snmp_asn1_enc_length_cnt(len.vb_value_len, &len.vb_len_len);

    return Ok(());
}

// #define OVB_BUILD_EXEC(code) BUILD_EXEC(code, ERR_ARG)

pub fn snmp_append_outbound_varbind(
    pbuf_stream: &mut snmp_pbuf_stream,
    varbind: &mut snmp_varbind,
) {
    let tlv: snmp_asn1_tlv;
    let len: snmp_varbind_len;
    let err: err_t;

    err = snmp_varbind_length(varbind, &len);

    if (err != ERR_OK) {
        return err;
    }

    /* check length already before adding first data because in case of GetBulk,
     *  data added so far is returned and therefore no partial data shall be added
     */
    if ((1 + len.vb_len_len + len.vb_value_len) > pbuf_stream.length) {
        return ERR_BUF;
    }

    //  'VarBind' sequence 
    SNMP_ASN1_SET_TLV_PARAMS(
        tlv,
        SNMP_ASN1_TYPE_SEQUENCE,
        len.vb_len_len,
        len.vb_value_len,
    );
    OVB_BUILD_EXEC(snmp_ans1_enc_tlv(pbuf_stream, &tlv));

    //  VarBind OID 
    SNMP_ASN1_SET_TLV_PARAMS(
        tlv,
        SNMP_ASN1_TYPE_OBJECT_ID,
        len.oid_len_len,
        len.oid_value_len,
    );
    OVB_BUILD_EXEC(snmp_ans1_enc_tlv(pbuf_stream, &tlv));
    OVB_BUILD_EXEC(snmp_asn1_enc_oid(
        pbuf_stream,
        varbind.oid.id,
        varbind.oid.len,
    ));

    //  VarBind value 
    SNMP_ASN1_SET_TLV_PARAMS(
        tlv,
        varbind.asn1_type,
        len.value_len_len,
        len.value_value_len,
    );
    OVB_BUILD_EXEC(snmp_ans1_enc_tlv(pbuf_stream, &tlv));

    if (len.value_value_len > 0) {
        if (varbind.value_len & SNMP_GET_VALUE_RAW_DATA) {
            OVB_BUILD_EXEC(snmp_asn1_enc_raw(
                pbuf_stream,
                varbind.value,
                len.value_value_len,
            ));
        } else {
            match (varbind.asn1_type) {
                SNMP_ASN1_TYPE_INTEGER => {
                    OVB_BUILD_EXEC(snmp_asn1_enc_s32t(
                        pbuf_stream,
                        len.value_value_len,
                        *(varbind.value),
                    ));
                }

                SNMP_ASN1_TYPE_COUNTER | SNMP_ASN1_TYPE_GAUGE | SNMP_ASN1_TYPE_TIMETICKS => {
                    OVB_BUILD_EXEC(snmp_asn1_enc_u32t(
                        pbuf_stream,
                        len.value_value_len,
                        *(varbind.value),
                    ));
                }

                SNMP_ASN1_TYPE_OCTET_STRING | SNMP_ASN1_TYPE_IPADDR | SNMP_ASN1_TYPE_OPAQUE => {
                    OVB_BUILD_EXEC(snmp_asn1_enc_raw(
                        pbuf_stream,
                        varbind.value,
                        len.value_value_len,
                    ));
                    len.value_value_len = varbind.value_len;
                }

                SNMP_ASN1_TYPE_OBJECT_ID => {
                    OVB_BUILD_EXEC(snmp_asn1_enc_oid(
                        pbuf_stream,
                        varbind.value,
                        varbind.value_len / sizeof,
                    ));
                }

                SNMP_ASN1_TYPE_COUNTER64 => {
                    OVB_BUILD_EXEC(snmp_asn1_enc_u64t(
                        pbuf_stream,
                        len.value_value_len,
                        *varbind.value,
                    ));
                }

                _ => {
                    LWIP_ASSERT("Unknown variable type", 0);
                }
            }
        }
    }

    return Ok(());
}

pub fn snmp_complete_outbound_frame(request: &mut snmp_request) -> Result<(), LwipError> {
    let tlv: snmp_asn1_tlv;
    let frame_size: u16;
    let outbound_padding: u8 = 0;

    if (request.version == SNMP_VERSION_1) {
        if (request.error_status != SNMP_ERR_NOERROR) {
            //  map v2c error codes to v1 compliant error code (according to RFC 2089) 
            match (request.error_status) {
                /* mapping of implementation specific "virtual" error codes
                 * (during processing of frame we already stored them in error_status field,
                 * so no need to check all varbinds here for those exceptions as suggested by RFC) */
                SNMP_ERR_NOSUCHINSTANCE | SNMP_ERR_NOSUCHOBJECT | SNMP_ERR_ENDOFMIBVIEW => {
                    request.error_status = SNMP_ERR_NOSUCHNAME;
                }

                //  mapping according to RFC 
                SNMP_ERR_WRONGVALUE
                | SNMP_ERR_WRONGENCODING
                | SNMP_ERR_WRONGTYPE
                | SNMP_ERR_WRONGLENGTH
                | SNMP_ERR_INCONSISTENTVALUE => {
                    request.error_status = SNMP_ERR_BADVALUE;
                }

                SNMP_ERR_NOACCESS
                | SNMP_ERR_NOTWRITABLE
                | SNMP_ERR_NOCREATION
                | SNMP_ERR_INCONSISTENTNAME
                | SNMP_ERR_AUTHORIZATIONERROR => {
                    request.error_status = SNMP_ERR_NOSUCHNAME;
                }

                SNMP_ERR_RESOURCEUNAVAILABLE | SNMP_ERR_COMMITFAILED | SNMP_ERR_UNDOFAILED | _ => {
                    request.error_status = SNMP_ERR_GENERROR;
                }
            }
        }
    } else {
        if (request.request_type == SNMP_ASN1_CONTEXT_PDU_SET_REQ) {
            //  map error codes to according to RFC 1905 (4.2.5.  The SetRequest-PDU) return 'NotWritable' for unknown OIDs) 
            match (request.error_status) {
                SNMP_ERR_NOSUCHINSTANCE | SNMP_ERR_NOSUCHOBJECT | SNMP_ERR_ENDOFMIBVIEW => {
                    request.error_status = SNMP_ERR_NOTWRITABLE;
                }

                _ => {}
            }
        }

        if (request.error_status >= SNMP_VARBIND_EXCEPTION_OFFSET) {
            //  should never occur because v2 frames store exceptions directly inside varbinds and not as frame error_status 
            //      LWIP_DEBUGF(SNMP_DEBUG, ("snmp_complete_outbound_frame() > Found v2 request with varbind exception code stored as error status!\n"));
            return ERR_ARG;
        }
    }

    if ((request.error_status != SNMP_ERR_NOERROR)
        || (request.request_type == SNMP_ASN1_CONTEXT_PDU_SET_REQ))
    {
        //  all inbound vars are returned in response without any modification for error responses and successful set requests
        let inbound_stream: snmp_pbuf_stream;
        OF_BUILD_EXEC(snmp_pbuf_stream_init(
            &inbound_stream,
            request.inbound_pbuf,
            request.inbound_varbind_offset,
            request.inbound_varbind_len,
        ));
        OF_BUILD_EXEC(snmp_pbuf_stream_init(
            &(request.outbound_pbuf_stream),
            request.outbound_pbuf,
            request.outbound_varbind_offset,
            request.outbound_pbuf.tot_len - request.outbound_varbind_offset,
        ));
        OF_BUILD_EXEC(snmp_pbuf_stream_writeto(
            &inbound_stream,
            &(request.outbound_pbuf_stream),
            0,
        ));
    }

    frame_size = request.outbound_pbuf_stream.offset;

    //  Calculate padding for encryption 
    if (request.version == SNMP_VERSION_3 && (request.msg_flags & SNMP_V3_PRIV_FLAG)) {
        let i: u8;
        outbound_padding =
            (8 - ((frame_size - request.outbound_scoped_pdu_seq_offset) & 0x07)) & 0x07;
        // for (i = 0; i < outbound_padding; i+= 1) {
        //   OF_BUILD_EXEC( snmp_pbuf_stream_write(&request.outbound_pbuf_stream, 0) );
        // }
    }

    //  complete missing length in 'Message' sequence ; 'Message' tlv is located at the beginning (offset 0) 
    SNMP_ASN1_SET_TLV_PARAMS(
        tlv,
        SNMP_ASN1_TYPE_SEQUENCE,
        3,
        frame_size + outbound_padding - 1 - 3,
    ); //  - type - length_len(fixed, see snmp_prepare_outbound_frame()) 
    OF_BUILD_EXEC(snmp_pbuf_stream_init(
        &(request.outbound_pbuf_stream),
        request.outbound_pbuf,
        0,
        request.outbound_pbuf.tot_len,
    ));
    OF_BUILD_EXEC(snmp_ans1_enc_tlv(&(request.outbound_pbuf_stream), &tlv));

    if (request.version == SNMP_VERSION_3) {
        //  complete missing length in 'globalData' sequence 
        //  - type - length_len(fixed, see snmp_prepare_outbound_frame()) 
        SNMP_ASN1_SET_TLV_PARAMS(
            tlv,
            SNMP_ASN1_TYPE_SEQUENCE,
            1,
            request.outbound_msg_global_data_end - request.outbound_msg_global_data_offset - 1 - 1,
        );
        OF_BUILD_EXEC(snmp_pbuf_stream_seek_abs(
            &(request.outbound_pbuf_stream),
            request.outbound_msg_global_data_offset,
        ));
        OF_BUILD_EXEC(snmp_ans1_enc_tlv(&(request.outbound_pbuf_stream), &tlv));

        //  complete missing length in 'msgSecurityParameters' sequence 
        SNMP_ASN1_SET_TLV_PARAMS(
            tlv,
            SNMP_ASN1_TYPE_OCTET_STRING,
            1,
            request.outbound_msg_security_parameters_end
                - request.outbound_msg_security_parameters_str_offset
                - 1
                - 1,
        );
        OF_BUILD_EXEC(snmp_pbuf_stream_seek_abs(
            &(request.outbound_pbuf_stream),
            request.outbound_msg_security_parameters_str_offset,
        ));
        OF_BUILD_EXEC(snmp_ans1_enc_tlv(&(request.outbound_pbuf_stream), &tlv));

        SNMP_ASN1_SET_TLV_PARAMS(
            tlv,
            SNMP_ASN1_TYPE_SEQUENCE,
            1,
            request.outbound_msg_security_parameters_end
                - request.outbound_msg_security_parameters_seq_offset
                - 1
                - 1,
        );
        OF_BUILD_EXEC(snmp_pbuf_stream_seek_abs(
            &(request.outbound_pbuf_stream),
            request.outbound_msg_security_parameters_seq_offset,
        ));
        OF_BUILD_EXEC(snmp_ans1_enc_tlv(&(request.outbound_pbuf_stream), &tlv));

        //  complete missing length in scoped PDU sequence 
        SNMP_ASN1_SET_TLV_PARAMS(
            tlv,
            SNMP_ASN1_TYPE_SEQUENCE,
            3,
            frame_size - request.outbound_scoped_pdu_seq_offset - 1 - 3,
        );
        OF_BUILD_EXEC(snmp_pbuf_stream_seek_abs(
            &(request.outbound_pbuf_stream),
            request.outbound_scoped_pdu_seq_offset,
        ));
        OF_BUILD_EXEC(snmp_ans1_enc_tlv(&(request.outbound_pbuf_stream), &tlv));
    }

    //  complete missing length in 'PDU' sequence 
    SNMP_ASN1_SET_TLV_PARAMS(
        tlv,
        request.request_out_type,
        3,
        frame_size - request.outbound_pdu_offset - 1 - 3,
    ); //  - type - length_len(fixed, see snmp_prepare_outbound_frame()) 
    OF_BUILD_EXEC(snmp_pbuf_stream_seek_abs(
        &(request.outbound_pbuf_stream),
        request.outbound_pdu_offset,
    ));
    OF_BUILD_EXEC(snmp_ans1_enc_tlv(&(request.outbound_pbuf_stream), &tlv));

    //  process and encode final error status 
    if (request.error_status != 0) {
        let len: usize;
        snmp_asn1_enc_s32t_cnt(request.error_status, &len);
        if (len != 1) {
            //  error, we only reserved one byte for it 
            return ERR_ARG;
        }
        OF_BUILD_EXEC(snmp_pbuf_stream_seek_abs(
            &(request.outbound_pbuf_stream),
            request.outbound_error_status_offset,
        ));
        OF_BUILD_EXEC(snmp_asn1_enc_s32t(
            &(request.outbound_pbuf_stream),
            len,
            request.error_status,
        ));

        //  for compatibility to v1, log statistics; in v2 (RFC 1907) these statistics are obsoleted 
        match (request.error_status) {
            SNMP_ERR_TOOBIG => {
                snmp_stats.outtoobigs += 1;
            }

            SNMP_ERR_NOSUCHNAME => {
                snmp_stats.outnosuchnames += 1;
            }

            SNMP_ERR_BADVALUE => {
                snmp_stats.outbadvalues += 1;
            }

            SNMP_ERR_GENERROR => {}
            _ => {
                snmp_stats.outgenerrs += 1;
            }
        }

        if (request.error_status == SNMP_ERR_TOOBIG) {
            request.error_index = 0; //  defined by RFC 1157 
        } else if (request.error_index == 0) {
            //  set index to varbind where error occured (if not already set before, e.g. during GetBulk processing) 
            request.error_index = request.inbound_varbind_enumerator.varbind_count;
        }
    } else {
        if (request.request_type == SNMP_ASN1_CONTEXT_PDU_SET_REQ) {
            snmp_stats.intotalsetvars += request.inbound_varbind_enumerator.varbind_count;
        } else {
            snmp_stats.intotalreqvars += request.inbound_varbind_enumerator.varbind_count;
        }
    }

    //  encode final error index
    if (request.error_index != 0) {
        let len: usize;
        snmp_asn1_enc_s32t_cnt(request.error_index, &len);
        if (len != 1) {
            //  error, we only reserved one byte for it 
            return ERR_VAL;
        }
        OF_BUILD_EXEC(snmp_pbuf_stream_seek_abs(
            &(request.outbound_pbuf_stream),
            request.outbound_error_index_offset,
        ));
        OF_BUILD_EXEC(snmp_asn1_enc_s32t(
            &(request.outbound_pbuf_stream),
            len,
            request.error_index,
        ));
    }

    //  complete missing length in 'VarBindList' sequence ; 'VarBindList' tlv is located directly before varbind offset 
    SNMP_ASN1_SET_TLV_PARAMS(
        tlv,
        SNMP_ASN1_TYPE_SEQUENCE,
        3,
        frame_size - request.outbound_varbind_offset,
    );
    OF_BUILD_EXEC(snmp_pbuf_stream_seek_abs(
        &(request.outbound_pbuf_stream),
        request.outbound_varbind_offset - 1 - 3,
    )); //  - type - length_len(fixed, see snmp_prepare_outbound_frame()) 
    OF_BUILD_EXEC(snmp_ans1_enc_tlv(&(request.outbound_pbuf_stream), &tlv));

    //  Authenticate response 

    //  Encrypt response 
    if (request.version == SNMP_VERSION_3 && (request.msg_flags & SNMP_V3_PRIV_FLAG)) {
        let key: [u8; 20];
        let algo: snmpv3_priv_algo_t;

        //  complete missing length in PDU sequence 
        OF_BUILD_EXEC(snmp_pbuf_stream_init(
            &request.outbound_pbuf_stream,
            request.outbound_pbuf,
            0,
            request.outbound_pbuf.tot_len,
        ));
        OF_BUILD_EXEC(snmp_pbuf_stream_seek_abs(
            &(request.outbound_pbuf_stream),
            request.outbound_scoped_pdu_string_offset,
        ));
        SNMP_ASN1_SET_TLV_PARAMS(
            tlv,
            SNMP_ASN1_TYPE_OCTET_STRING,
            3,
            frame_size + outbound_padding - request.outbound_scoped_pdu_string_offset - 1 - 3,
        );
        OF_BUILD_EXEC(snmp_ans1_enc_tlv(&(request.outbound_pbuf_stream), &tlv));

        OF_BUILD_EXEC(snmpv3_get_user(
            request.msg_user_name,
            None,
            None,
            &algo,
            key,
        ));

        OF_BUILD_EXEC(snmpv3_crypt(
            &request.outbound_pbuf_stream,
            tlv.value_len,
            key,
            request.msg_privacy_parameters,
            request.msg_authoritative_engine_boots,
            request.msg_authoritative_engine_time,
            algo,
            SNMP_V3_PRIV_MODE_ENCRYPT,
        ));
    }

    if (request.version == SNMP_VERSION_3 && (request.msg_flags & SNMP_V3_AUTH_FLAG)) {
        let key: [u8; 20];
        let algo: snmpv3_auth_algo_t;
        let hmac: [u8; 20];

        OF_BUILD_EXEC(snmpv3_get_user(
            request.msg_user_name,
            &algo,
            key,
            None,
            None,
        ));
        OF_BUILD_EXEC(snmp_pbuf_stream_init(
            &(request.outbound_pbuf_stream),
            request.outbound_pbuf,
            0,
            request.outbound_pbuf.tot_len,
        ));
        OF_BUILD_EXEC(snmpv3_auth(
            &request.outbound_pbuf_stream,
            frame_size + outbound_padding,
            key,
            algo,
            hmac,
        ));

        MEMCPY(
            request.msg_authentication_parameters,
            hmac,
            SNMP_V3_MAX_AUTH_PARAM_LENGTH,
        );
        OF_BUILD_EXEC(snmp_pbuf_stream_init(
            &request.outbound_pbuf_stream,
            request.outbound_pbuf,
            0,
            request.outbound_pbuf.tot_len,
        ));
        OF_BUILD_EXEC(snmp_pbuf_stream_seek_abs(
            &request.outbound_pbuf_stream,
            request.outbound_msg_authentication_parameters_offset,
        ));

        SNMP_ASN1_SET_TLV_PARAMS(
            tlv,
            SNMP_ASN1_TYPE_OCTET_STRING,
            1,
            SNMP_V3_MAX_AUTH_PARAM_LENGTH,
        );
        OF_BUILD_EXEC(snmp_ans1_enc_tlv(&request.outbound_pbuf_stream, &tlv));
        OF_BUILD_EXEC(snmp_asn1_enc_raw(
            &request.outbound_pbuf_stream,
            request.msg_authentication_parameters,
            SNMP_V3_MAX_AUTH_PARAM_LENGTH,
        ));
    }

    pbuf_realloc(request.outbound_pbuf, frame_size + outbound_padding);

    snmp_stats.outgetresponses += 1;
    snmp_stats.outpkts += 1;

    return Ok(());
}

pub fn snmp_execute_write_callbacks(request: &mut snmp_request) {
    let inbound_varbind_enumerator: snmp_varbind_enumerator;
    let vb: snmp_varbind;

    snmp_vb_enumerator_init(
        &inbound_varbind_enumerator,
        request.inbound_pbuf,
        request.inbound_varbind_offset,
        request.inbound_varbind_len,
    );
    vb.value = None; //  do NOT decode value (we enumerate outbound buffer here, so all varbinds have values assigned, which we don't need here) 

    while (snmp_vb_enumerator_get_next(&inbound_varbind_enumerator, &vb)
        == SNMP_VB_ENUMERATOR_ERR_OK)
    {
        snmp_write_callback(vb.oid.id, vb.oid.len, snmp_write_callback_arg);
    }
}

//  ----------------------------------------------------------------------- 
//  VarBind enumerator methods 
//  ----------------------------------------------------------------------- 

pub fn snmp_vb_enumerator_init(
    enumerator: &mut snmp_varbind_enumerator,
    p: &mut PacketBuffer,
    offset: u16,
    length: u16,
) {
    snmp_pbuf_stream_init(&(enumerator.pbuf_stream), p, offset, length);
    enumerator.varbind_count = 0;
}

// #define VB_PARSE_EXEC(code)   PARSE_EXEC(code, SNMP_VB_ENUMERATOR_ERR_ASN1ERROR)
// #define VB_PARSE_ASSERT(code) PARSE_ASSERT(code, SNMP_VB_ENUMERATOR_ERR_ASN1ERROR)

pub fn snmp_vb_enumerator_get_next(
    enumerator: &mut snmp_varbind_enumerator,
    varbind: &mut snmp_varbind,
) -> snmp_vb_enumerator_err_t {
    let tlv: snmp_asn1_tlv;
    let varbind_len: u16;
    let err: err_t;

    if (enumerator.pbuf_stream.length == 0) {
        return SNMP_VB_ENUMERATOR_ERR_EOVB;
    }
    enumerator.varbind_count += 1;

    //  decode varbind itself (parent container of a varbind) 
    VB_PARSE_EXEC(snmp_asn1_dec_tlv(&(enumerator.pbuf_stream), &tlv));
    VB_PARSE_ASSERT(
        (tlv.asn1_type == SNMP_ASN1_TYPE_SEQUENCE)
            && (tlv.value_len <= enumerator.pbuf_stream.length),
    );
    varbind_len = tlv.value_len;

    //  decode varbind name (object id) 
    VB_PARSE_EXEC(snmp_asn1_dec_tlv(&(enumerator.pbuf_stream), &tlv));
    VB_PARSE_ASSERT(
        (tlv.asn1_type == SNMP_ASN1_TYPE_OBJECT_ID)
            && (SNMP_ASN1_TLV_LENGTH(tlv) < varbind_len)
            && (tlv.value_len < enumerator.pbuf_stream.length),
    );

    VB_PARSE_EXEC(snmp_asn1_dec_oid(
        &(enumerator.pbuf_stream),
        tlv.value_len,
        varbind.oid.id,
        &(varbind.oid.len),
        SNMP_MAX_OBJ_ID_LEN,
    ));
    varbind_len -= SNMP_ASN1_TLV_LENGTH(tlv);

    //  decode varbind value (object id) 
    VB_PARSE_EXEC(snmp_asn1_dec_tlv(&(enumerator.pbuf_stream), &tlv));
    VB_PARSE_ASSERT(
        (SNMP_ASN1_TLV_LENGTH(tlv) == varbind_len)
            && (tlv.value_len <= enumerator.pbuf_stream.length),
    );
    varbind.asn1_type = tlv.asn1_type;

    //  shall the value be decoded ? 
    if (varbind.value != None) {
        match (varbind.asn1_type) {
            SNMP_ASN1_TYPE_INTEGER => {
                VB_PARSE_EXEC(snmp_asn1_dec_s32t(
                    &(enumerator.pbuf_stream),
                    tlv.value_len,
                    varbind.value,
                ));
                varbind.value_len = sizeof;
            }

            SNMP_ASN1_TYPE_COUNTER | SNMP_ASN1_TYPE_GAUGE | SNMP_ASN1_TYPE_TIMETICKS => {
                VB_PARSE_EXEC(snmp_asn1_dec_u32t(
                    &(enumerator.pbuf_stream),
                    tlv.value_len,
                    varbind.value,
                ));
                varbind.value_len = sizeof;
            }

            SNMP_ASN1_TYPE_OCTET_STRING | SNMP_ASN1_TYPE_OPAQUE => {
                err = snmp_asn1_dec_raw(
                    &(enumerator.pbuf_stream),
                    tlv.value_len,
                    varbind.value,
                    &varbind.value_len,
                    SNMP_MAX_VALUE_SIZE,
                );
                if (err == ERR_MEM) {
                    return SNMP_VB_ENUMERATOR_ERR_INVALIDLENGTH;
                }
                VB_PARSE_ASSERT(err == ERR_OK);
            }

            SNMP_ASN1_TYPE_None => {
                varbind.value_len = 0;
            }

            SNMP_ASN1_TYPE_OBJECT_ID => {
                //  misuse tlv.length_len as OID_length transporter 
                err = snmp_asn1_dec_oid(
                    &(enumerator.pbuf_stream),
                    tlv.value_len,
                    varbind.value,
                    &tlv.length_len,
                    SNMP_MAX_OBJ_ID_LEN,
                );
                if (err == ERR_MEM) {
                    return SNMP_VB_ENUMERATOR_ERR_INVALIDLENGTH;
                }
                VB_PARSE_ASSERT(err == ERR_OK);
                varbind.value_len = tlv.length_len * sizeof;
            }

            SNMP_ASN1_TYPE_IPADDR => {
                if (tlv.value_len == 4) {
                    //  must be exactly 4 octets! 
                    VB_PARSE_EXEC(snmp_asn1_dec_raw(
                        &(enumerator.pbuf_stream),
                        tlv.value_len,
                        varbind.value,
                        &varbind.value_len,
                        SNMP_MAX_VALUE_SIZE,
                    ));
                } else {
                    VB_PARSE_ASSERT(0);
                }
            }

            SNMP_ASN1_TYPE_COUNTER64 => {
                VB_PARSE_EXEC(snmp_asn1_dec_u64t(
                    &(enumerator.pbuf_stream),
                    tlv.value_len,
                    varbind.value,
                ));
                varbind.value_len = sizeof;
            }

            _ => {
                VB_PARSE_ASSERT(0);
            }
        }
    } else {
        snmp_pbuf_stream_seek(&(enumerator.pbuf_stream), tlv.value_len);
        varbind.value_len = tlv.value_len;
    }

    return SNMP_VB_ENUMERATOR_ERR_OK;
}
