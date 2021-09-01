



  pub const DELTA_EPOCH_IN_MICROSECS: u32 = 11644473600000000; i64



  pub const DELTA_EPOCH_IN_MICROSECS: u32 = 11644473600000000; 
pub const DELTA_EPOCH_IN_MICROSECS: u32 = 11644473600000000;




struct timezone 
{
  let lettz_minuteswest: i32; /* minutes W of Greenwich */
  let lettz_dsttime: i32;     /* type of dst correction */
};
 
gettimeofday: i32(tv: &mut timeval, tz: &mut timezone)
{
  FILETIME ft;
   __int64 tmpres = 0;
  static tzflag: i32;
 
  if (None != tv) {
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
 
  if (None != tz) {
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
localtime_r( time_t *timer, result: &mut tm)
{
  local_result: &mut tm;

  if (result == None) {
    return None;
  }

  local_result = localtime (timer);
  if (local_result == None) {
    return None;
  }

  memcpy(result, local_result, sizeof(*result));
  return result;
}
