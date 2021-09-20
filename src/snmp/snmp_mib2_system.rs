/*
 * @file
 * Management Information Base II (RFC1213) SYSTEM objects and functions.
 */

/*
 * Copyright (c) 2006 Axon Digital Design B.V., The Netherlands.
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
 * Author: Dirk Ziegelmeier <dziegel@gmx.de>
 *         Christiaan Simons <christiaan.simons@axon.tv>
 */

// #define SYNC_NODE_NAME(node_name) node_name ## _synced
// #define CREATE_LWIP_SYNC_NODE(oid, node_name) \
//    static const struct snmp_threadsync_node node_name ## _synced = SNMP_CREATE_THREAD_SYNC_NODE(oid, &node_name.node, &snmp_mib2_lwip_locks);

// #define SYNC_NODE_NAME(node_name) node_name
// #define CREATE_LWIP_SYNC_NODE(oid, node_name)

/* --- system .1.3.6.1.2.1.1 ----------------------------------------------------- */

/* mib-2.system.sysDescr */
// static const u8   sysdescr_default[] = SNMP_LWIP_MIB2_SYSDESC;
// static const u8  *sysdescr           = sysdescr_default;
// static const sysdescr_len: &mut u16       = None; /* use strlen for determining len */
/* mib-2.system.sysContact */
// static const u8   syscontact_default[]     = SNMP_LWIP_MIB2_SYSCONTACT;
// static const u8  *syscontact               = syscontact_default;
// static const syscontact_len: &mut u16           = None; /* use strlen for determining len */
// static u8        *syscontact_wr            = None; /* if writable, points to the same buffer as syscontact (required for correct constness) */
// static u16       *syscontact_wr_len        = None; /* if writable, points to the same buffer as syscontact_len (required for correct constness) */
// static u16        syscontact_bufsize       = 0;    /* 0=not writable */
/* mib-2.system.sysName */
// static const u8   sysname_default[]        = SNMP_LWIP_MIB2_SYSNAME;
// static const u8  *sysname                  = sysname_default;
// static const sysname_len: &mut u16              = None; /* use strlen for determining len */
// static u8        *sysname_wr               = None; /* if writable, points to the same buffer as sysname (required for correct constness) */
// static u16       *sysname_wr_len           = None; /* if writable, points to the same buffer as sysname_len (required for correct constness) */
// static u16        sysname_bufsize          = 0;    /* 0=not writable */
/* mib-2.system.sysLocation */
// static const u8   syslocation_default[]    = SNMP_LWIP_MIB2_SYSLOCATION;
// static const u8  *syslocation              = syslocation_default;
// static const syslocation_len: &mut u16           = None; /* use strlen for determining len */
// static u8        *syslocation_wr            = None; /* if writable, points to the same buffer as syslocation (required for correct constness) */
// static u16       *syslocation_wr_len        = None; /* if writable, points to the same buffer as syslocation_len (required for correct constness) */
// static u16        syslocation_bufsize       = 0;    /* 0=not writable */
/*
 * @ingroup snmp_mib2
 * Initializes sysDescr pointers.
 *
 * @param str if non-NULL then copy str pointer
 * @param len points to string length, excluding zero terminator
 */
pub fn snmp_mib2_set_sysdescr(str: &mut Vec<u8>, len: &mut u16) {
    if (str != None) {
        sysdescr = str;
        sysdescr_len = len;
    }
}

/*
 * @ingroup snmp_mib2
 * Initializes sysContact pointers
 *
 * @param ocstr if non-NULL then copy str pointer
 * @param ocstrlen points to string length, excluding zero terminator.
 *        if set to NULL it is assumed that ocstr is NULL-terminated.
 * @param bufsize size of the buffer in bytes.
 *        (this is required because the buffer can be overwritten by snmp-set)
 *        if ocstrlen is NULL buffer needs space for terminating 0 byte.
 *        otherwise complete buffer is used for string.
 *        if bufsize is set to 0, the value is regarded as read-only.
 */
pub fn snmp_mib2_set_syscontact(ocstr: &mut Vec<u8>, ocstrlen: &mut u16, bufsize: u16) {
    if (ocstr != None) {
        syscontact = ocstr;
        syscontact_wr = ocstr;
        syscontact_len = ocstrlen;
        syscontact_wr_len = ocstrlen;
        syscontact_bufsize = bufsize;
    }
}

/*
 * @ingroup snmp_mib2
 * see \ref snmp_mib2_set_syscontact but set pointer to readonly memory
 */
