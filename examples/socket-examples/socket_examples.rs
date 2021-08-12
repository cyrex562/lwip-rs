













#define SOCK_TARGET_HOST4  "192.168.0.1"



#define SOCK_TARGET_HOST6  "FE80::12:34FF:FE56:78AB"



#define SOCK_TARGET_PORT  80



#define SOCK_TARGET_MAXHTTPPAGESIZE 1024



pub const SOCKET_EXAMPLES_RUN_PARALLEL: u32 = 0;


const cmpbuf: u8[8] = {0xab, 0xab, 0xab, 0xab, 0xab, 0xab, 0xab, 0xab};

/* a helper struct to ensure memory before/after fd_set is not touched */
typedef struct _xx
{
  buf1: u8[8];
  fd_set readset;
  buf2: u8[8];
  fd_set writeset;
  buf3: u8[8];
  fd_set errset;
  buf4: u8[8];
} fdsets;

#define INIT_FDSETS(sets) do { \
  memset((sets)->buf1, 0xab, 8); \
  memset((sets)->buf2, 0xab, 8); \
  memset((sets)->buf3, 0xab, 8); \
  memset((sets)->buf4, 0xab, 8); \
}while(0)

#define CHECK_FDSETS(sets) do { \
  LWIP_ASSERT("buf1 fail", !memcmp((sets)->buf1, cmpbuf, 8)); \
  LWIP_ASSERT("buf2 fail", !memcmp((sets)->buf2, cmpbuf, 8)); \
  LWIP_ASSERT("buf3 fail", !memcmp((sets)->buf3, cmpbuf, 8)); \
  LWIP_ASSERT("buf4 fail", !memcmp((sets)->buf4, cmpbuf, 8)); \
}while(0)

static ip_addr_t dstaddr;

/* This is an example function that tests
    blocking- and nonblocking connect. */
