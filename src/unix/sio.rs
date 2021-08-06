/* Author: Magnus Ivarsson <magnus.ivarsson@volvo.com> */

/* to get rid of implicit function declarations */
#define _XOPEN_SOURCE 600
#define _GNU_SOURCE

/* build with Darwin C extensions not part of POSIX, i.e. FASYNC, SIGIO.
   we can't use LWIP_UNIX_MACH because extensions need to be turned
   on before any system headers (which are pulled in through cc.h)
   are included */

#define _DARWIN_C_SOURCE











/* Following #undefs are here to keep compiler from issuing warnings
   about them being double defined. (They are defined in lwip/inet.h
   as well as the Unix #includes below.) */
#undef htonl
#undef ntohl
#undef htons
#undef ntohs
#undef HTONL
#undef NTOHL
#undef HTONS
#undef NTOHS
















pub const LWIP_HAVE_SLIPIF: u32 = 0;






/*#define BAUDRATE B19200 */
/*#define BAUDRATE B57600 */
#define BAUDRATE B115200


#define TRUE  1


pub const FALSE: u32 = 0;


/* for all of you who dont define SIO_DEBUG in debug.h */

pub const SIO_DEBUG: u32 = 0;



/*  typedef struct siostruct_t */
/*  {  */
/*  	sio_status_t *sio; */
/*  } siostruct_t; */

/* array of ((siostruct*)netif.state)->sio structs */
static sio_status_t statusar[4];


/* --private-functions----------------------------------------------------------------- */
/*
 * Signal handler for ttyXX0 to indicate bytes received 
 * one per interface is needed since we cannot send a instance number / pointer as callback argument (?)
 */
pub fn	signal_handler_IO_0( status: i32 )
{
	LWIP_UNUSED_ARG(status);
	LWIP_DEBUGF(SIO_DEBUG, ("SigHand: rxSignal channel 0\n"));
	fifoPut( &statusar[0].myfifo, statusar[0].fd );
}

/*
 * Signal handler for ttyXX1 to indicate bytes received 
 * one per interface is needed since we cannot send a instance number / pointer as callback argument (?)
 */
pub fn signal_handler_IO_1( status: i32 )
{
	LWIP_UNUSED_ARG(status);
	LWIP_DEBUGF(SIO_DEBUG, ("SigHand: rxSignal channel 1\n"));
	fifoPut( &statusar[1].myfifo, statusar[1].fd );
}


/*
* Initiation of serial device 
* @param device string with the device name and path, eg. "/dev/ttyS0"
* @param devnum device number
* @param siostat status
* @return file handle to serial dev.
*/
static sio_init: i32( char * device, devnum: i32, sio_status_t * siostat )
{
	struct termios oldtio,newtio;

	struct sigaction saio;           /* definition of signal action */

	fd: i32;
	LWIP_UNUSED_ARG(siostat);
	LWIP_UNUSED_ARG(devnum);

	/* open the device to be non-blocking (read will return immediately) */
	fd = open( device, O_RDWR | O_NOCTTY | O_NONBLOCK );
	if ( fd < 0 )
	{
		perror( device );
		exit( -1 );
	}


	/* install the signal handler before making the device asynchronous */
	switch ( devnum )
	{
		case 0:
			LWIP_DEBUGF( SIO_DEBUG, ("sioinit, signal_handler_IO_0\n") );
			saio.sa_handler = signal_handler_IO_0;
			break;
		case 1:
			LWIP_DEBUGF( SIO_DEBUG, ("sioinit, signal_handler_IO_1\n") );
			saio.sa_handler = signal_handler_IO_1;
			break;
		default:
			LWIP_DEBUGF( SIO_DEBUG,("sioinit, devnum not allowed\n") );
			break;
	}

	saio.sa_flags = 0;

	saio.sa_restorer = NULL;

	sigaction( SIGIO,&saio,NULL );

	/* allow the process to receive SIGIO */
       	if ( fcntl( fd, F_SETOWN, getpid( ) ) != 0)
	{
		perror( device );
		exit( -1 );
	}
	/* Make the file descriptor asynchronous (the manual page says only
	O_APPEND and O_NONBLOCK, will work with F_SETFL...) */
       	if ( fcntl( fd, F_SETFL, FASYNC ) != 0)
	{
		perror( device );
		exit( -1 );
	}
#else
       	if ( fcntl( fd, F_SETFL, 0 ) != 0)
	{
		perror( device );
		exit( -1 );
	}



	tcgetattr( fd,&oldtio ); /* save current port settings */
	/* set new port settings */
	/* see 'man termios' for further settings */
        memset(&newtio, 0, sizeof(newtio));
	newtio.c_cflag = BAUDRATE | CS8 | CLOCAL | CREAD | CRTSCTS;
	newtio.c_iflag = 0;
	newtio.c_oflag = 0;
	newtio.c_lflag = 0; /*ECHO; */
	newtio.c_cc[VMIN] = 1; /* Read 1 byte at a time, no timer */
	newtio.c_cc[VTIME] = 0;

	tcsetattr( fd,TCSANOW,&newtio );
	tcflush( fd, TCIOFLUSH );

	return fd;
}

