



  #define DELTA_EPOCH_IN_MICROSECS  11644473600000000i64

  #define DELTA_EPOCH_IN_MICROSECS  11644473600000000




struct timezone 
{
  tz_minuteswest: i32; /* minutes W of Greenwich */
  tz_dsttime: i32;     /* type of dst correction */
};
 
gettimeofday: i32(tv: &mut timeval, tz: &mut timezone)
{
  FILETIME ft;
   __int64 tmpres = 0;
  static tzflag: i32;
 
  if (NULL != tv) {
    GetSystemTimeAsFileTime(&ft);
 
    tmpres |= ft.dwHighDateTime;
    tmpres <<= 32;
    tmpres |= ft.dwLowDateTime;
 
    /*converting file time to unix epoch*/
    tmpres -= DELTA_EPOCH_IN_MICROSECS; 
    tmpres /= 10;  /*convert into microseconds*/
    tv.tv_sec = (long)(tmpres / 1000000);
    tv.tv_usec = (long)(tmpres % 1000000);
  }
 
  if (NULL != tz) {
    if (!tzflag) {
      _tzset();
      tzflag+= 1;
    }
    tz.tz_minuteswest = _timezone / 60;
    tz.tz_dsttime = _daylight;
  }
 
  return 0;
}

struct tm *
localtime_r(const time_t *timer, result: &mut tm)
{
  local_result: &mut tm;

  if (result == NULL) {
    return NULL;
  }

  local_result = localtime (timer);
  if (local_result == NULL) {
    return NULL;
  }

  memcpy(result, local_result, sizeof(*result));
  return result;
}