pub fn snmp_mib2_set_syscontact_readonly(ocstr: &mut Vec<u8>, ocstrlen: &mut u16) {
    if (ocstr != None) {
        syscontact = ocstr;
        syscontact_len = ocstrlen;
        syscontact_wr = None;
        syscontact_wr_len = None;
        syscontact_bufsize = 0;
    }
}

/*
 * @ingroup snmp_mib2
 * Initializes sysName pointers
 *
 * @param ocstr if non-NULL then copy str pointer
 * @param ocstrlen points to string length, excluding zero terminator.
 *        if set to NULL it is assumed that ocstr is NULL-terminated.
 * @param bufsize size of the buffer in bytes.
 *        (this is required because the buffer can be overwritten by snmp-set)
 *        if ocstrlen is NULL buffer needs space for terminating 0 byte.
 *        otherwise complete buffer is used for string.
 *        if bufsize is set to 0, the value is regarded as read-only.
 */
pub fn snmp_mib2_set_sysname(ocstr: &mut Vec<u8>, ocstrlen: &mut u16, bufsize: u16) {
    if (ocstr != None) {
        sysname = ocstr;
        sysname_wr = ocstr;
        sysname_len = ocstrlen;
        sysname_wr_len = ocstrlen;
        sysname_bufsize = bufsize;
    }
}

/*
 * @ingroup snmp_mib2
 * see \ref snmp_mib2_set_sysname but set pointer to readonly memory
 */
pub fn snmp_mib2_set_sysname_readonly(ocstr: &mut Vec<u8>, ocstrlen: &mut u16) {
    if (ocstr != None) {
        sysname = ocstr;
        sysname_len = ocstrlen;
        sysname_wr = None;
        sysname_wr_len = None;
        sysname_bufsize = 0;
    }
}

/*
 * @ingroup snmp_mib2
 * Initializes sysLocation pointers
 *
 * @param ocstr if non-NULL then copy str pointer
 * @param ocstrlen points to string length, excluding zero terminator.
 *        if set to NULL it is assumed that ocstr is NULL-terminated.
 * @param bufsize size of the buffer in bytes.
 *        (this is required because the buffer can be overwritten by snmp-set)
 *        if ocstrlen is NULL buffer needs space for terminating 0 byte.
 *        otherwise complete buffer is used for string.
 *        if bufsize is set to 0, the value is regarded as read-only.
 */
pub fn snmp_mib2_set_syslocation(ocstr: &mut Vec<u8>, ocstrlen: &mut u16, bufsize: u16) {
    if (ocstr != None) {
        syslocation = ocstr;
        syslocation_wr = ocstr;
        syslocation_len = ocstrlen;
        syslocation_wr_len = ocstrlen;
        syslocation_bufsize = bufsize;
    }
}

/*
 * @ingroup snmp_mib2
 * see \ref snmp_mib2_set_syslocation but set pointer to readonly memory
 */
pub fn snmp_mib2_set_syslocation_readonly(ocstr: &mut Vec<u8>, ocstrlen: &mut u16) {
    if (ocstr != None) {
        syslocation = ocstr;
        syslocation_len = ocstrlen;
        syslocation_wr = None;
        syslocation_wr_len = None;
        syslocation_bufsize = 0;
    }
}

pub fn system_get_value(node: &mut snmp_scalar_array_node_def, value: &mut Vec<u8>) {
    u8 * var = None;
    i16 * var_len;
    let result: u16;

    match (node.oid) {
        1 => {
            /* sysDescr */
            var = sysdescr;
            var_len = sysdescr_len;
        }

        2 => {
            /* sysObjectID */
            let dev_enterprise_oid: &mut snmp_obj_id = snmp_get_device_enterprise_oid();
            MEMCPY(
                value,
                dev_enterprise_oid.id,
                dev_enterprise_oid.len * sizeof,
            );
            return dev_enterprise_oid.len * sizeof;
        }
        3 => {
            /* sysUpTime */
            MIB2_COPY_SYSUPTIME_TO(value);
            return sizeof;
        }
        4 => {
            /* sysContact */
            var = syscontact;
            var_len = syscontact_len;
        }
        // break;
        5 => {
            /* sysName */
            var = sysname;
            var_len = sysname_len;
        }

        6 => {
            /* sysLocation */
            var = syslocation;
            var_len = syslocation_len;
        }

        7 => {
            /* sysServices */
            *value = SNMP_SYSSERVICES;
            return sizeof;
        }
        _ => {
            //      LWIP_DEBUGF(SNMP_MIB_DEBUG, ("system_get_value(): unknown id: %"S32_F"\n", node.oid));
            return 0;
        }
    }

    /* handle string values (OID 1,4,5 and 6) */
    LWIP_ASSERT("", (value != None));
    if (var_len == None) {
        result = strlen(var);
    } else {
        result = *var_len;
    }
    MEMCPY(value, var, result);
    return result;
}