pub fn
sockex_nonblocking_connect(arg: &mut Vec<u8>)
{

  s: i32;
  ret: i32;
  opt: i32;

  sockaddr_in6 addr;
#else /* LWIP_IPV6 */
  struct sockaddr_in addr;

  fdsets sets;
  struct timeval tv;
  ticks_a: u32, ticks_b;
  err: i32;
  const ipaddr: &mut ip_addr_t = (const ip_addr_t*)arg;
  struct pollfd fds;
  INIT_FDSETS(&sets);

  /* set up address to connect to */
  memset(&addr, 0, sizeof(addr));

  addr.sin6_len = sizeof(addr);
  addr.sin6_family = AF_INET6;
  addr.sin6_port = PP_HTONS(SOCK_TARGET_PORT);
  inet6_addr_from_ip6addr(&addr.sin6_addr, ip_2_ip6(ipaddr));
#else /* LWIP_IPV6 */
  addr.sin_len = sizeof(addr);
  addr.sin_family = AF_INET;
  addr.sin_port = PP_HTONS(SOCK_TARGET_PORT);
  inet_addr_from_ip4addr(&addr.sin_addr, ip_2_ip4(ipaddr));


  /* first try blocking: */

  /* create the socket */

  s = lwip_socket(AF_INET6, SOCK_STREAM, 0);
#else /* LWIP_IPV6 */
  s = lwip_socket(AF_INET, SOCK_STREAM, 0);

  LWIP_ASSERT("s >= 0", s >= 0);

  /* connect */
  ret = lwip_connect(s, (struct sockaddr*)&addr, sizeof(addr));
  /* should succeed */
  LWIP_ASSERT("ret == 0", ret == 0);

  /* write something */
  ret = lwip_write(s, "test", 4);
  LWIP_ASSERT("ret == 4", ret == 4);

  /* close */
  ret = lwip_close(s);
  LWIP_ASSERT("ret == 0", ret == 0);

  /* now try nonblocking and close before being connected */

  /* create the socket */

  s = lwip_socket(AF_INET6, SOCK_STREAM, 0);
#else /* LWIP_IPV6 */
  s = lwip_socket(AF_INET, SOCK_STREAM, 0);

  LWIP_ASSERT("s >= 0", s >= 0);
  /* nonblocking */
  opt = lwip_fcntl(s, F_GETFL, 0);
  LWIP_ASSERT("ret != -1", ret != -1);
  opt |= O_NONBLOCK;
  ret = lwip_fcntl(s, F_SETFL, opt);
  LWIP_ASSERT("ret != -1", ret != -1);
  /* connect */
  ret = lwip_connect(s, (struct sockaddr*)&addr, sizeof(addr));
  /* should have an error: "inprogress" */
  LWIP_ASSERT("ret == -1", ret == -1);
  err = errno;
  LWIP_ASSERT("errno == EINPROGRESS", err == EINPROGRESS);
  /* close */
  ret = lwip_close(s);
  LWIP_ASSERT("ret == 0", ret == 0);
  /* try to close again, should fail with EBADF */
  ret = lwip_close(s);
  LWIP_ASSERT("ret == -1", ret == -1);
  err = errno;
  LWIP_ASSERT("errno == EBADF", err == EBADF);
  printf("closing socket in nonblocking connect succeeded\n");

  /* now try nonblocking, connect should succeed:
     this test only works if it is fast enough, i.e. no breakpoints, please! */

  /* create the socket */

  s = lwip_socket(AF_INET6, SOCK_STREAM, 0);
#else /* LWIP_IPV6 */
  s = lwip_socket(AF_INET, SOCK_STREAM, 0);

  LWIP_ASSERT("s >= 0", s >= 0);

  /* nonblocking */
  opt = 1;
  ret = lwip_ioctl(s, FIONBIO, &opt);
  LWIP_ASSERT("ret == 0", ret == 0);

  /* connect */
  ret = lwip_connect(s, (struct sockaddr*)&addr, sizeof(addr));
  /* should have an error: "inprogress" */
  LWIP_ASSERT("ret == -1", ret == -1);
  err = errno;
  LWIP_ASSERT("errno == EINPROGRESS", err == EINPROGRESS);

  /* write should fail, too */
  ret = lwip_write(s, "test", 4);
  LWIP_ASSERT("ret == -1", ret == -1);
  err = errno;
  LWIP_ASSERT("errno == EINPROGRESS", err == EINPROGRESS);

  CHECK_FDSETS(&sets);
  FD_ZERO(&sets.readset);
  CHECK_FDSETS(&sets);
  FD_SET(s, &sets.readset);
  CHECK_FDSETS(&sets);
  FD_ZERO(&sets.writeset);
  CHECK_FDSETS(&sets);
  FD_SET(s, &sets.writeset);
  CHECK_FDSETS(&sets);
  FD_ZERO(&sets.errset);
  CHECK_FDSETS(&sets);
  FD_SET(s, &sets.errset);
  CHECK_FDSETS(&sets);
  tv.tv_sec = 0;
  tv.tv_usec = 0;
  /* select without waiting should fail */
  ret = lwip_select(s + 1, &sets.readset, &sets.writeset, &sets.errset, &tv);
  CHECK_FDSETS(&sets);
  LWIP_ASSERT("ret == 0", ret == 0);
  LWIP_ASSERT("!FD_ISSET(s, &writeset)", !FD_ISSET(s, &sets.writeset));
  LWIP_ASSERT("!FD_ISSET(s, &readset)", !FD_ISSET(s, &sets.readset));
  LWIP_ASSERT("!FD_ISSET(s, &errset)", !FD_ISSET(s, &sets.errset));

  fds.fd = s;
  fds.events = POLLIN|POLLOUT;
  fds.revents = 0;
  ret = lwip_poll(&fds, 1, 0);
  LWIP_ASSERT("ret == 0", ret == 0);
  LWIP_ASSERT("fds.revents == 0", fds.revents == 0);

  FD_ZERO(&sets.readset);
  FD_SET(s, &sets.readset);
  FD_ZERO(&sets.writeset);
  FD_SET(s, &sets.writeset);
  FD_ZERO(&sets.errset);
  FD_SET(s, &sets.errset);
  ticks_a = sys_now();
  /* select with waiting should succeed */
  ret = lwip_select(s + 1, &sets.readset, &sets.writeset, &sets.errset, NULL);
  ticks_b = sys_now();
  LWIP_ASSERT("ret == 1", ret == 1);
  LWIP_ASSERT("FD_ISSET(s, &writeset)", FD_ISSET(s, &sets.writeset));
  LWIP_ASSERT("!FD_ISSET(s, &readset)", !FD_ISSET(s, &sets.readset));
  LWIP_ASSERT("!FD_ISSET(s, &errset)", !FD_ISSET(s, &sets.errset));

  fds.fd = s;
  fds.events = POLLIN|POLLOUT;
  fds.revents = 0;
  ret = lwip_poll(&fds, 1, 0);
  LWIP_ASSERT("ret == 1", ret == 1);
  LWIP_ASSERT("fds.revents & POLLOUT", fds.revents & POLLOUT);

  /* now write should succeed */
  ret = lwip_write(s, "test", 4);
  LWIP_ASSERT("ret == 4", ret == 4);

  /* close */
  ret = lwip_close(s);
  LWIP_ASSERT("ret == 0", ret == 0);

  printf("select() needed %d ticks to return writable\n", (ticks_b - ticks_a));


  /* now try nonblocking to invalid address:
     this test only works if it is fast enough, i.e. no breakpoints, please! */

  /* create the socket */

  s = lwip_socket(AF_INET6, SOCK_STREAM, 0);
#else /* LWIP_IPV6 */
  s = lwip_socket(AF_INET, SOCK_STREAM, 0);

  LWIP_ASSERT("s >= 0", s >= 0);

  /* nonblocking */
  opt = 1;
  ret = lwip_ioctl(s, FIONBIO, &opt);
  LWIP_ASSERT("ret == 0", ret == 0);


  addr.sin6_addr.un.u8_addr[0]++; /* this should result in an invalid address */
#else /* LWIP_IPV6 */
  addr.sin_addr.s_addr++; /* this should result in an invalid address */


  /* connect */
  ret = lwip_connect(s, (struct sockaddr*)&addr, sizeof(addr));
  /* should have an error: "inprogress" */
  LWIP_ASSERT("ret == -1", ret == -1);
  err = errno;
  LWIP_ASSERT("errno == EINPROGRESS", err == EINPROGRESS);

  /* write should fail, too */
  ret = lwip_write(s, "test", 4);
  LWIP_ASSERT("ret == -1", ret == -1);
  err = errno;
  LWIP_ASSERT("errno == EINPROGRESS", err == EINPROGRESS);
  LWIP_UNUSED_ARG(err);

  FD_ZERO(&sets.readset);
  FD_SET(s, &sets.readset);
  FD_ZERO(&sets.writeset);
  FD_SET(s, &sets.writeset);
  FD_ZERO(&sets.errset);
  FD_SET(s, &sets.errset);
  tv.tv_sec = 0;
  tv.tv_usec = 0;
  /* select without waiting should fail */
  ret = lwip_select(s + 1, &sets.readset, &sets.writeset, &sets.errset, &tv);
  LWIP_ASSERT("ret == 0", ret == 0);

  FD_ZERO(&sets.readset);
  FD_SET(s, &sets.readset);
  FD_ZERO(&sets.writeset);
  FD_SET(s, &sets.writeset);
  FD_ZERO(&sets.errset);
  FD_SET(s, &sets.errset);
  ticks_a = sys_now();
  /* select with waiting should eventually succeed and return errset! */
  ret = lwip_select(s + 1, &sets.readset, &sets.writeset, &sets.errset, NULL);
  ticks_b = sys_now();
  LWIP_ASSERT("ret > 0", ret > 0);
  LWIP_ASSERT("FD_ISSET(s, &errset)", FD_ISSET(s, &sets.errset));
  /*LWIP_ASSERT("!FD_ISSET(s, &readset)", !FD_ISSET(s, &sets.readset));
  LWIP_ASSERT("!FD_ISSET(s, &writeset)", !FD_ISSET(s, &sets.writeset));*/

  /* close */
  ret = lwip_close(s);
  LWIP_ASSERT("ret == 0", ret == 0);
  LWIP_UNUSED_ARG(ret);

  printf("select() needed %d ticks to return error\n", (ticks_b - ticks_a));
  printf("sockex_nonblocking_connect finished successfully\n");
#else
  LWIP_UNUSED_ARG(arg);

}

