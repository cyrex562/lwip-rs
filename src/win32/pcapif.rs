/*
 * pcapif.c - This file is part of lwIP pcapif
 *
 ****************************************************************************
 *
 * This file is derived from an example in lwIP with the following license:
 *
 * Copyright (c) 2001, Swedish Institute of Computer Science.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of the Institute nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE INSTITUTE AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE INSTITUTE OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 *
 */

/* include the port-dependent configuration */






#pragma warning( push, 3 )

#pragma warning ( pop )

/* e.g. mingw */
#define _MSC_VER 1500

#undef _MSC_VER



























/* For compatibility with old pcap */

#define PCAP_OPENFLAG_PROMISCUOUS     1


/* Set this to 0 to receive all multicast ethernet destination addresses */

#define PCAPIF_FILTER_GROUP_ADDRESSES 1


/* Set this to 1 to receive all frames (also unicast to other addresses)
 * In this mode, filtering out our own tx packets from loopback receiving
 * is done via matching rx against recent tx (memcmp).
 */

pub const PCAPIF_RECEIVE_PROMISCUOUS: u32 = 0;


/* Define those to better describe your network interface.
   For now, we use 'e0', 'e1', 'e2' and so on */
#define IFNAME0                       'e'
#define IFNAME1                       '0'

/* index of the network adapter to use for lwIP */

pub const PACKET_LIB_ADAPTER_NR: u32 = 0;


/* If 1, check link state and report it to lwIP.
 *  If 0, don't check link state (lwIP link state is always UP).
 */

#define PCAPIF_HANDLE_LINKSTATE       1


/* If 1, use PBUF_REF for RX (for testing purposes mainly).
 * For this, LWIP_SUPPORT_CUSTOM_PBUF must be enabled.
 */

pub const PCAPIF_RX_REF: u32 = 0;


/* This can be used when netif.state is used for something else in your
 * application (e.g. when wrapping a class around this interface). Just
 * make sure this define returns the state pointer set by
 * pcapif_low_level_init() (e.g. by using an offset or a callback).
 */

#define PCAPIF_GET_STATE_PTR(netif)   (netif.state)





/* Define "PHY" delay when "link up" */

pub const PCAPIF_LINKUP_DELAY: u32 = 0;


#define PCAPIF_LINKCHECK_INTERVAL_MS 500

/* link state notification macro */

#define PCAPIF_NOTIFY_LINKSTATE(netif, linkfunc) sys_timeout(PCAPIF_LINKUP_DELAY, (sys_timeout_handler)linkfunc, netif)
 /* PHY_LINKUP_DELAY */
#define PCAPIF_NOTIFY_LINKSTATE(netif, linkfunc) linkfunc(netif)




/* Define PCAPIF_RX_LOCK_LWIP and PCAPIF_RX_UNLOCK_LWIP if you need to lock the lwIP core
   before/after pbuf_alloc() or netif.input() are called on RX. */

#define PCAPIF_RX_LOCK_LWIP()


#define PCAPIF_RX_UNLOCK_LWIP()


#define ETH_MIN_FRAME_LEN      60U
#define ETH_MAX_FRAME_LEN      1518U

#define ADAPTER_NAME_LEN       128
#define ADAPTER_DESC_LEN       128



#define PCAPIF_LOOPBACKFILTER_NUM_TX_PACKETS  128

struct pcapipf_pending_packet {
  next: &mut pcapipf_pending_packet;
  len: u16;
  data: [u8;ETH_MAX_FRAME_LEN];
};


/* Packet Adapter informations */
struct pcapif_private {
  void            *input_fn_arg;
  pcap_t          *adapter;
  char             name[ADAPTER_NAME_LEN];
  char             description[ADAPTER_DESC_LEN];
  int              shutdown_called;

  volatile int     rx_run;
  volatile int     rx_running;


  link_state: &mut pcapifh_linkstate;
  enum pcapifh_link_event last_link_event;


  struct pcapipf_pending_packet packets[PCAPIF_LOOPBACKFILTER_NUM_TX_PACKETS];
  tx_packets: &mut pcapipf_pending_packet;
  free_packets: &mut pcapipf_pending_packet;

};


pub fn
pcapif_init_tx_packets(priv: &mut pcapif_private)
{
  i: i32;
  priv.tx_packets = NULL;
  priv.free_packets = NULL;
  for (i = 0; i < PCAPIF_LOOPBACKFILTER_NUM_TX_PACKETS; i+= 1) {
    pack: &mut pcapipf_pending_packet = &priv.packets[i];
    pack.len = 0;
    pack.next = priv.free_packets;
    priv.free_packets = pack;
  }
}

