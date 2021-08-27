use crate::core::ip4_addr;

/*
 * @file
 * Network Interface Sequential API module
 *
 * @defgroup netifapi NETIF API
 * @ingroup sequential_api
 * Thread-safe functions to be called from non-TCPIP threads
 *
 * @defgroup netifapi_netif NETIF related
 * @ingroup netifapi
 * To be called from non-TCPIP threads
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
 */

// #define NETIF(name)      (name)
// #define NETIFAPI_VAR_DECLARE(name)  API_VAR_DECLARE(NetIfcapi_msg, name)
// #define NETIFAPI_VAR_ALLOC(name)    API_VAR_ALLOC(NetIfcapi_msg, MEMP_NETIFAPI_MSG, name, ERR_MEM)
// #define NETIFAPI_VAR_FREE(name)     API_VAR_FREE(MEMP_NETIFAPI_MSG, name)

/*
 * Call netif_add() inside the tcpip_thread context.
 */
pub fn netifapi_do_netif_add(m: &mut tcpip_api_call_data) -> Result<(), LwipError> {
    /* cast through void* to silence alignment warnings.
     * We know it works because the structs have been instantiated as NetIfcapi_msg */
    let msg: &mut netifapi_msg = m;

    if (!netif_add(
        msg.netif,
        (msg.msg.add.ipaddr),
        (msg.msg.add.netmask),
        (msg.msg.add.gw),
        msg.msg.add.state,
        msg.msg.add.init,
        msg.msg.add.input,
    )) {
        return ERR_IF;
    } else {
       return Ok(());
    }
}

/*
 * Call netif_set_addr() inside the tcpip_thread context.
 */
pub fn netifapi_do_netif_set_addr(m: &mut tcpip_api_call_data) -> Result<(), LwipError> {
    /* cast through void* to silence alignment warnings.
     * We know it works because the structs have been instantiated as NetIfcapi_msg */
    let msg: &mut netifapi_msg = m;

    netif_set_addr(
        msg.netif,
        API_EXPR_REF(msg.msg.add.ipaddr),
        API_EXPR_REF(msg.msg.add.netmask),
        API_EXPR_REF(msg.msg.add.gw),
    );
   return Ok(());
}

/*
* Call netif_name_to_index() inside the tcpip_thread context.
*/
pub fn netifapi_do_name_to_index(m: &mut tcpip_api_call_data) -> Result<(), LwipError> {
    /* cast through void* to silence alignment warnings.
     * We know it works because the structs have been instantiated as NetIfcapi_msg */
    let msg: &mut netifapi_msg = m;

    msg.msg.ifs.index = netif_name_to_index(msg.msg.ifs.name);
   return Ok(());
}

/*
* Call netif_index_to_name() inside the tcpip_thread context.
*/
pub fn netifapi_do_index_to_name(m: &mut tcpip_api_call_data) -> Result<(), LwipError> {
    /* cast through void* to silence alignment warnings.
     * We know it works because the structs have been instantiated as NetIfcapi_msg */
    let msg: &mut netifapi_msg = m;

    if (!netif_index_to_name(msg.msg.ifs.index, msg.msg.ifs.name)) {
        /* return failure via empty name */
        msg.msg.ifs.name[0] = '\0';
    }
   return Ok(());
}

/*
 * Call the "errtfunc" (or the "voidfunc" if "errtfunc" is NULL) inside the
 * tcpip_thread context.
 */
pub fn netifapi_do_netif_common(m: &mut tcpip_api_call_data) -> Result<(), LwipError> {
    /* cast through void* to silence alignment warnings.
     * We know it works because the structs have been instantiated as NetIfcapi_msg */
    let msg: &mut netifapi_msg = m;

    if (msg.msg.common.errtfunc != NULL) {
        return msg.msg.common.errtfunc(msg.netif);
    } else {
        msg.msg.common.voidfunc(msg.netif);
       return Ok(());
    }
}

/*
 * @ingroup netifapi_arp
 * Add or update an entry in the ARP cache.
 * For an update, ipaddr is used to find the cache entry.
 *
 * @param ipaddr IPv4 address of cache entry
 * @param ethaddr hardware address mapped to ipaddr
 * @param type type of ARP cache entry
 * @return ERR_OK: entry added/updated, else error from err_t
 */
pub fn netifapi_arp_add(ipaddr: &mut ip4_addr, ethaddr: &mut eth_addr, atype: netifapi_arp_entry) {
    let err: err_t;

    /* We only support permanent entries currently */

    LOCK_TCPIP_CORE();
    err = etharp_add_static_entry(ipaddr, ethaddr);
    UNLOCK_TCPIP_CORE();
    /* @todo add new vars to NetIfcapi_msg and create a 'do' func */

    err = ERR_VAL;
    return err;
}

/*
 * @ingroup netifapi_arp
 * Remove an entry in the ARP cache identified by ipaddr
 *
 * @param ipaddr IPv4 address of cache entry
 * @param type type of ARP cache entry
 * @return ERR_OK: entry removed, else error from err_t
 */
pub fn netifapi_arp_remove(ipaddr: &mut ip4_addr, atype: netifapi_arp_entry) {
    let err: err_t;
    /* We only support permanent entries currently */
    LOCK_TCPIP_CORE();
    err = etharp_remove_static_entry(ipaddr);
    UNLOCK_TCPIP_CORE();
    /* @todo add new vars to NetIfcapi_msg and create a 'do' func */

    err = ERR_VAL;
    return err;
}

