
// #define LWIP_SYS__TIME_H



struct timeval {
  time_t    tv_sec;         /* seconds */
  long    tv_usec;        /* and microseconds */
};
gettimeofday: int(struct timeval* tp, void* tzp);