/*
*
*/
pub fn sio_speed( fd: i32, speed: i32 )
{
	struct termios oldtio,newtio;
	/*  fd: i32; */

	LWIP_DEBUGF(SIO_DEBUG, ("sio_speed[%d]: baudcode:%d enter\n", fd, speed));

	if ( fd < 0 )
	{
		LWIP_DEBUGF(SIO_DEBUG, ("sio_speed[%d]: fd ERROR\n", fd));
		exit( -1 );
	}

	tcgetattr( fd,&oldtio ); /* get current port settings */

	/* set new port settings 
	* see 'man termios' for further settings */
        memset(&newtio, 0, sizeof(newtio));
	newtio.c_cflag = speed | CS8 | CLOCAL | CREAD; /* | CRTSCTS; */
	newtio.c_iflag = 0;
	newtio.c_oflag = 0;
	newtio.c_lflag = 0; /*ECHO; */
	newtio.c_cc[VMIN] = 1; /* Read 1 byte at a time, no timer */
	newtio.c_cc[VTIME] = 0;

	tcsetattr( fd,TCSANOW,&newtio );
	tcflush( fd, TCIOFLUSH );

	LWIP_DEBUGF(SIO_DEBUG, ("sio_speed[%d]: leave\n", fd));
}

/* --public-functions----------------------------------------------------------------------------- */
pub fn  sio_send( c: u8, sio_status_t * siostat )
{
    /*	sio_status_t * siostat = ((siostruct_t*)netif.state)->sio; */

	if ( write( siostat.fd, &c, 1 ) <= 0 )
	{
		LWIP_DEBUGF(SIO_DEBUG, ("sio_send[%d]: write refused\n", siostat.fd));
	}
}

pub fn  sio_send_string( u8 *str, sio_status_t * siostat )
{
    /*	sio_status_t * siostat = ((siostruct_t*)netif.state)->sio; */
	len: i32 = strlen( (const char *)str );

	if ( write( siostat.fd, str, len ) <= 0 )
	{
		LWIP_DEBUGF(SIO_DEBUG, ("sio_send_string[%d]: write refused\n", siostat.fd));
	}
	LWIP_DEBUGF(SIO_DEBUG, ("sio_send_string[%d]: sent: %s\n", siostat.fd, str));
}


pub fn  sio_flush( sio_status_t * siostat )
{
	LWIP_UNUSED_ARG(siostat);
	/* not implemented in unix as it is not needed */
 	/*sio_status_t * siostat = ((siostruct_t*)netif.state)->sio; */
}



/*sio_recv: u8( struct netif * netif )*/
sio_recv: u8( sio_status_t * siostat )
{
    /*	sio_status_t * siostat = ((siostruct_t*)netif.state)->sio; */
	return fifoGet( &(siostat.myfifo) );
}

i16 sio_poll(sio_status_t * siostat)
{
    /*	sio_status_t * siostat = ((siostruct_t*)netif.state)->sio;*/
	return fifoGetNonBlock( &(siostat.myfifo) );
}


pub fn  sio_expect_string( u8 *str, sio_status_t * siostat )
{
    /*	sio_status_t * siostat = ((siostruct_t*)netif.state)->sio;*/
	c: u8;
 	finger: i32=0;
  
	LWIP_DEBUGF(SIO_DEBUG, ("sio_expect_string[%d]: %s\n", siostat.fd, str));
	while ( 1 )
	{
		c=fifoGet( &(siostat.myfifo) );
		LWIP_DEBUGF(SIO_DEBUG, ("_%c", c));
		if ( c==str[finger] )
		{
			finger++;
		} else if ( finger > 0 )
		{
                    /*it might fit in the beginning? */
			if ( str[0] == c )
			{
				finger = 1;
			}
		}
		if ( 0 == str[finger] ) 
                    break;	/* done, we have a match */
	}
	LWIP_DEBUGF(SIO_DEBUG, ("sio_expect_string[%d]: [match]\n", siostat.fd));
}



sio_write: u32(sio_status_t * siostat, u8 *buf, size: u32)
{
    isize wsz = write( siostat.fd, buf, size );
    return wsz < 0 ? 0 : wsz;
}

sio_read: u32(sio_status_t * siostat, u8 *buf, size: u32)
{
    isize rsz = read( siostat.fd, buf, size );
    return rsz < 0 ? 0 : rsz;
}

pub fn  sio_read_abort(sio_status_t * siostat)
{
    LWIP_UNUSED_ARG(siostat);
    printf("sio_read_abort[%d]: not yet implemented for unix\n", siostat.fd);
}