/*
 * @ingroup netifapi_netif
 * Call netif_add() in a thread-safe way by running that function inside the
 * tcpip_thread context.
 *
 * @note for params @see netif_add()
 */
pub fn netifapi_netif_add(
    netif: &mut NetIfc,
    ipaddr: &mut ip4_addr,
    netmask: &mut ip4_addr,
    gw: &mut ip4_addr,
    state: &mut Vec<u8>,
    init: netif_init_fn,
    input: netif_input_fn,
) {
    let err: err_t;
    NETIFAPI_VAR_DECLARE(msg);
    NETIFAPI_VAR_ALLOC(msg);

    if (ipaddr == NULL) {
        ipaddr = IP4_ADDR_ANY4;
    }
    if (netmask == NULL) {
        netmask = IP4_ADDR_ANY4;
    }
    if (gw == NULL) {
        gw = IP4_ADDR_ANY4;
    }

    NETIFmsg.netif = netif;

    NETIFmsg.msg.add.ipaddr = NETIF(ipaddr);
    NETIFmsg.msg.add.netmask = NETIF(netmask);
    NETIFmsg.msg.add.gw = NETIF(gw);

    NETIFmsg.msg.add.state = state;
    NETIFmsg.msg.add.init = init;
    NETIFmsg.msg.add.input = input;
    err = tcpip_api_call(netifapi_do_netif_add, &msg.call);
    NETIFAPI_VAR_FREE(msg);
    return err;
}

/*
 * @ingroup netifapi_netif
 * Call netif_set_addr() in a thread-safe way by running that function inside the
 * tcpip_thread context.
 *
 * @note for params @see netif_set_addr()
 */
pub fn netifapi_netif_set_addr(
    netif: &mut NetIfc,
    ipaddr: &mut ip4_addr,
    netmask: &mut ip4_addr,
    gw: &mut ip4_addr,
) {
    let err: err_t;
    NETIFAPI_VAR_DECLARE(msg);
    NETIFAPI_VAR_ALLOC(msg);

    if (ipaddr == NULL) {
        ipaddr = IP4_ADDR_ANY4;
    }
    if (netmask == NULL) {
        netmask = IP4_ADDR_ANY4;
    }
    if (gw == NULL) {
        gw = IP4_ADDR_ANY4;
    }

    NETIFmsg.netif = netif;
    NETIFmsg.msg.add.ipaddr = NETIF(ipaddr);
    NETIFmsg.msg.add.netmask = NETIF(netmask);
    NETIFmsg.msg.add.gw = NETIF(gw);
    err = tcpip_api_call(netifapi_do_netif_set_addr, &msg.call);
    NETIFAPI_VAR_FREE(msg);
    return err;
}

/*
 * call the "errtfunc" (or the "voidfunc" if "errtfunc" is NULL) in a thread-safe
 * way by running that function inside the tcpip_thread context.
 *
 * @note use only for functions where there is only "netif" parameter.
 */
pub fn netifapi_netif_common(
    netif: &mut NetIfc,
    voidfunc: netifapi_void_fn,
    errtfunc: netifapi_errt_fn,
) {
    let err: err_t;
    let msg: Vec<u8>;
    msg.netif = netif;
    msg.msg.common.voidfunc = voidfunc;
    msg.msg.common.errtfunc = errtfunc;
    err = tcpip_api_call(netifapi_do_netif_common, &msg.call);
    return err;
}

/*
* @ingroup netifapi_netif
* Call netif_name_to_index() in a thread-safe way by running that function inside the
* tcpip_thread context.
*
* @param name the interface name of the netif
* @param idx output index of the found netif
*/
pub fn netifapi_netif_name_to_index(name: &String, idx: &mut u8) {
    let err: err_t;
    // NETIFAPI_VAR_DECLARE(msg);
    // NETIFAPI_VAR_ALLOC(msg);
    let msg: Vec<u8>;
    *idx = 0;
    strncpy(msg.msg.ifs.name, name, NETIF_NAMESIZE - 1);
    msg.msg.ifs.name[NETIF_NAMESIZE - 1] = '\0';
    msg.msg.ifs.name = name;

    err = tcpip_api_call(netifapi_do_name_to_index, &msg.call);
    if (!err) {
        *idx = NETIFmsg.msg.ifs.index;
    }
    NETIFAPI_VAR_FREE(msg);
    return err;
}

/*
* @ingroup netifapi_netif
* Call netif_index_to_name() in a thread-safe way by running that function inside the
* tcpip_thread context.
*
* @param idx the interface index of the netif
* @param name output name of the found netif, empty '\0' string if netif not found.
*             name should be of at least NETIF_NAMESIZE bytes
*/
pub fn netifapi_netif_index_to_name(idx: u8, name: &mut String) {
    let err: err_t;
    NETIFAPI_VAR_DECLARE(msg);
    NETIFAPI_VAR_ALLOC(msg);

    NETIFmsg.msg.ifs.index = idx;

    NETIFmsg.msg.ifs.name = name;

    err = tcpip_api_call(netifapi_do_index_to_name, &msg.call);

    if (!err) {
        strncpy(name, NETIFmsg.msg.ifs.name, NETIF_NAMESIZE - 1);
        name[NETIF_NAMESIZE - 1] = '\0';
    }

    NETIFAPI_VAR_FREE(msg);
    return err;
}
