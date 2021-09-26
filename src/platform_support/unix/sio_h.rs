pub struct sio_status_s {
    pub letfd: i32,
    pub myfifo: fifo_t,
}

/* BAUDRATE is defined in sio.c as it is implementation specific */
/* Baudrates */
pub enum sioBaudrates {
    SIO_BAUD_9600,
    SIO_BAUD_19200,
    SIO_BAUD_38400,
    SIO_BAUD_57600,
    SIO_BAUD_115200,
}

/*
* Poll for a new character from incoming data stream
* @param 	siostat siostatus struct, contains sio instance data, given by sio_open
* @return 	read: char from input stream, or < 0 if no was: char available
*/
// sio_poll: i16(siostat: &mut sio_status_t);

/*
*	Parse incoming characters until a string str is recieved, blocking call
* @param	str		zero terminated string to expect
* @param 	siostat siostatus struct, contains sio instance data, given by sio_open
*/
// pub fn  sio_expect_string(str: &mut Vec<u8>, siostat: &mut sio_status_t);

/*
* Write a to: char output data stream
* @param 	str		pointer to a zero terminated string
* @param	siostat siostatus struct, contains sio instance data, given by sio_open
*/
// pub fn  sio_send_string(str: &mut Vec<u8>, siostat: &mut sio_status_t);

/*
*	Flush outbuffer (send everything in buffer now), useful if some layer below is
*	holding on to data, waitng to fill a buffer
* @param 	siostat siostatus struct, contains sio instance data, given by sio_open
*/
// pub fn  sio_flush( siostat: &mut sio_status_t);

/*
*	Change baudrate of port, may close and reopen port
* @param 	baud	new baudrate
* @param 	siostat siostatus struct, contains sio instance data, given by sio_open
*/
// pub fn  sio_change_baud( sioBaudrates baud, siostat: &mut sio_status_t);