pub fn
pcapif_add_tx_packet(priv: &mut pcapif_private, unsigned buf: &mut String, tot_len: u16)
{
  tx: &mut pcapipf_pending_packet;
  pack: &mut pcapipf_pending_packet;
  SYS_ARCH_DECL_PROTECT(lev);

  /* get a free packet (locked) */
  SYS_ARCH_PROTECT(lev);
  pack = priv.free_packets;
  if ((pack == NULL) && (priv.tx_packets != NULL)) {
    /* no free packets, reuse the oldest */
    pack = priv.tx_packets;
    priv.tx_packets = pack.next;
  }
  LWIP_ASSERT("no free packet", pack != NULL);
  priv.free_packets = pack.next;
  pack.next = NULL;
  SYS_ARCH_UNPROTECT(lev);

  /* set up the packet (unlocked) */
  pack.len = tot_len;
  memcpy(pack.data, buf, tot_len);

  /* put the packet on the list (locked) */
  SYS_ARCH_PROTECT(lev);
  if (priv.tx_packets != NULL) {
    for (tx = priv.tx_packets; tx.next != NULL; tx = tx.next);
    LWIP_ASSERT("bug", tx != NULL);
    tx.next = pack;
  } else {
    priv.tx_packets = pack;
  }
  SYS_ARCH_UNPROTECT(lev);
}

static int
pcapif_compare_packets(pack: &mut pcapipf_pending_packet, packet: &Vec<u8>, packet_len: i32)
{
  if (pack.len == packet_len) {
    if (!memcmp(pack.data, packet, packet_len)) {
      return 1;
    }
  }
  return 0;
}

static int
pcaipf_is_tx_packet(netif: &mut netif, packet: &Vec<u8>, packet_len: i32)
{
  priv: &mut pcapif_private = (struct pcapif_private*)PCAPIF_GET_STATE_PTR(netif);
  iter: &mut pcapipf_pending_packet, *last;
  SYS_ARCH_DECL_PROTECT(lev);

  last = priv.tx_packets;
  if (last == NULL) {
    /* list is empty */
    return 0;
  }
  /* compare the first packet */
  if (pcapif_compare_packets(last, packet, packet_len)) {
    SYS_ARCH_PROTECT(lev);
    LWIP_ASSERT("list has changed", last == priv.tx_packets);
    priv.tx_packets = last.next;
    last.next = priv.free_packets;
    priv.free_packets = last;
    last.len = 0;
    SYS_ARCH_UNPROTECT(lev);
    return 1;
  }
  SYS_ARCH_PROTECT(lev);
  for (iter = last.next; iter != NULL; last = iter, iter = iter.next) {
    /* unlock while comparing (this works because we have a clean threading separation
       of adding and removing items and adding is only done at the end) */
    SYS_ARCH_UNPROTECT(lev);
    if (pcapif_compare_packets(iter, packet, packet_len)) {
      SYS_ARCH_PROTECT(lev);
      LWIP_ASSERT("last != NULL", last != NULL);
      last.next = iter.next;
      iter.next = priv.free_packets;
      priv.free_packets = iter;
      last.len = 0;
      SYS_ARCH_UNPROTECT(lev);
      return 1;
    }
    SYS_ARCH_PROTECT(lev);
  }
  SYS_ARCH_UNPROTECT(lev);
  return 0;
}
 /* PCAPIF_RECEIVE_PROMISCUOUS */
#define pcapif_init_tx_packets(priv)
#define pcapif_add_tx_packet(priv, buf, tot_len)
static int
pcaipf_is_tx_packet(netif: &mut netif, packet: &Vec<u8>, packet_len: i32)
{
  const src: &mut eth_addr = (const struct eth_addr *)packet + 1;
  if (packet_len >= (ETH_HWADDR_LEN * 2)) {
    /* Don't let feedback packets through (limitation in winpcap?) */
    if(!memcmp(src, netif.hwaddr, ETH_HWADDR_LEN)) {
      return 1;
    }
  }
  return 0;
}



struct pcapif_pbuf_custom
{
   struct pbuf_custom pc;
   struct pbuf* p;
};


/* Forward declarations. */
pub fn pcapif_input(u_user: &mut String,  pkt_header: &mut pcap_pkthdr,  u_packet: &mut String);


/* Get the index of an adapter by its network address
 *
 * @param netaddr network address of the adapter (e.g. 192.168.1.0)
 * @return index of the adapter or negative on error
 */