/* This is an example function that tests
    the recv function (timeout etc.). */
pub fn
sockex_testrecv(arg: &mut Vec<u8>)
{
  s: i32;
  ret: i32;
  err: i32;

  opt: i32, opt2;
#else
  struct timeval opt, opt2;

  socklen_t opt2size;

  sockaddr_in6 addr;
#else /* LWIP_IPV6 */
  struct sockaddr_in addr;

  len: usize;
  char rxbuf[SOCK_TARGET_MAXHTTPPAGESIZE];

  fd_set readset;
  fd_set errset;
  struct timeval tv;

  const ipaddr: &mut ip_addr_t = (const ip_addr_t*)arg;

  /* set up address to connect to */
  memset(&addr, 0, sizeof(addr));

  addr.sin6_len = sizeof(addr);
  addr.sin6_family = AF_INET6;
  addr.sin6_port = PP_HTONS(SOCK_TARGET_PORT);
  inet6_addr_from_ip6addr(&addr.sin6_addr, ip_2_ip6(ipaddr));
#else /* LWIP_IPV6 */
  addr.sin_len = sizeof(addr);
  addr.sin_family = AF_INET;
  addr.sin_port = PP_HTONS(SOCK_TARGET_PORT);
  inet_addr_from_ip4addr(&addr.sin_addr, ip_2_ip4(ipaddr));


  /* first try blocking: */

  /* create the socket */

  s = lwip_socket(AF_INET6, SOCK_STREAM, 0);
#else /* LWIP_IPV6 */
  s = lwip_socket(AF_INET, SOCK_STREAM, 0);

  LWIP_ASSERT("s >= 0", s >= 0);

  /* connect */
  ret = lwip_connect(s, (struct sockaddr*)&addr, sizeof(addr));
  /* should succeed */
  LWIP_ASSERT("ret == 0", ret == 0);

  /* set recv timeout (100 ms) */

  opt = 100;
#else
  opt.tv_sec = 0;
  opt.tv_usec = 100 * 1000;

  ret = lwip_setsockopt(s, SOL_SOCKET, SO_RCVTIMEO, &opt, sizeof(opt));
  LWIP_ASSERT("ret == 0", ret == 0);

  opt2 = 0;
#else
  opt2.tv_sec = 0;
  opt2.tv_usec = 0;

  opt2size = sizeof(opt2);
  ret = lwip_getsockopt(s, SOL_SOCKET, SO_RCVTIMEO, &opt2, &opt2size);
  LWIP_ASSERT("ret == 0", ret == 0);
  LWIP_ASSERT("opt2size == sizeof(opt2)", opt2size == sizeof(opt2));

  LWIP_ASSERT("opt == opt2", opt == opt2);
#else
  LWIP_ASSERT("opt == opt2", opt.tv_sec == opt2.tv_sec);
  LWIP_ASSERT("opt == opt2", opt.tv_usec == opt2.tv_usec);


  /* write the start of a GET request */
#define SNDSTR1 "G"
  len = strlen(SNDSTR1);
  ret = lwip_write(s, SNDSTR1, len);
  LWIP_ASSERT("ret == len", ret == len);

  /* should time out if the other side is a good HTTP server */
  ret = lwip_read(s, rxbuf, 1);
  LWIP_ASSERT("ret == -1", ret == -1);
  err = errno;
  LWIP_ASSERT("errno == EAGAIN", err == EAGAIN);
  LWIP_UNUSED_ARG(err);

  /* write the rest of a GET request */
#define SNDSTR2 "ET / HTTP_1.1\r\n\r\n"
  len = strlen(SNDSTR2);
  ret = lwip_write(s, SNDSTR2, len);
  LWIP_ASSERT("ret == len", ret == len);

  /* wait a while: should be enough for the server to send a response */
  sys_msleep(1000);

  /* should not time out but receive a response */
  ret = lwip_read(s, rxbuf, SOCK_TARGET_MAXHTTPPAGESIZE);
  LWIP_ASSERT("ret > 0", ret > 0);


  /* now select should directly return because the socket is readable */
  FD_ZERO(&readset);
  FD_ZERO(&errset);
  FD_SET(s, &readset);
  FD_SET(s, &errset);
  tv.tv_sec = 10;
  tv.tv_usec = 0;
  ret = lwip_select(s + 1, &readset, NULL, &errset, &tv);
  LWIP_ASSERT("ret == 1", ret == 1);
  LWIP_ASSERT("!FD_ISSET(s, &errset)", !FD_ISSET(s, &errset));
  LWIP_ASSERT("FD_ISSET(s, &readset)", FD_ISSET(s, &readset));


  /* should not time out but receive a response */
  ret = lwip_read(s, rxbuf, SOCK_TARGET_MAXHTTPPAGESIZE);
  /* might receive a second packet for HTTP/1.1 servers */
  if (ret > 0) {
    /* should return 0: closed */
    ret = lwip_read(s, rxbuf, SOCK_TARGET_MAXHTTPPAGESIZE);
    LWIP_ASSERT("ret == 0", ret == 0);
  }

  /* close */
  ret = lwip_close(s);
  LWIP_ASSERT("ret == 0", ret == 0);
  LWIP_UNUSED_ARG(ret);

  printf("sockex_testrecv finished successfully\n");
}


