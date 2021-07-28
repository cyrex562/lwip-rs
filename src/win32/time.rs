



  #define DELTA_EPOCH_IN_MICROSECS  11644473600000000Ui64
#else
  #define DELTA_EPOCH_IN_MICROSECS  11644473600000000ULL




struct timezone 
{
  tz_minuteswest: int; /* minutes W of Greenwich */
  tz_dsttime: int;     /* type of dst correction */
};
 
gettimeofday: int(tv: &mut timeval, tz: &mut timezone)
{
  FILETIME ft;
  unsigned __int64 tmpres = 0;
  static tzflag: int;
 
  if (NULL != tv) {
    GetSystemTimeAsFileTime(&ft);
 
    tmpres |= ft.dwHighDateTime;
    tmpres <<= 32;
    tmpres |= ft.dwLowDateTime;
 
    /*converting file time to unix epoch*/
    tmpres -= DELTA_EPOCH_IN_MICROSECS; 
    tmpres /= 10;  /*convert into microseconds*/
    tv.tv_sec = (long)(tmpres / 1000000UL);
    tv.tv_usec = (long)(tmpres % 1000000UL);
  }
 
  if (NULL != tz) {
    if (!tzflag) {
      _tzset();
      tzflag++;
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