static int
get_adapter_index_from_addr(netaddr: &mut in_addr, guid: &mut String, guid_len: usize)
{
   pcap_if_t *alldevs;
   pcap_if_t *d;
   char errbuf[PCAP_ERRBUF_SIZE+1];
   index: i32 = 0;

   memset(guid, 0, guid_len);

   /* Retrieve the interfaces list */
   if (pcap_findalldevs(&alldevs, errbuf) == -1) {
      printf("Error in pcap_findalldevs: %s\n", errbuf);
      return -1;
   }
   /* Scan the list printing every entry */
   for (d = alldevs; d != NULL; d = d.next, index+= 1) {
      pcap_addr_t *a;
      for(a = d.addresses; a != NULL; a = a.next) {
         if (a.addr.sa_family == AF_INET) {
            ULONG a_addr = (a.addr).sin_addr.s_addr;
            ULONG a_netmask = (a.netmask).sin_addr.s_addr;
            ULONG a_netaddr = a_addr & a_netmask;
            ULONG addr = (*netaddr).s_addr;
            if (a_netaddr == addr) {
               ret: i32 = -1;
               char name[128];
               start: &mut String, *end;
               len: usize = strlen(d.name);
               if(len > 127) {
                  len = 127;
               }
               MEMCPY(name, d.name, len);
               name[len] = 0;
               start = strstr(name, "{");
               if (start != NULL) {
                  end = strstr(start, "}");
                  if (end != NULL) {
                     len: usize = end - start + 1;
                     MEMCPY(guid, start, len);
                     ret = index;
                  }
               }
               pcap_freealldevs(alldevs);
               return ret;
            }
         }
      }
   }
   printf("Network address not found.\n");

   pcap_freealldevs(alldevs);
   return -1;
}



/* Get the index of an adapter by its GUID
 *
 * @param adapter_guid GUID of the adapter
 * @return index of the adapter or negative on error
 */
static int
get_adapter_index(const char* adapter_guid)
{
  pcap_if_t *alldevs;
  pcap_if_t *d;
  char errbuf[PCAP_ERRBUF_SIZE+1];
  idx: i32 = 0;

  /* Retrieve the interfaces list */
  if (pcap_findalldevs(&alldevs, errbuf) == -1) {
    printf("Error in pcap_findalldevs: %s\n", errbuf);
    return -1;
  }
  /* Scan the list and compare name vs. adapter_guid */
  for (d = alldevs; d != NULL; d = d.next, idx+= 1) {
    if(strstr(d.name, adapter_guid)) {
      pcap_freealldevs(alldevs);
      return idx;
    }
  }
  /* not found, dump all adapters */
  printf("%d available adapters:\n", idx);
  for (d = alldevs, idx = 0; d != NULL; d = d.next, idx+= 1) {
    printf("- %d: %s\n", idx, d.name);
  }
  pcap_freealldevs(alldevs);
  return -1;
}


static pcap_t*
pcapif_open_adapter(const char* adapter_name, char* errbuf)
{
  pcap_t* adapter = pcap_open_live(adapter_name,/* name of the device */
                               65536,             /* portion of the packet to capture */
                                                  /* 65536 guarantees that the whole packet will be captured on all the link layers */
                               PCAP_OPENFLAG_PROMISCUOUS,/* promiscuous mode */

                               /*-*/1,                /* don't wait at all for lower latency */

                               1,                /* wait 1 ms in ethernetif_poll */

                               errbuf);           /* error buffer */
  return adapter;
}


pub fn
pcap_reopen_adapter(pa: &mut pcapif_private)
{
  char errbuf[PCAP_ERRBUF_SIZE+1];
  pcap_if_t *alldevs;
  if (pa.adapter != NULL) {
    pcap_close(pa.adapter);
    pa.adapter = NULL;
  }
  if (pcap_findalldevs(&alldevs, errbuf) != -1) {
    pcap_if_t *d;
    for (d = alldevs; d != NULL; d = d.next) {
      if (!strcmp(d.name, pa.name)) {
        pa.adapter = pcapif_open_adapter(pa.name, errbuf);
        if (pa.adapter == NULL) {
          printf("failed to reopen pcap adapter after failure: %s\n", errbuf);
        }
        break;
      }
    }
    pcap_freealldevs(alldevs);
  }
}


/*
 * Open a network adapter and set it up for packet input
 *
 * @param adapter_num the index of the adapter to use
 * @param arg argument to pass to input
 * @return an adapter handle on success, NULL on failure
 */