/* helper struct for the 2 functions below (multithreaded: thread-argument) */
struct sockex_select_helper {
  socket: i32;
  wait_read: i32;
  expect_read: i32;
  wait_write: i32;
  expect_write: i32;
  wait_err: i32;
  expect_err: i32;
  wait_ms: i32;
  sem: sys_sem_t;
};

/* helper thread to wait for socket events using select */
pub fn
sockex_select_waiter(arg: &mut Vec<u8>)
{
  helper: &mut sockex_select_helper = (struct sockex_select_helper *)arg;
  ret: i32;
  fd_set readset;
  fd_set writeset;
  fd_set errset;
  struct timeval tv;

  LWIP_ASSERT("helper != NULL", helper != NULL);

  FD_ZERO(&readset);
  FD_ZERO(&writeset);
  FD_ZERO(&errset);
  if (helper.wait_read) {
    FD_SET(helper.socket, &readset);
  }
  if (helper.wait_write) {
    FD_SET(helper.socket, &writeset);
  }
  if (helper.wait_err) {
    FD_SET(helper.socket, &errset);
  }

  tv.tv_sec = helper.wait_ms / 1000;
  tv.tv_usec = (helper.wait_ms % 1000) * 1000;

  ret = lwip_select(helper.socket, &readset, &writeset, &errset, &tv);
  if (helper.expect_read || helper.expect_write || helper.expect_err) {
    LWIP_ASSERT("ret > 0", ret > 0);
  } else {
    LWIP_ASSERT("ret == 0", ret == 0);
  }
  LWIP_UNUSED_ARG(ret);
  if (helper.expect_read) {
    LWIP_ASSERT("FD_ISSET(helper.socket, &readset)", FD_ISSET(helper.socket, &readset));
  } else {
    LWIP_ASSERT("!FD_ISSET(helper.socket, &readset)", !FD_ISSET(helper.socket, &readset));
  }
  if (helper.expect_write) {
    LWIP_ASSERT("FD_ISSET(helper.socket, &writeset)", FD_ISSET(helper.socket, &writeset));
  } else {
    LWIP_ASSERT("!FD_ISSET(helper.socket, &writeset)", !FD_ISSET(helper.socket, &writeset));
  }
  if (helper.expect_err) {
    LWIP_ASSERT("FD_ISSET(helper.socket, &errset)", FD_ISSET(helper.socket, &errset));
  } else {
    LWIP_ASSERT("!FD_ISSET(helper.socket, &errset)", !FD_ISSET(helper.socket, &errset));
  }
  sys_sem_signal(&helper.sem);
}

