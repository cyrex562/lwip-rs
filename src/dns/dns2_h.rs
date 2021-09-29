/*
 * @file
 * DNS API
 */

/*
 * lwip DNS resolver header file.

 * Author: Jim Pettinato
 *   April 2007

 * ported from uIP resolv.c Copyright (c) 2002-2003, Adam Dunkels.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. The name of the author may not be used to endorse or promote
 *    products derived from this software without specific prior
 *    written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS
 * OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE
 * GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
 * WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */


// #define LWIP_HDR_DNS_H












//  DNS timer period 
pub const DNS_TMR_INTERVAL: u64 =          1000;

//  DNS resolve types: 
pub const LWIP_DNS_ADDRTYPE_IPV4: u32 = 0;
// #define LWIP_DNS_ADDRTYPE_IPV6      1
// #define LWIP_DNS_ADDRTYPE_IPV4_IPV6 2 //  try to resolve IPv4 first, try IPv6 if IPv4 fails only 
// #define LWIP_DNS_ADDRTYPE_IPV6_IPV4 3 //  try to resolve IPv6 first, try IPv4 if IPv6 fails only 


// #define LWIP_DNS_ADDRTYPE_DEFAULT   LWIP_DNS_ADDRTYPE_IPV4_IPV6

// #elif LWIP_IPV4
// #define LWIP_DNS_ADDRTYPE_DEFAULT   LWIP_DNS_ADDRTYPE_IPV4

// #define LWIP_DNS_ADDRTYPE_DEFAULT   LWIP_DNS_ADDRTYPE_IPV6



//  struct used for local host-list 
pub struct local_hostlist_entry {
  //  static hostname 
  pub name: String,
  //  static host address in network byteorder 
  pub  addr: LwipAddr,
  // pub next: &mut local_hostlist_entry;
}

// pub fn DNS_LOCAL_HOSTLIST_ELEM(name, addr_init) {name, addr_init, NULL}


pub const DNS_LOCAL_HOSTLIST_MAX_NAMELEN: usize =  DNS_MAX_NAME_LENGTH;

pub const LOCALHOSTLIST_ELEM_SIZE: usize = ((sizeof(local_hostlist_entry) + DNS_LOCAL_HOSTLIST_MAX_NAMELEN + 1));

// extern const LwipAddr dns_mquery_v4group;

// extern const LwipAddr dns_mquery_v6group;

/* Callback which is invoked when a hostname is found.
 * A function of this type must be implemented by the application using the DNS resolver.
 * @param name pointer to the name that was looked up.
 * @param ipaddr pointer to an LwipAddr containing the IP address of the hostname,
 *        or NULL if the name could not be found (or on any other error).
 * @param callback_arg a user-specified callback argument passed to dns_gethostbyname
*/
// typedef void (*dns_found_callback)(name: &String,  ipaddr: &mut LwipAddr, callback_arg: &mut Vec<u8>);
type dns_found_callback = fn(name: &String, ipaddr: &mut LwipAddr, callback_arg: &mut Vec<u8>);

// pub fn              dns_init();
// pub fn              dns_tmr();
// pub fn              dns_setserver(numdns: u8,  dnsserver: &mut LwipAddr);
// const dns_getserver: &mut LwipAddr(numdns: u8);
// pub fn             dns_gethostbyname(hostname: &String, addr: &mut LwipAddr, dns_found_callback found, callback_arg: &mut Vec<u8>);
// pub fn             dns_gethostbyname_addrtype(hostname: &String, addr: &mut LwipAddr, dns_found_callback found, callback_arg: &mut Vec<u8>, dns_addrtype: u8);
// usize         dns_local_iterate(dns_found_callback iterator_fn, iterator_arg: &mut Vec<u8>);
// pub fn           dns_local_lookup(hostname: &String, addr: &mut LwipAddr, dns_addrtype: u8);
// int            dns_local_removehost(hostname: &String,  addr: &mut LwipAddr);
// pub fn           dns_local_addhost(hostname: &String,  addr: &mut LwipAddr);