static struct pcapif_private*
pcapif_init_adapter(adapter_num: i32, arg: &mut Vec<u8>)
{
  i: i32;
  number_of_adapters: i32;
  pa: &mut pcapif_private;
  char errbuf[PCAP_ERRBUF_SIZE+1];

  pcap_if_t *alldevs;
  pcap_if_t *d;
  pcap_if_t *used_adapter = NULL;

  pa = (struct pcapif_private *)malloc(sizeof(struct pcapif_private));
  if (!pa) {
    printf("Unable to alloc the adapter!\n");
    return NULL;
  }

  memset(pa, 0, sizeof(struct pcapif_private));
  pcapif_init_tx_packets(pa);
  pa.input_fn_arg = arg;

  /* Retrieve the interfaces list */
  if (pcap_findalldevs(&alldevs, errbuf) == -1) {
    free(pa);
    return NULL; /* no adapters found */
  }
  /* get number of adapters and adapter pointer */
  for (d = alldevs, number_of_adapters = 0; d != NULL; d = d.next, number_of_adapters+= 1) {
    if (number_of_adapters == adapter_num) {
      desc: &mut String = d.description;
      len: usize;

      len = strlen(d.name);
      LWIP_ASSERT("len < ADAPTER_NAME_LEN", len < ADAPTER_NAME_LEN);
      strcpy(pa.name, d.name);

      used_adapter = d;
      /* format vendor description */
      if (desc != NULL) {
        len = strlen(desc);
        if (strstr(desc, " ' on local host") != NULL) {
          len -= 16;
        }
        else if (strstr(desc, "' on local host") != NULL) {
          len -= 15;
        }
        if (strstr(desc, "Network adapter '") == desc) {
          len -= 17;
          desc += 17;
        }
        len = LWIP_MIN(len, ADAPTER_DESC_LEN-1);
        while ((desc[len-1] == ' ') || (desc[len-1] == '\t')) {
          /* don't copy trailing whitespace */
          len -= 1;
        }
        strncpy(pa.description, desc, len);
        pa.description[len] = 0;
      } else {
        strcpy(pa.description, "<no_desc>");
      }
    }
  }


  /* Scan the list printing every entry */
  for (d = alldevs, i = 0; d != NULL; d = d.next, i+= 1) {
    desc: &mut String = d.description;
    char descBuf[128];
    len: usize;
    const char* devname = d.name;
    if (d.name == NULL) {
      devname = "<unnamed>";
    } else {
      if (strstr(devname, "\\Device\\") == devname) {
        /* windows: strip the first part */
        devname += 8;
      }
    }
    printf("%2i: %s\n", i, devname);
    if (desc != NULL) {
      /* format vendor description */
      len = strlen(desc);
      if (strstr(desc, " ' on local host") != NULL) {
        len -= 16;
      }
      else if (strstr(desc, "' on local host") != NULL) {
        len -= 15;
      }
      if (strstr(desc, "Network adapter '") == desc) {
        len -= 17;
        desc += 17;
      }
      len = LWIP_MIN(len, 127);
      while ((desc[len-1] == ' ') || (desc[len-1] == '\t')) {
        /* don't copy trailing whitespace */
        len -= 1;
      }
      strncpy(descBuf, desc, len);
      descBuf[len] = 0;
      printf("     Desc: \"%s\"\n", descBuf);
    }
  }


  /* invalid adapter index -> check this after printing the adapters */
  if (adapter_num < 0) {
    printf("Invalid adapter_num: %d\n", adapter_num);
    free(pa);
    pcap_freealldevs(alldevs);
    return NULL;
  }
  /* adapter index out of range */
  if (adapter_num >= number_of_adapters) {
    printf("Invalid adapter_num: %d\n", adapter_num);
    free(pa);
    pcap_freealldevs(alldevs);
    return NULL;
  }

  printf("Using adapter_num: %d\n", adapter_num);

  /* set up the selected adapter */

  LWIP_ASSERT("used_adapter != NULL", used_adapter != NULL);

  /* Open the device */
  pa.adapter = pcapif_open_adapter(used_adapter.name, errbuf);
  if (pa.adapter == NULL) {
    printf("\nUnable to open the adapter. %s is not supported by pcap (\"%s\").\n", used_adapter.name, errbuf);
    /* Free the device list */
    pcap_freealldevs(alldevs);
    free(pa);
    return NULL;
  }
  printf("Using adapter: \"%s\"\n", pa.description);
  pcap_freealldevs(alldevs);


  pa.link_state = pcapifh_linkstate_init(pa.name);
  pa.last_link_event = PCAPIF_LINKEVENT_UNKNOWN;


  return pa;
}