/* This is an example function that tests
    more than one thread being active in select. */
pub fn
sockex_testtwoselects(arg: &mut Vec<u8>)
{
  s1: i32;
  s2: i32;
  ret: i32;

  sockaddr_in6 addr;
#else /* LWIP_IPV6 */
  struct sockaddr_in addr;

  len: usize;
  lwiperr: err_t;
  struct sockex_select_helper h1, h2, h3, h4;
  const ipaddr: &mut ip_addr_t = (const ip_addr_t*)arg;

  /* set up address to connect to */
  memset(&addr, 0, sizeof(addr));

  addr.sin6_len = sizeof(addr);
  addr.sin6_family = AF_INET6;
  addr.sin6_port = PP_HTONS(SOCK_TARGET_PORT);
  inet6_addr_from_ip6addr(&addr.sin6_addr, ip_2_ip6(ipaddr));
#else /* LWIP_IPV6 */
  addr.sin_len = sizeof(addr);
  addr.sin_family = AF_INET;
  addr.sin_port = PP_HTONS(SOCK_TARGET_PORT);
  inet_addr_from_ip4addr(&addr.sin_addr, ip_2_ip4(ipaddr));


  /* create the sockets */

  s1 = lwip_socket(AF_INET6, SOCK_STREAM, 0);
  s2 = lwip_socket(AF_INET6, SOCK_STREAM, 0);
#else /* LWIP_IPV6 */
  s1 = lwip_socket(AF_INET, SOCK_STREAM, 0);
  s2 = lwip_socket(AF_INET, SOCK_STREAM, 0);

  LWIP_ASSERT("s1 >= 0", s1 >= 0);
  LWIP_ASSERT("s2 >= 0", s2 >= 0);

  /* connect, should succeed */
  ret = lwip_connect(s1, (struct sockaddr*)&addr, sizeof(addr));
  LWIP_ASSERT("ret == 0", ret == 0);
  ret = lwip_connect(s2, (struct sockaddr*)&addr, sizeof(addr));
  LWIP_ASSERT("ret == 0", ret == 0);

  /* write the start of a GET request */
#define SNDSTR1 "G"
  len = strlen(SNDSTR1);
  ret = lwip_write(s1, SNDSTR1, len);
  LWIP_ASSERT("ret == len", ret == len);
  ret = lwip_write(s2, SNDSTR1, len);
  LWIP_ASSERT("ret == len", ret == len);
  LWIP_UNUSED_ARG(ret);

  h1.wait_read  = 1;
  h1.wait_write = 1;
  h1.wait_err   = 1;
  h1.expect_read  = 0;
  h1.expect_write = 0;
  h1.expect_err   = 0;
  lwiperr = sys_sem_new(&h1.sem, 0);
  LWIP_ASSERT("lwiperr == ERR_OK", lwiperr == ERR_OK);
  h1.socket = s1;
  h1.wait_ms = 500;

  h2 = h1;
  lwiperr = sys_sem_new(&h2.sem, 0);
  LWIP_ASSERT("lwiperr == ERR_OK", lwiperr == ERR_OK);
  h2.socket = s2;
  h2.wait_ms = 1000;

  h3 = h1;
  lwiperr = sys_sem_new(&h3.sem, 0);
  LWIP_ASSERT("lwiperr == ERR_OK", lwiperr == ERR_OK);
  h3.socket = s2;
  h3.wait_ms = 1500;

  h4 = h1;
  lwiperr = sys_sem_new(&h4.sem, 0);
  LWIP_ASSERT("lwiperr == ERR_OK", lwiperr == ERR_OK);
  LWIP_UNUSED_ARG(lwiperr);
  h4.socket = s2;
  h4.wait_ms = 2000;

  /* select: all sockets should time out if the other side is a good HTTP server */

  sys_thread_new("sockex_select_waiter1", sockex_select_waiter, &h2, 0, 0);
  sys_msleep(100);
  sys_thread_new("sockex_select_waiter2", sockex_select_waiter, &h1, 0, 0);
  sys_msleep(100);
  sys_thread_new("sockex_select_waiter2", sockex_select_waiter, &h4, 0, 0);
  sys_msleep(100);
  sys_thread_new("sockex_select_waiter2", sockex_select_waiter, &h3, 0, 0);

  sys_sem_wait(&h1.sem);
  sys_sem_wait(&h2.sem);
  sys_sem_wait(&h3.sem);
  sys_sem_wait(&h4.sem);

  /* close */
  ret = lwip_close(s1);
  LWIP_ASSERT("ret == 0", ret == 0);
  ret = lwip_close(s2);
  LWIP_ASSERT("ret == 0", ret == 0);

  printf("sockex_testtwoselects finished successfully\n");
}
#else
pub fn
sockex_testtwoselects(arg: &mut Vec<u8>)
{
  LWIP_UNUSED_ARG(arg);
}



