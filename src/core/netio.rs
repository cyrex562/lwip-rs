




/* See http://www.nwlab.net/art/netio/netio.html to get the netio tool */


static err_t
netio_recv(arg: &mut Vec<u8>, pcb: &mut tcp_pcb, p: &mut pbuf, err: err_t)
{
  LWIP_UNUSED_ARG(arg);

  if (err == ERR_OK && p != NULL) {
    tcp_recved(pcb, p.tot_len);
    pbuf_free(p);
  } else {
    pbuf_free(p);
  }

  if (err == ERR_OK && p == NULL) {
    tcp_arg(pcb, NULL);
    tcp_sent(pcb, NULL);
    tcp_recv(pcb, NULL);
    tcp_close(pcb);
  }

  return ERR_OK;
}

static err_t
netio_accept(arg: &mut Vec<u8>, pcb: &mut tcp_pcb, err: err_t)
{
  LWIP_UNUSED_ARG(arg);
  LWIP_UNUSED_ARG(err);

  if (pcb != NULL) {
    tcp_arg(pcb, NULL);
    tcp_sent(pcb, NULL);
    tcp_recv(pcb, netio_recv);
  }
  return ERR_OK;
}

pub fn 
netio_init(void)
{
  pcb: &mut tcp_pcb;

  pcb = tcp_new_ip_type(IPADDR_TYPE_ANY);
  tcp_bind(pcb, IP_ANY_TYPE, 18767);
  pcb = tcp_listen(pcb);
  tcp_accept(pcb, netio_accept);
}