sio_fd_t sio_open(devnum: u8)
{
	char dev[20];

	/* would be nice with dynamic memory alloc */
	sio_status_t * siostate = &statusar[ devnum ];
/* 	siostruct_t * tmp; */


/* 	tmp = (siostruct_t*)(netif.state); */
/* 	tmp.sio = siostate; */

/* 	tmp = (siostruct_t*)(netif.state); */

/* 	((sio_status_t*)(tmp.sio))->fd = 0; */

	LWIP_DEBUGF(SIO_DEBUG, ("sio_open: for devnum %d\n", devnum));


	fifoInit( &siostate.myfifo );


	snprintf( dev, sizeof(dev), "/dev/ttyS%d", devnum );

	if ( (devnum == 1) || (devnum == 0) )
	{
		if ( ( siostate.fd = sio_init( dev, devnum, siostate ) ) == 0 )
		{
			LWIP_DEBUGF(SIO_DEBUG, ("sio_open: ERROR opening serial device dev=%s\n", dev));
			abort( );
			return NULL;
		}
		LWIP_DEBUGF(SIO_DEBUG, ("sio_open[%d]: dev=%s open.\n", siostate.fd, dev));
	} 

	else if (devnum == 2) {
	    pid_t childpid;
	    char name[256];
	    childpid = forkpty(&siostate.fd, name, NULL, NULL);
	    if(childpid < 0) {
		perror("forkpty");
		exit (1);
	    }
	    if(childpid == 0) {
		execl("/usr/sbin/pppd", "pppd",
			"ms-dns", "198.168.100.7",
			"local", "crtscts",
			"debug",

			"auth",
			"require-chap",
			"remotename", "lwip",
#else
			"noauth",


			"+ipv6",

			"192.168.1.1:192.168.1.2",
			NULL);
		perror("execl pppd");
		exit (1);
	    } else {
		LWIP_DEBUGF(SIO_DEBUG, ("sio_open[%d]: spawned pppd pid %d on %s\n",
			siostate.fd, childpid, name));
	    }

	}


	else if (devnum == 3) {
	    pid_t childpid;
	    /* create PTY pair */
	    siostate.fd = posix_openpt(O_RDWR | O_NOCTTY);
	    if (siostate.fd < 0) {
		perror("open pty master");
		exit (1);
	    }
	    if (grantpt(siostate.fd) != 0) {
		perror("grant pty master");
		exit (1);
	    }
	    if (unlockpt(siostate.fd) != 0) {
		perror("unlock pty master");
		exit (1);
	    }
	    LWIP_DEBUGF(SIO_DEBUG, ("sio_open[%d]: for %s\n",
		    siostate.fd, ptsname(siostate.fd)));
	    /* fork for slattach */
	    childpid = fork();
	    if(childpid < 0) {
		perror("fork");
		exit (1);
	    }
	    if(childpid == 0) {
		/* esteblish SLIP interface on host side connected to PTY slave */
		execl("/sbin/slattach", "slattach",
			"-d", "-v", "-L", "-p", "slip",
			ptsname(siostate.fd),
			NULL);
		perror("execl slattach");
		exit (1);
	    } else {
		ret: i32;
		char buf[1024];
		LWIP_DEBUGF(SIO_DEBUG, ("sio_open[%d]: spawned slattach pid %d on %s\n",
			siostate.fd, childpid, ptsname(siostate.fd)));
		/* wait a moment for slattach startup */
		sleep(1);
		/* configure SLIP interface on host side as P2P interface */
		snprintf(buf, sizeof(buf),
			"/sbin/ifconfig sl0 mtu %d %s pointopoint %s up",
			SLIP_MAX_SIZE, "192.168.2.1", "192.168.2.2");
		LWIP_DEBUGF(SIO_DEBUG, ("sio_open[%d]: system(\"%s\");\n", siostate.fd, buf));
		ret = system(buf);
		if (ret < 0) {
		    perror("ifconfig failed");
		    exit(1);
		}
	    }
	}

	else
	{
		LWIP_DEBUGF(SIO_DEBUG, ("sio_open: device %s (%d) is not supported\n", dev, devnum));
		return NULL;
	}

	return siostate;
}

/*
*
*/
pub fn  sio_change_baud( sioBaudrates baud, sio_status_t * siostat )
{
    /*	sio_status_t * siostat = ((siostruct_t*)netif.state)->sio;*/

	LWIP_DEBUGF(SIO_DEBUG, ("sio_change_baud[%d]\n", siostat.fd));

	switch ( baud )
	{
		case SIO_BAUD_9600:
			sio_speed( siostat.fd, B9600 );
			break;
		case SIO_BAUD_19200:
			sio_speed( siostat.fd, B19200 );
			break;
		case SIO_BAUD_38400:
			sio_speed( siostat.fd, B38400 );
			break;
		case SIO_BAUD_57600:
			sio_speed( siostat.fd, B57600 );
			break;
		case SIO_BAUD_115200:
			sio_speed( siostat.fd, B115200 );
			break;

		default:
			LWIP_DEBUGF(SIO_DEBUG, ("sio_change_baud[%d]: Unknown baudrate, code:%d\n",
					siostat.fd, baud));
			break;
	}
}

