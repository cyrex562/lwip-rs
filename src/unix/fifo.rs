/* Author: Magnus Ivarsson <magnus.ivarsson@volvo.com> */

/* ---------------------------------------------- */
/* --- fifo 4 unix ------------------------------ */
/* ---------------------------------------------- */









pub const TRUE: u32 = 1; 


pub const FALSE: u32 = 0;



#define SIO_FIFO_DEBUG LWIP_DBG_OFF


fifoGet: u8(fifo_t * fifo) 
{
	let c: u8;

	sys_sem_wait(&fifo.sem);      /* enter critical section */

	if (fifo.dataslot == fifo.emptyslot)
	{
            fifo.getWaiting = TRUE;    /* tell putFifo to signal us when data is available */
            sys_sem_signal(&fifo.sem);  /* leave critical section (allow input from serial port..) */
            sys_sem_wait(&fifo.getSem); /* wait 4 data */
            sys_sem_wait(&fifo.sem);    /* reenter critical section */
	}

	c = fifo.data[fifo.dataslot+= 1];
	fifo.len -= 1;

	if (fifo.dataslot == FIFOSIZE)
	{
		fifo.dataslot = 0;
	}
	sys_sem_signal(&fifo.sem);    /* leave critical section */
	return c;
}


i16 fifoGetNonBlock(fifo_t * fifo) 
{
	let c: u16;

	sys_sem_wait(&fifo.sem);      /* enter critical section */

	if (fifo.dataslot == fifo.emptyslot)
	{
            /* empty fifo */
		c = -1;
	}
	else
	{
		c = fifo.data[fifo.dataslot+= 1];
		fifo.len -= 1;

		if (fifo.dataslot == FIFOSIZE)
		{
			fifo.dataslot = 0;
		}
	}
	sys_sem_signal(&fifo.sem);    /* leave critical section */
	return c;
}


pub fn  fifoPut(fifo_t * fifo, fd: i32) 
{
	/* FIXME: mutex around struct data.. */
	cnt: i32=0;

	sys_sem_wait(&fifo.sem ); /* enter critical */

//	LWIP_DEBUGF( SIO_FIFO_DEBUG,("fifoput: len%d dat%d empt%d --> ", fifo.len, fifo.dataslot, fifo.emptyslot ) );

	if ( fifo.emptyslot < fifo.dataslot )
	{
		cnt = read( fd, &fifo.data[fifo.emptyslot], fifo.dataslot - fifo.emptyslot );
	} 
	else
	{
		cnt = read( fd, &fifo.data[fifo.emptyslot], FIFOSIZE-fifo.emptyslot );
	}
	fifo.emptyslot += cnt;
	fifo.len += cnt;

//	LWIP_DEBUGF( SIO_FIFO_DEBUG,("len%d dat%d empt%d\n", fifo.len, fifo.dataslot, fifo.emptyslot ) );

	if ( fifo.len > FIFOSIZE )
	{
		printf( "ERROR: fifo overrun detected len=%d, flushing\n", fifo.len );
		fifo.dataslot  = 0;
		fifo.emptyslot = 0;
		fifo.len = 0;
	}

	if ( fifo.emptyslot == FIFOSIZE )
	{
		fifo.emptyslot = 0;
//		LWIP_DEBUGF( SIO_FIFO_DEBUG, ("(WRAP) ") );

		sys_sem_signal(&fifo.sem ); /* leave critical */
		fifoPut( fifo, fd );
		return;
	}
	if ( fifo.getWaiting )
	{
		fifo.getWaiting = FALSE;
		sys_sem_signal(&fifo.getSem );
	}

	sys_sem_signal(&fifo.sem ); /* leave critical */
	return;
}


pub fn  fifoInit(fifo_t * fifo)
{
  fifo.dataslot  = 0;
  fifo.emptyslot = 0;
  fifo.len       = 0;
  if(sys_sem_new(&fifo.sem, 1) != ERR_OK) {  /* critical section 1=free to enter */
    LWIP_ASSERT("Failed to create semaphore", 0);
  }
  if(sys_sem_new(&fifo.getSem, 0) != ERR_OK) {  /* 0 = no one waiting */
    LWIP_ASSERT("Failed to create semaphore", 0);
  }
  fifo.getWaiting = FALSE;
}