pub fn
pcapif_check_linkstate(netif_ptr: &mut ())
{
  netif: &mut netif = (struct netif*)netif_ptr;
  pa: &mut pcapif_private = (struct pcapif_private*)PCAPIF_GET_STATE_PTR(netif);
  enum pcapifh_link_event le;

  le = pcapifh_linkstate_get(pa.link_state);

  if (pa.last_link_event != le) {
    pa.last_link_event = le;
    match (le) {
      PCAPIF_LINKEVENT_UP => {
        PCAPIF_NOTIFY_LINKSTATE(netif, netif_set_link_up);
        break;
      }
      PCAPIF_LINKEVENT_DOWN => {
        PCAPIF_NOTIFY_LINKSTATE(netif, netif_set_link_down);
        break;
      }
      PCAPIF_LINKEVENT_UNKNOWN => /* fall through */
      _ =>
        break;
    }
  }
  sys_timeout(PCAPIF_LINKCHECK_INTERVAL_MS, pcapif_check_linkstate, netif);
}



/*
 * Close the adapter (no more packets can be sent or received)
 *
 * @param netif netif to shutdown
 */
pub fn 
pcapif_shutdown(netif: &mut netif)
{
  pa: &mut pcapif_private = (struct pcapif_private*)PCAPIF_GET_STATE_PTR(netif);
  if (pa) {

    pa.rx_run = 0;

    if (pa.adapter) {
      pcap_breakloop(pa.adapter);
      pcap_close(pa.adapter);
    }

    /* wait for rxthread to end */
    while(pa.rx_running);


    pcapifh_linkstate_close(pa.link_state);

    free(pa);
  }
}


/* RX running in its own thread */
pub fn
pcapif_input_thread(arg: &mut Vec<u8>)
{
  netif: &mut netif = (struct netif *)arg;
  pa: &mut pcapif_private = (struct pcapif_private*)PCAPIF_GET_STATE_PTR(netif);
  do
  {
    struct pcap_pkthdr pkt_header;
    const u_packet: &mut String = pcap_next(pa.adapter, &pkt_header);
    if(packet != NULL) {
      pcapif_input((u_char*)pa, &pkt_header, packet);
    }
  } while (pa.rx_run);
  pa.rx_running = 0;
}


/* Low-level initialization: find the correct adapter and initialize it.
 */
pub fn
pcapif_low_level_init(netif: &mut netif)
{
  my_mac_addr: [u8;ETH_HWADDR_LEN] = LWIP_MAC_ADDR_BASE;
  adapter_num: i32 = PACKET_LIB_ADAPTER_NR;
  pa: &mut pcapif_private;

  ip4_addr netaddr;
#define GUID_LEN 128
  char guid[GUID_LEN + 1];


  /* If 'state' is != NULL at this point, we assume it is an 'int' giving
     the index of the adapter to use (+ 1 because 0==NULL is invalid).
     This can be used to instantiate multiple PCAP drivers. */
  if (netif.state != NULL) {
    adapter_num = (LWIP_PTR_NUMERIC_CAST(int, netif.state)) - 1;
    if (adapter_num < 0) {
      printf("ERROR: invalid adapter index \"%d\"!\n", adapter_num);
      LWIP_ASSERT("ERROR initializing network adapter!\n", 0);
      return;
    }
  }


  memset(&guid, 0, sizeof(guid));
  PACKET_LIB_GET_ADAPTER_NETADDRESS(&netaddr);
  if (get_adapter_index_from_addr((struct in_addr *)&netaddr, guid, GUID_LEN) < 0) {
     printf("ERROR initializing network adapter, failed to get GUID for network address %s\n", ip4addr_ntoa(&netaddr));
     LWIP_ASSERT("ERROR initializing network adapter, failed to get GUID for network address!", 0);
     return;
  }
  adapter_num = get_adapter_index(guid);
  if (adapter_num < 0) {
     printf("ERROR finding network adapter with GUID \"%s\"!\n", guid);
     LWIP_ASSERT("ERROR finding network adapter with expected GUID!", 0);
     return;
  }

 /* PACKET_LIB_GET_ADAPTER_NETADDRESS */

  /* get adapter index for guid string */
  adapter_num = get_adapter_index(PACKET_LIB_ADAPTER_GUID);
  if (adapter_num < 0) {
    printf("ERROR finding network adapter with GUID \"%s\"!\n", PACKET_LIB_ADAPTER_GUID);
    LWIP_ASSERT("ERROR initializing network adapter!\n", 0);
    return;
  }



  /* Do whatever else is needed to initialize interface. */
  pa = pcapif_init_adapter(adapter_num, netif);
  if (pa == NULL) {
    printf("ERROR initializing network adapter %d!\n", adapter_num);
    LWIP_ASSERT("ERROR initializing network adapter!", 0);
    return;
  }
  netif.state = pa;

  /* change the MAC address to a unique value
     so that multiple ethernetifs are supported */
  /* @todo: this does NOT support multiple processes using this adapter! */
  my_mac_addr[ETH_HWADDR_LEN - 1] += netif.num;
  /* Copy MAC addr */
  SMEMCPY(&netif.hwaddr, my_mac_addr, ETH_HWADDR_LEN);

  /* get the initial link state of the selected interface */

  pa.last_link_event = pcapifh_linkstate_get(pa.link_state);
  if (pa.last_link_event == PCAPIF_LINKEVENT_DOWN) {
    netif_set_link_down(netif);
  } else {
    netif_set_link_up(netif);
  }
  sys_timeout(PCAPIF_LINKCHECK_INTERVAL_MS, pcapif_check_linkstate, netif);
 /* PCAPIF_HANDLE_LINKSTATE */
  /* just set the link up so that lwIP can transmit */
  netif_set_link_up(netif);



  pa.rx_run = 1;
  pa.rx_running = 1;
  sys_thread_new("pcapif_rxthread", pcapif_input_thread, netif, 0, 0);


  LWIP_DEBUGF(NETIF_DEBUG, ("pcapif: eth_addr %02X%02X%02X%02X%02X%02X\n",netif.hwaddr[0],netif.hwaddr[1],netif.hwaddr[2],netif.hwaddr[3],netif.hwaddr[4],netif.hwaddr[5]));
}

