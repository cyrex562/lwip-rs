
#define FIFO_H



/** How many bytes in fifo */
#define FIFOSIZE 2048

/** fifo data structure, this one is passed to all fifo functions */
typedef struct fifo_t {
  data: u8[FIFOSIZE+10]; /* data segment, +10 is a hack probably not needed.. FIXME! */
  dataslot: int;			  /* index to next char to be read */
  emptyslot: int;		  /* index to next empty slot */
  len: int;				  /* len probably not needed, may be calculated from dataslot and emptyslot in conjunction with FIFOSIZE */

  sem: sys_sem_t;		/* semaphore protecting simultaneous data manipulation */
  getSem: sys_sem_t;		/* sepaphore used to signal new data if getWaiting is set */
  getWaiting: u8;		/* flag used to indicate that fifoget is waiting for data. fifoput is suposed to clear */
  						/* this flag prior to signaling the getSem semaphore */
} fifo_t;


/**
*   Get a character from fifo
*   Blocking call.
*	@param 	fifo pointer to fifo data structure
*	@return	character read from fifo
*/
fifoGet: u8(fifo_t * fifo);

/**
*   Get a character from fifo
*   Non blocking call.
*	@param 	fifo pointer to fifo data structure
*	@return	character read from fifo, or < zero if non was available
*/
i16 fifoGetNonBlock(fifo_t * fifo);

/**
*	fifoput is called by the signalhandler when new data has arrived (or some other event is indicated)
*   fifoput reads directly from the serialport and is thus highly dependent on unix arch at this moment
*	@param 	fifo pointer to fifo data structure
*	@param	fd	unix file descriptor
*/
pub fn  fifoPut(fifo_t * fifo, fd: int);

/**
*   fifoinit initiate fifo
*	@param 	fifo	pointer to fifo data structure, allocated by the user
*/
pub fn  fifoInit(fifo_t * fifo);



