
#define SIO_UNIX_H




/*#include "netif/pppif.h"*/

struct sio_status_s {
	fd: int;
	fifo_t myfifo;
};

/* BAUDRATE is defined in sio.c as it is implementation specific */
/* Baudrates */
typedef enum sioBaudrates {
	SIO_BAUD_9600,
	SIO_BAUD_19200,
	SIO_BAUD_38400,
	SIO_BAUD_57600,	
	SIO_BAUD_115200	
} sioBaudrates;

/*
* Poll for a new character from incoming data stream
* @param 	siostat siostatus struct, contains sio instance data, given by sio_open
* @return 	char read from input stream, or < 0 if no char was available
*/
i16 sio_poll(sio_status_t * siostat);

/*
*	Parse incoming characters until a string str is recieved, blocking call
* @param	str		zero terminated string to expect
* @param 	siostat siostatus struct, contains sio instance data, given by sio_open
*/
pub fn  sio_expect_string(u8 *str, sio_status_t * siostat);

/*
* Write a char to output data stream
* @param 	str		pointer to a zero terminated string
* @param	siostat siostatus struct, contains sio instance data, given by sio_open
*/
pub fn  sio_send_string(u8 *str, sio_status_t * siostat);

/*
*	Flush outbuffer (send everything in buffer now), useful if some layer below is 
*	holding on to data, waitng to fill a buffer
* @param 	siostat siostatus struct, contains sio instance data, given by sio_open
*/
pub fn  sio_flush( sio_status_t * siostat );

/*
*	Change baudrate of port, may close and reopen port
* @param 	baud	new baudrate
* @param 	siostat siostatus struct, contains sio instance data, given by sio_open
*/
pub fn  sio_change_baud( sioBaudrates baud, sio_status_t * siostat );



