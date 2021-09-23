/* See http://www.nwlab.net/art/netio/netio.html to get the netio tool */

pub fn netio_recv(
    arg: &mut Vec<u8>,
    pcb: &mut TcpContext,
    p: &mut pbuf,
    err: err_t,
) -> Result<(), LwipError> {
    if (err == ERR_OK && p != None) {
        tcp_recved(pcb, p.tot_len);
        pbuf_free(p);
    } else {
        pbuf_free(p);
    }

    if (err == ERR_OK && p == None) {
        tcp_arg(pcb, None);
        tcp_sent(pcb, None);
        tcp_recv(pcb, None);
        tcp_close(pcb);
    }

   return Ok(());
}

pub fn netio_accept(arg: &mut Vec<u8>, pcb: &mut TcpContext, err: err_t) -> Result<(), LwipError> {
    if (pcb != None) {
        tcp_arg(pcb, None);
        tcp_sent(pcb, None);
        tcp_recv(pcb, netio_recv);
    }
   return Ok(());
}

pub fn netio_init() {
    let mut pcb: &mut TcpContext;

    pcb = tcp_new_ip_type(IPADDR_TYPE_ANY);
    tcp_bind(pcb, IP_ANY_TYPE, 18767);
    pcb = tcp_listen(pcb);
    tcp_accept(pcb, netio_accept);
}