pub fn system_set_test(node: &mut snmp_scalar_array_node_def, len: usize, value: &mut Vec<u8>) {
    let snmp_ret: err_t = SNMP_ERR_WRONGVALUE;
    let var_bufsize: &mut u16 = None;
    let mut var_wr_len: &mut u16;

    match (node.oid) {
        4 => {
            /* sysContact */
            var_bufsize = &syscontact_bufsize;
            var_wr_len = syscontact_wr_len;
        }

        5 => {
            /* sysName */
            var_bufsize = &sysname_bufsize;
            var_wr_len = sysname_wr_len;
        }

        6 => {
            /* sysLocation */
            var_bufsize = &syslocation_bufsize;
            var_wr_len = syslocation_wr_len;
        }

        _ => {
            //      LWIP_DEBUGF(SNMP_MIB_DEBUG, ("system_set_test(): unknown id: %"S32_F"\n", node.oid));
            return ret;
        }
    }

    /* check if value is writable at all */
    if (*var_bufsize > 0) {
        if (var_wr_len == None) {
            /* we have to take the terminating 0 into account */
            if (len < *var_bufsize) {
                ret = SNMP_ERR_NOERROR;
            }
        } else {
            if (len <= *var_bufsize) {
                ret = SNMP_ERR_NOERROR;
            }
        }
    } else {
        ret = SNMP_ERR_NOTWRITABLE;
    }

    return ret;
}

pub fn system_set_value(node: &mut snmp_scalar_array_node_def, len: usize, value: &mut Vec<u8>) {
    u8 * var_wr = None;
    let mut var_wr_len: &mut u16;

    match (node.oid) {
        4 => {
            /* sysContact */
            var_wr = syscontact_wr;
            var_wr_len = syscontact_wr_len;
        }

        5 => {
            /* sysName */
            var_wr = sysname_wr;
            var_wr_len = sysname_wr_len;
        }

        6 => {
            /* sysLocation */
            var_wr = syslocation_wr;
            var_wr_len = syslocation_wr_len;
        }

        _ => {
            //      LWIP_DEBUGF(SNMP_MIB_DEBUG, ("system_set_value(): unknown id: %"S32_F"\n", node.oid));
            return SNMP_ERR_GENERROR;
        }
    }

    /* no need to check size of target buffer, this was already done in set_test method */
    LWIP_ASSERT("", var_wr != None);
    MEMCPY(var_wr, value, len);

    if (var_wr_len == None) {
        /* add terminating 0 */
        var_wr[len] = 0;
    } else {
        *var_wr_len = len;
    }

    return SNMP_ERR_NOERROR;
}

pub const system_nodes: [snmp_scalar_array_node_def] = [
    snmp_scalar_array_node_def::new(1, SNMP_ASN1_TYPE_OCTET_STRING, SNMP_NODE_INSTANCE_READ_ONLY), /* sysDescr */
    snmp_scalar_array_node_def::new(2, SNMP_ASN1_TYPE_OBJECT_ID, SNMP_NODE_INSTANCE_READ_ONLY), /* sysObjectID */
    snmp_scalar_array_node_def::new(3, SNMP_ASN1_TYPE_TIMETICKS, SNMP_NODE_INSTANCE_READ_ONLY), /* sysUpTime */
    snmp_scalar_array_node_def::new(
        4,
        SNMP_ASN1_TYPE_OCTET_STRING,
        SNMP_NODE_INSTANCE_READ_WRITE,
    ), /* sysContact */
    snmp_scalar_array_node_def::new(
        5,
        SNMP_ASN1_TYPE_OCTET_STRING,
        SNMP_NODE_INSTANCE_READ_WRITE,
    ), /* sysName */
    snmp_scalar_array_node_def::new(
        6,
        SNMP_ASN1_TYPE_OCTET_STRING,
        SNMP_NODE_INSTANCE_READ_WRITE,
    ), /* sysLocation */
    snmp_scalar_array_node_def::new(7, SNMP_ASN1_TYPE_INTEGER, SNMP_NODE_INSTANCE_READ_ONLY), /* sysServices */
];

// const struct snmp_scalar_array_node snmp_mib2_system_node = SNMP_SCALAR_CREATE_ARRAY_NODE(1, system_nodes, system_get_value, system_set_test, system_set_value);