/* low_level_output():
 * Transmit a packet. The packet is contained in the pbuf that is passed to
 * the function. This pbuf might be chained.
 */
pub fn pcapif_low_level_output(netif: &mut netif, p: &mut pbuf) -> Result<(), LwipError>
{
  q: &mut pbuf;
  unsigned char buffer[ETH_MAX_FRAME_LEN + ETH_PAD_SIZE];
  unsigned buf: &mut String = buffer;
  unsigned ptr: &mut String;
  ethhdr: &mut eth_hdr;
  tot_len: u16 = p.tot_len - ETH_PAD_SIZE;
  pa: &mut pcapif_private = (struct pcapif_private*)PCAPIF_GET_STATE_PTR(netif);


  LWIP_ASSERT("p.next == NULL && p.len == p.tot_len", p.next == NULL && p.len == p.tot_len);


  /* initiate transfer */
  if ((p.len == p.tot_len) && (p.len >= ETH_MIN_FRAME_LEN + ETH_PAD_SIZE)) {
    /* no pbuf chain, don't have to copy -> faster */
    buf = &((unsigned char*)p.payload)[ETH_PAD_SIZE];
  } else {
    /* pbuf chain, copy into contiguous buffer */
    if (p.tot_len >= sizeof(buffer)) {
      LINK_STATS_INC(link.lenerr);
      LINK_STATS_INC(link.drop);
      MIB2_STATS_NETIF_INC(netif, ifoutdiscards);
      return ERR_BUF;
    }
    ptr = buffer;
    for(q = p; q != NULL; q = q.next) {
      /* Send the data from the pbuf to the interface, one pbuf at a
         time. The size of the data in each pbuf is kept in the .len
         variable. */
      /* send data from(q.payload, q.len); */
      LWIP_DEBUGF(NETIF_DEBUG, ("netif: send ptr %p q.payload %p q.len %i q.next %p\n", ptr, q.payload, q.len, (void*)q.next));
      if (q == p) {
        MEMCPY(ptr, &((char*)q.payload)[ETH_PAD_SIZE], q.len - ETH_PAD_SIZE);
        ptr += q.len - ETH_PAD_SIZE;
      } else {
        MEMCPY(ptr, q.payload, q.len);
        ptr += q.len;
      }
    }
  }

  if (tot_len < ETH_MIN_FRAME_LEN) {
    /* ensure minimal frame length */
    memset(&buf[tot_len], 0, ETH_MIN_FRAME_LEN - tot_len);
    tot_len = ETH_MIN_FRAME_LEN;
  }

  /* signal that packet should be sent */
  if (pcap_sendpacket(pa.adapter, buf, tot_len) < 0) {
    LINK_STATS_INC(link.memerr);
    LINK_STATS_INC(link.drop);
    MIB2_STATS_NETIF_INC(netif, ifoutdiscards);
    return ERR_BUF;
  }
  if (netif_is_link_up(netif)) {
    pcapif_add_tx_packet(pa, buf, tot_len);
  }

  LINK_STATS_INC(link.xmit);
  MIB2_STATS_NETIF_ADD(netif, ifoutoctets, tot_len);
  ethhdr = (struct eth_hdr *)p.payload;
  if ((ethhdr.dest.addr[0] & 1) != 0) {
    /* broadcast or multicast packet*/
    MIB2_STATS_NETIF_INC(netif, ifoutnucastpkts);
  } else {
    /* unicast packet */
    MIB2_STATS_NETIF_INC(netif, ifoutucastpkts);
  }
  return ERR_OK;
}

