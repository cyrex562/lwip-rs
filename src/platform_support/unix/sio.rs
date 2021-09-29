//  Author: Magnus Ivarsson <magnus.ivarsson@volvo.com> 

//  to get rid of implicit function declarations 
pub const _XOPEN_SOURCE: u32 = 600;
// #define _GNU_SOURCE

/* build with Darwin C extensions not part of POSIX, i.e. FASYNC, SIGIO.
we can't use LWIP_UNIX_MACH because extensions need to be turned
on before any system headers (which are pulled in through cc.h)
are included */

// #define _DARWIN_C_SOURCE

/* Following #undefs are here to keep compiler from issuing warnings
about them being double defined. (They are defined in lwip/inet.h
as well as the Unix #includes below.) */
//#undef htonl
//#undef ntohl
//#undef htons
//#undef ntohs
//#undef HTONL
//#undef NTOHL
//#undef HTONS
//#undef NTOHS

pub const LWIP_HAVE_SLIPIF: u32 = 0;

// #define BAUDRATE B19200 
// #define BAUDRATE B57600 
pub const BAUDRATE: u32 = B115200;

pub const TRUE: u32 = 1;

pub const FALSE: u32 = 0;

//  for all of you who dont define SIO_DEBUG in debug.h 

pub const SIO_DEBUG: u32 = 0;

//   typedef struct siostruct_t 
//   {  
//   	sio_status_t *sio; 
//   } siostruct_t; 

//  array of (netif.state).sio structs 
// static sio_status_t statusar[4];

//  --private-functions----------------------------------------------------------------- 
/*
 * Signal handler for ttyXX0 to indicate bytes received
 * one per interface is needed since we cannot send a instance number / pointer as callback argument (?)
 */
pub fn signal_handler_IO_0(status: i32) {
    //	LWIP_DEBUGF(SIO_DEBUG, ("SigHand: rxSignal channel 0\n"));
    fifoPut(&statusar[0].myfifo, statusar[0].fd);
}

/*
 * Signal handler for ttyXX1 to indicate bytes received
 * one per interface is needed since we cannot send a instance number / pointer as callback argument (?)
 */
pub fn signal_handler_IO_1(status: i32) {
    //	LWIP_DEBUGF(SIO_DEBUG, ("SigHand: rxSignal channel 1\n"));
    fifoPut(&statusar[1].myfifo, statusar[1].fd);
}

/*
* Initiation of serial device
* @param device string with the device name and path, eg. "/dev/ttyS0"
* @param devnum device number
* @param siostat status
* @return file handle to serial dev.
*/
pub fn sio_init(device: &mut String, devnum: i32, siostat: &mut sio_status_t) -> i32 {
    // struct termios oldtio,newtio;
    let newtio: termios;
    let oldtio: termios;

    let saio: sigaction; //  definition of signal action 

    let letfd: i32;

    //  open the device to be non-blocking (read will return immediately) 
    fd = open(device, O_RDWR | O_NOCTTY | O_NONBLOCK);
    if (fd < 0) {
        perror(device);
        exit(-1);
    }

    //  install the signal handler before making the device asynchronous 
    match (devnum) {
        0 => {
            //			LWIP_DEBUGF( SIO_DEBUG, ("sioinit, signal_handler_IO_0\n") );
            saio.sa_handler = signal_handler_IO_0;
        }

        1 => {
            //			LWIP_DEBUGF( SIO_DEBUG, ("sioinit, signal_handler_IO_1\n") );
            saio.sa_handler = signal_handler_IO_1;
        }

        _ => {
            //			LWIP_DEBUGF( SIO_DEBUG,("sioinit, devnum not allowed\n") );
        }
    }

    saio.sa_flags = 0;

    saio.sa_restorer = None;

    sigaction(SIGIO, &saio, None);

    //  allow the process to receive SIGIO 
    if (fcntl(fd, F_SETOWN, getpid()) != 0) {
        perror(device);
        exit(-1);
    }
    /* Make the file descriptor asynchronous (the manual page says only
    O_APPEND and O_NONBLOCK, will work with F_SETFL...) */
    if (fcntl(fd, F_SETFL, FASYNC) != 0) {
        perror(device);
        exit(-1);
    }

    if (fcntl(fd, F_SETFL, 0) != 0) {
        perror(device);
        exit(-1);
    }

    tcgetattr(fd, &oldtio); //  save current port settings 
    //  set new port settings 
    //  see 'man termios' for further settings 
    //memset(&newtio, 0, sizeof(newtio));
    newtio.c_cflag = BAUDRATE | CS8 | CLOCAL | CREAD | CRTSCTS;
    newtio.c_iflag = 0;
    newtio.c_oflag = 0;
    newtio.c_lflag = 0; // ECHO; 
    newtio.c_cc[VMIN] = 1; //  Read 1 byte at a time, no timer 
    newtio.c_cc[VTIME] = 0;

    tcsetattr(fd, TCSANOW, &newtio);
    tcflush(fd, TCIOFLUSH);

    return fd;
}