pub fn
socket_example_test(void* arg)
{
  sys_msleep(1000);
  sockex_nonblocking_connect(arg);
  sockex_testrecv(arg);
  sockex_testtwoselects(arg);
  printf("all tests done, thread ending\n");
}


pub fn  socket_examples_init()
{
  addr_ok: i32;

  IP_SET_TYPE_VAL(dstaddr, IPADDR_TYPE_V6);
  addr_ok = ip6addr_aton(SOCK_TARGET_HOST6, ip_2_ip6(&dstaddr));
#else /* LWIP_IPV6 */
  IP_SET_TYPE_VAL(dstaddr, IPADDR_TYPE_V4);
  addr_ok = ip4addr_aton(SOCK_TARGET_HOST4, ip_2_ip4(&dstaddr));

  LWIP_ASSERT("invalid address", addr_ok);

  sys_thread_new("sockex_nonblocking_connect", sockex_nonblocking_connect, &dstaddr, 0, 0);
  sys_thread_new("sockex_testrecv", sockex_testrecv, &dstaddr, 0, 0);
  sys_thread_new("sockex_testtwoselects", sockex_testtwoselects, &dstaddr, 0, 0);
#else
  sys_thread_new("socket_example_test", socket_example_test, &dstaddr, 0, 0);

}