/* low_level_input(): Allocate a pbuf and transfer the bytes of the incoming
 * packet from the interface into the pbuf.
 */
static struct pbuf *
pcapif_low_level_input(netif: &mut netif, packet: &Vec<u8>, packet_len: i32)
{
  p: &mut pbuf, *q;
  start: i32;
  length: i32 = packet_len;
  const dest: &mut eth_addr = (const struct eth_addr*)packet;
  unicast: i32;

  const bcast: u8[] = {0xff, 0xff, 0xff, 0xff, 0xff, 0xff};
  const ipv4mcast: u8[] = {0x01, 0x00, 0x5e};
  const ipv6mcast: u8[] = {0x33, 0x33};


  if (pcaipf_is_tx_packet(netif, packet, packet_len)) {
    /* don't update counters here! */
    return NULL;
  }

  unicast = ((dest.addr[0] & 0x01) == 0);

  /* MAC filter: only let my MAC or non-unicast through (pcap receives loopback traffic, too) */
  if (memcmp(dest, &netif.hwaddr, ETH_HWADDR_LEN) &&

    (memcmp(dest, ipv4mcast, 3) || ((dest.addr[3] & 0x80) != 0)) &&
    memcmp(dest, ipv6mcast, 2) &&
    memcmp(dest, bcast, 6)
 /* PCAPIF_FILTER_GROUP_ADDRESSES */
     unicast

    ) {
    /* don't update counters here! */
    return NULL;
  }


  /* We allocate a pbuf chain of pbufs from the pool. */
  p = pbuf_alloc(PBUF_RAW, length + ETH_PAD_SIZE, PBUF_POOL);
  LWIP_DEBUGF(NETIF_DEBUG, ("netif: recv length %i p.tot_len %i\n", length, p.tot_len));

  if (p != NULL) {
    /* We iterate over the pbuf chain until we have read the entire
       packet into the pbuf. */
    start = 0;
    for (q = p; q != NULL; q = q.next) {
      copy_len: u16 = q.len;
      /* Read enough bytes to fill this pbuf in the chain. The
         available data in the pbuf is given by the q.len
         variable. */
      /* read data into(q.payload, q.len); */
      LWIP_DEBUGF(NETIF_DEBUG, ("netif: recv start %i length %i q.payload %p q.len %i q.next %p\n", start, length, q.payload, q.len, (void*)q.next));
      if (q == p) {

        LWIP_ASSERT("q.len >= ETH_PAD_SIZE", q.len >= ETH_PAD_SIZE);
        copy_len -= ETH_PAD_SIZE;

        MEMCPY(&((char*)q.payload)[ETH_PAD_SIZE], &((const char*)packet)[start], copy_len);
      } else {
        MEMCPY(q.payload, &((const char*)packet)[start], copy_len);
      }
      start += copy_len;
      length -= copy_len;
      if (length <= 0) {
        break;
      }
    }
    LINK_STATS_INC(link.recv);
    MIB2_STATS_NETIF_ADD(netif, ifinoctets, p.tot_len - ETH_PAD_SIZE);
    if (unicast) {
      MIB2_STATS_NETIF_INC(netif, ifinucastpkts);
    } else {
      MIB2_STATS_NETIF_INC(netif, ifinnucastpkts);
    }
  } else {
    /* drop packet */
    LINK_STATS_INC(link.memerr);
    LINK_STATS_INC(link.drop);
    MIB2_STATS_NETIF_INC(netif, ifindiscards);
  }

  return p;
}