/*
*
*/
pub fn sio_speed(fd: i32, speed: i32) {
    let newtio: termios;
    let oldtio: termios;
    //   fd: i32; 

    //	LWIP_DEBUGF(SIO_DEBUG, ("sio_speed[%d]: baudcode:%d enter\n", fd, speed));

    if (fd < 0) {
        //		LWIP_DEBUGF(SIO_DEBUG, ("sio_speed[%d]: fd ERROR\n", fd));
        exit(-1);
    }

    tcgetattr(fd, &oldtio); //  get current port settings 

    /* set new port settings
    	* see 'man termios' for further settings */
    //memset(&newtio, 0, sizeof(newtio));
    newtio.c_cflag = speed | CS8 | CLOCAL | CREAD; //  | CRTSCTS; 
    newtio.c_iflag = 0;
    newtio.c_oflag = 0;
    newtio.c_lflag = 0; // ECHO; 
    newtio.c_cc[VMIN] = 1; //  Read 1 byte at a time, no timer 
    newtio.c_cc[VTIME] = 0;

    tcsetattr(fd, TCSANOW, &newtio);
    tcflush(fd, TCIOFLUSH);

    //	LWIP_DEBUGF(SIO_DEBUG, ("sio_speed[%d]: leave\n", fd));
}

//  --public-functions----------------------------------------------------------------------------- 
pub fn sio_send(c: u8, siostat: &mut sio_status_t) {
    // 	siostat: &mut sio_status_t= (netif.state).sio; 

    if (write(siostat.fd, &c, 1) <= 0) {
        //		LWIP_DEBUGF(SIO_DEBUG, ("sio_send[%d]: write refused\n", siostat.fd));
    }
}

pub fn sio_send_string(str: &mut Vec<u8>, siostat: &mut sio_status_t) {
    // 	siostat: &mut sio_status_t= (netif.state).sio; 
    let len: i32 = strlen(str);

    if (write(siostat.fd, str, len) <= 0) {
        //		LWIP_DEBUGF(SIO_DEBUG, ("sio_send_string[%d]: write refused\n", siostat.fd));
    }
    //	LWIP_DEBUGF(SIO_DEBUG, ("sio_send_string[%d]: sent: %s\n", siostat.fd, str));
}

pub fn sio_flush(siostat: &mut sio_status_t) {

    //  not implemented in unix as it is not needed 
    // siostat: &mut sio_status_t= (netif.state).sio; 
}

// sio_recv: u8( netif: &mut NetIfc )
pub fn sio_recv(siostat: &mut sio_status_t) -> u8 {
    // 	siostat: &mut sio_status_t= (netif.state).sio; 
    return fifoGet(&(siostat.myfifo));
}

pub fn sio_poll(siostat: &mut sio_status_t) -> u16 {
    // 	siostat: &mut sio_status_t= (netif.state).sio;
    return fifoGetNonBlock(&(siostat.myfifo));
}

pub fn sio_expect_string(str: &mut Vec<u8>, siostat: &mut sio_status_t) {
    // 	siostat: &mut sio_status_t= (netif.state).sio;
    let c: u8;
    let finger: i32 = 0;

    //	LWIP_DEBUGF(SIO_DEBUG, ("sio_expect_string[%d]: %s\n", siostat.fd, str));
    while (1) {
        c = fifoGet(&(siostat.myfifo));
        //		LWIP_DEBUGF(SIO_DEBUG, ("_%c", c));
        if (c == str[finger]) {
            finger += 1;
        } else if (finger > 0) {
            // it might fit in the beginning? 
            if (str[0] == c) {
                finger = 1;
            }
        }
        if (0 == str[finger]) {
            break;
        } //  done, we have a match 
    }
    //	LWIP_DEBUGF(SIO_DEBUG, ("sio_expect_string[%d]: [match]\n", siostat.fd));
}

pub fn sio_write(siostat: &mut sio_status_t, buf: &mut Vec<u8>, size: u32) -> u32 {
    let wsz = write(siostat.fd, buf, size);
    // return wsz < 0 ? 0 : wsz;
}

pub fn sio_read(siostat: &mut sio_status_t, buf: &mut Vec<u8>, size: u32) -> u32 {
    let rsz = read(siostat.fd, buf, size);
    // return rsz < 0 ? 0 : rsz;
}

pub fn sio_read_abort(siostat: &mut sio_status_t) {
    printf(
        "sio_read_abort[%d]: not yet implemented for unix\n",
        siostat.fd,
    );
}

