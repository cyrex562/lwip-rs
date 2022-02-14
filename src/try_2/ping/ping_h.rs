//

/*
 * PING_USE_SOCKETS: Set to 1 to use sockets, otherwise the raw api is used
 */

pub const PING_USE_SOCKETS: u32 = LWIP_SOCKET;

fn ping_init(ping_addr: &mut LwipAddr);

fn ping_send_now();