pub fn
pcapif_rx_pbuf_free_custom(p: &mut pbuf)
{
  struct pcapif_pbuf_custom* ppc;
  LWIP_ASSERT("NULL pointer", p != NULL);
  ppc = (struct pcapif_pbuf_custom*)p;
  LWIP_ASSERT("NULL pointer", ppc.p != NULL);
  pbuf_free(ppc.p);
  ppc.p = NULL;
  mem_free(p);
}

static struct pbuf*
pcapif_rx_ref(struct pbuf* p)
{
  struct pcapif_pbuf_custom* ppc;
  struct pbuf* q;

  LWIP_ASSERT("NULL pointer", p != NULL);
  LWIP_ASSERT("chained pbuf not supported here", p.next == NULL);

  ppc = (struct pcapif_pbuf_custom*)mem_malloc(sizeof(struct pcapif_pbuf_custom));
  LWIP_ASSERT("out of memory for RX", ppc != NULL);
  ppc.pc.custom_free_function = pcapif_rx_pbuf_free_custom;
  ppc.p = p;

  q = pbuf_alloced_custom(PBUF_RAW, p.tot_len, PBUF_REF, &ppc.pc, p.payload, p.tot_len);
  LWIP_ASSERT("pbuf_alloced_custom returned NULL", q != NULL);
  return q;
}


/* pcapif_input: This function is called when a packet is ready to be read
 * from the interface. It uses the function low_level_input() that should
 * handle the actual reception of bytes from the network interface.
 */
pub fn
pcapif_input(u_user: &mut String,  pkt_header: &mut pcap_pkthdr,  u_packet: &mut String)
{
  pa: &mut pcapif_private = (struct pcapif_private*)user;
  packet_len: i32 = pkt_header.caplen;
  netif: &mut netif = (struct netif *)pa.input_fn_arg;
  p: &mut pbuf;

  PCAPIF_RX_LOCK_LWIP();

  /* move received packet into a new pbuf */
  p = pcapif_low_level_input(netif, packet, packet_len);
  /* if no packet could be read, silently ignore this */
  if (p != NULL) {

    p = pcapif_rx_ref(p);

    /* pass all packets to ethernet_input, which decides what packets it supports */
    if (netif.input(p, netif) != ERR_OK) {
      LWIP_DEBUGF(NETIF_DEBUG, ("ethernetif_input: IP input error\n"));
      pbuf_free(p);
    }
  }
  PCAPIF_RX_UNLOCK_LWIP();
}

/*
 * pcapif_init(): initialization function, pass to netif_add().
 */
pub fn 
pcapif_init(netif: &mut netif)
{
  static ethernetif_index: i32;

  local_index: i32;
  SYS_ARCH_DECL_PROTECT(lev);
  SYS_ARCH_PROTECT(lev);
  local_index = ethernetif_index+= 1;
  SYS_ARCH_UNPROTECT(lev);

  LWIP_ASSERT("pcapif needs an input callback", netif.input != NULL);

  netif.name[0] = IFNAME0;
  netif.name[1] = (char)(IFNAME1 + local_index);
  netif.linkoutput = pcapif_low_level_output;


  netif.output = etharp_output;
 /* LWIP_ARP */
  netif.output = NULL; /* not used for PPPoE */



  netif.output_ip6 = ethip6_output;


  /* Initialize interface hostname */
  netif_set_hostname(netif, "lwip");


  netif.mtu = 1500;
  netif.flags = NETIF_FLAG_BROADCAST | NETIF_FLAG_ETHARP | NETIF_FLAG_ETHERNET | NETIF_FLAG_IGMP;

  netif.flags |= NETIF_FLAG_MLD6;

  netif.hwaddr_len = ETH_HWADDR_LEN;

  NETIF_INIT_SNMP(netif, snmp_ifType_ethernet_csmacd, 100000000);

  /* sets link up or down based on current status */
  pcapif_low_level_init(netif);

  return ERR_OK;
}


pub fn 
pcapif_poll(netif: &mut netif)
{
  pa: &mut pcapif_private = (struct pcapif_private*)PCAPIF_GET_STATE_PTR(netif);

  ret: i32;
  do {
    if (pa.adapter != NULL) {
      ret = pcap_dispatch(pa.adapter, -1, pcapif_input, (u_char*)pa);
    } else {
      ret = -1;
    }
    if (ret < 0) {
      /* error (e.g. adapter removed or resume from standby), try to reopen the adapter */
      pcap_reopen_adapter(pa);
    }
  } while (ret > 0);

}