pub fn sio_open(devnum: u8) -> sio_fd_t {
    let dev: String;

    //  would be nice with dynamic memory alloc 
    let siostate: &mut sio_status_t = &statusar[devnum];
    //  	siostruct_t * tmp; 

    //  	tmp = (netif.state); 
    //  	tmp.sio = siostate; 

    //  	tmp = (netif.state); 

    //  	((tmp.sio)).fd = 0; 

    //	LWIP_DEBUGF(SIO_DEBUG, ("sio_open: for devnum %d\n", devnum));

    fifoInit(&siostate.myfifo);

    snprintf(dev, sizeof(dev), "/dev/ttyS%d", devnum);

    if ((devnum == 1) || (devnum == 0)) {
        if ((siostate.fd = sio_init(&mut dev, devnum, siostate)) == 0) {
            //			LWIP_DEBUGF(SIO_DEBUG, ("sio_open: ERROR opening serial device dev=%s\n", dev));
            abort();
            return None;
        }
    //		LWIP_DEBUGF(SIO_DEBUG, ("sio_open[%d]: dev=%s open.\n", siostate.fd, dev));
    } else if (devnum == 2) {
        let childpid: pid_t;
        let name: String;
        childpid = forkpty(&siostate.fd, name, None, None);
        if (childpid < 0) {
            perror("forkpty");
            exit(1);
        }
        if (childpid == 0) {
            execl(
                "/usr/sbin/pppd",
                "pppd",
                "ms-dns",
                "198.168.100.7",
                "local",
                "crtscts",
                "debug",
                "auth",
                "require-chap",
                "remotename",
                "lwip",
                "noauth",
                "+ipv6",
                "192.168.1.1:192.168.1.2",
                None,
            );
            perror("execl pppd");
            exit(1);
        } else {
            /*LWIP_DEBUGF(SIO_DEBUG, ("sio_open[%d]: spawned pppd pid %d on %s\n",
            siostate.fd, childpid, name));*/
        }
    } else if (devnum == 3) {
        let childpid: pid_t;
        //  create PTY pair 
        siostate.fd = posix_openpt(O_RDWR | O_NOCTTY);
        if (siostate.fd < 0) {
            perror("open pty master");
            exit(1);
        }
        if (grantpt(siostate.fd) != 0) {
            perror("grant pty master");
            exit(1);
        }
        if (unlockpt(siostate.fd) != 0) {
            perror("unlock pty master");
            exit(1);
        }
        /*LWIP_DEBUGF(SIO_DEBUG, ("sio_open[%d]: for %s\n",
        siostate.fd, ptsname(siostate.fd)));*/
        //  fork for slattach 
        childpid = fork();
        if (childpid < 0) {
            perror("fork");
            exit(1);
        }
        if (childpid == 0) {
            //  esteblish SLIP interface on host side connected to PTY slave 
            execl(
                "/sbin/slattach",
                "slattach",
                "-d",
                "-v",
                "-L",
                "-p",
                "slip",
                ptsname(siostate.fd),
                None,
            );
            perror("execl slattach");
            exit(1);
        } else {
            let letret: i32;
            let buf: String;
            /*LWIP_DEBUGF(SIO_DEBUG, ("sio_open[%d]: spawned slattach pid %d on %s\n",
            siostate.fd, childpid, ptsname(siostate.fd)));*/
            //  wait a moment for slattach startup 
            sleep(1);
            //  configure SLIP interface on host side as P2P interface 
            snprintf(
                buf,
                sizeof(buf),
                "/sbin/ifconfig sl0 mtu %d %s pointopoint %s up",
                SLIP_MAX_SIZE,
                "192.168.2.1",
                "192.168.2.2",
            );
            //		LWIP_DEBUGF(SIO_DEBUG, ("sio_open[%d]: system(\"%s\");\n", siostate.fd, buf));
            ret = system(buf);
            if (ret < 0) {
                perror("ifconfig failed");
                exit(1);
            }
        }
    } else {
        //		LWIP_DEBUGF(SIO_DEBUG, ("sio_open: device %s (%d) is not supported\n", dev, devnum));
        return None;
    }

    return siostate;
}

/*
*
*/
pub fn sio_change_baud(baud: sioBaudrates, siostat: &mut sio_status_t) {
    // 	siostat: &mut sio_status_t= (netif.state).sio;

    //	LWIP_DEBUGF(SIO_DEBUG, ("sio_change_baud[%d]\n", siostat.fd));

    match (baud) {
        SIO_BAUD_9600 => {
            sio_speed(siostat.fd, B9600);
        }

        SIO_BAUD_19200 => {
            sio_speed(siostat.fd, B19200);
        }

        SIO_BAUD_38400 => {
            sio_speed(siostat.fd, B38400);
        }

        SIO_BAUD_57600 => {
            sio_speed(siostat.fd, B57600);
        }

        SIO_BAUD_115200 => {
            sio_speed(siostat.fd, B115200);
        }

        _ => {
            /*LWIP_DEBUGF(SIO_DEBUG, ("sio_change_baud[%d]: Unknown baudrate, code:%d\n",
            siostat.fd, baud));*/
        }
    }
}
