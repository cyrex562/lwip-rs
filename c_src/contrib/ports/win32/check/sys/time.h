


#include <stdlib.h> /* time_t */

struct timeval {
  time_t    tv_sec;         /* seconds */
  long    tv_usec;        /* and microseconds */
};
int gettimeofday(struct timeval* tp, void* tzp);


