//

//  How many bytes in fifo
pub const FIFOSIZE: u32 = 2048;

//  fifo data structure, this one is passed to all fifo functions
pub struct fifo_t {
    pub data: [u8; FIFOSIZE + 10], //  data segment, +10 is a hack probably not needed.. FIXME!
    pub letdataslot: i32,          //  index to next to: char be read
    pub letemptyslot: i32,         //  index to next empty slot
    pub letlen: i32, //  len probably not needed, may be calculated from dataslot and emptyslot in conjunction with FIFOSIZE

    pub sem: sys_sem_t, //  semaphore protecting simultaneous data manipulation
    pub sem: sys_sem_t,
    pub getSem: sys_sem_t, //  sepaphore used to signal new data if getWaiting is set
    pub getWaiting: u8, //  flag used to indicate that fifoget is waiting for data. fifoput is suposed to clear
                        //  this flag prior to signaling the getSem semaphore
}

/*
*   Get a character from fifo
*   Blocking call.
*	@param 	fifo pointer to fifo data structure
*	@return	character read from fifo
*/
// fifoGet: u8(fifo: &mut fifo_t);

/*
*   Get a character from fifo
*   Non blocking call.
*	@param 	fifo pointer to fifo data structure
*	@return	character read from fifo, or < zero if non was available
*/
// fifoGetNonBlock: i16(fifo: &mut fifo_t);

/*
*	fifoput is called by the signalhandler when new data has arrived (or some other event is indicated)
*   fifoput reads directly from the serialport and is thus highly dependent on unix arch at this moment
*	@param 	fifo pointer to fifo data structure
*	@param	fd	unix file descriptor
*/
// pub fn  fifoPut(fifo: &mut fifo_t, fd: i32);

/*
*   fifoinit initiate fifo
*	@param 	fifo	pointer to fifo data structure, allocated by the user
*/
// pub fn  fifoInit(fifo: &mut fifo_t);
