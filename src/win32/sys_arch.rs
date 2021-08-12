/*
 * Copyright (c) 2001-2003 Swedish Institute of Computer Science.
 * All rights reserved. 
 * 
 * Redistribution and use in source and binary forms, with or without modification, 
 * are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice,
 *    this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 * 3. The name of the author may not be used to endorse or promote products
 *    derived from this software without specific prior written permission. 
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR IMPLIED 
 * WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF 
 * MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT 
 * SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, 
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT 
 * OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS 
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN 
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING 
 * IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY 
 * OF SUCH DAMAGE.
 *
 * This file is part of the lwIP TCP/IP stack.
 * 
 * Author: Adam Dunkels <adam@sics.se>
 *         Simon Goldschmidt
 *
 */





#pragma warning (push, 3)



#pragma warning (pop)










/* Set this to 1 to enable assertion checks that SYS_ARCH_PROTECT() is only
 * called once in a call stack (calling it nested might cause trouble in some
 * implementations, so let's avoid this in core code as long as we can).
 */

// #define LWIP_SYS_ARCH_CHECK_NESTED_PROTECT 1


/* Set this to 1 to enable assertion checks that SYS_ARCH_PROTECT() is *not*
 * called before functions potentiolly involving the OS scheduler.
 *
 * This scheme is currently broken only for non-core-locking when waking up
 * threads waiting on a socket via select/poll.
 */

// #define LWIP_SYS_ARCH_CHECK_SCHEDULING_UNPROTECTED LWIP_TCPIP_CORE_LOCKING


// #define LWIP_WIN32_SYS_ARCH_ENABLE_PROTECT_COUNTER (LWIP_SYS_ARCH_CHECK_NESTED_PROTECT || LWIP_SYS_ARCH_CHECK_SCHEDULING_UNPROTECTED)

/* These functions are used from NO_SYS also, for precise timer triggering */
static LARGE_INTEGER freq, sys_start_time;
#define SYS_INITIALIZED() (freq.QuadPart != 0)

static DWORD netconn_sem_tls_index;

static HCRYPTPROV hcrypt;

u32
sys_win_rand()
{
  ret: u32;
  if (CryptGenRandom(hcrypt, sizeof(ret), (BYTE*)&ret)) {
    return ret;
  }
  LWIP_ASSERT("CryptGenRandom failed", 0);
  return 0;
}

pub fn
sys_win_rand_init()
{
  if (!CryptAcquireContext(&hcrypt, NULL, NULL, PROV_RSA_FULL, 0)) {
    DWORD err = GetLastError();
    LWIP_PLATFORM_DIAG(("CryptAcquireContext failed with error %d, trying to create NEWKEYSET", err));
    if(!CryptAcquireContext(&hcrypt, NULL, NULL, PROV_RSA_FULL, CRYPT_NEWKEYSET)) {
      char errbuf[128];
      err = GetLastError();
      snprintf(errbuf, sizeof(errbuf), "CryptAcquireContext failed with error %d", err);
      
      LWIP_ASSERT(errbuf, 0);
    }
  }
}

pub fn
sys_init_timing()
{
  QueryPerformanceFrequency(&freq);
  QueryPerformanceCounter(&sys_start_time);
}

static LONGLONG
sys_get_ms_longlong()
{
  LONGLONG ret;
  LARGE_INTEGER now;

  if (!SYS_INITIALIZED()) {
    sys_init();
    LWIP_ASSERT("initialization failed", SYS_INITIALIZED());
  }

  QueryPerformanceCounter(&now);
  ret = now.QuadPart-sys_start_time.QuadPart;
  return (u32)(((ret)*1000)/freq.QuadPart);
}

u32
sys_jiffies()
{
  return (u32)sys_get_ms_longlong();
}

u32
sys_now()
{
  return (u32)sys_get_ms_longlong();
}

CRITICAL_SECTION critSec;

static protection_depth: i32;


pub fn
InitSysArchProtect()
{
  InitializeCriticalSection(&critSec);
}

sys_prot_t
sys_arch_protect()
{

  if (!SYS_INITIALIZED()) {
    sys_init();
    LWIP_ASSERT("initialization failed", SYS_INITIALIZED());
  }

  EnterCriticalSection(&critSec);

  LWIP_ASSERT("nested SYS_ARCH_PROTECT", protection_depth == 0);


  protection_depth+= 1;

  return 0;
}

pub fn 
sys_arch_unprotect(sys_prot_t pval)
{
  

  LWIP_ASSERT("missing SYS_ARCH_PROTECT", protection_depth == 1);

  LWIP_ASSERT("missing SYS_ARCH_PROTECT", protection_depth > 0);


  protection_depth--;

  LeaveCriticalSection(&critSec);
}


/* This checks that SYS_ARCH_PROTECT() hasn't been called by protecting
 * and then checking the level
 */
pub fn
sys_arch_check_not_protected()
{
  sys_arch_protect();
  LWIP_ASSERT("SYS_ARCH_PROTECT before scheduling", protection_depth == 1);
  sys_arch_unprotect(0);
}

#define sys_arch_check_not_protected()


pub fn
msvc_sys_init()
{
  sys_win_rand_init();
  sys_init_timing();
  InitSysArchProtect();
  netconn_sem_tls_index = TlsAlloc();
  LWIP_ASSERT("TlsAlloc failed", netconn_sem_tls_index != TLS_OUT_OF_INDEXES);
}

pub fn 
sys_init()
{
  msvc_sys_init();
}



struct threadlist {
  lwip_thread_fn function;
  arg: &mut Vec<u8>;
  DWORD id;
  next: &mut threadlist;
};

static lwip_win32_threads: &mut threadlist = NULL;

pub fn 
sys_sem_new(sys_sem_t *sem, count: u8)
{
  HANDLE new_sem = NULL;

  LWIP_ASSERT("sem != NULL", sem != NULL);

  new_sem = CreateSemaphore(0, count, 100000, 0);
  LWIP_ASSERT("Error creating semaphore", new_sem != NULL);
  if(new_sem != NULL) {
    if (SYS_INITIALIZED()) {
      SYS_ARCH_LOCKED(SYS_STATS_INC_USED(sem));
    } else {
      SYS_STATS_INC_USED(sem);
    }

    LWIP_ASSERT("sys_sem_new() counter overflow", lwip_stats.sys.sem.used != 0);

    sem.sem = new_sem;
    return ERR_OK;
  }
   
  /* failed to allocate memory... */
  if (SYS_INITIALIZED()) {
    SYS_ARCH_LOCKED(SYS_STATS_INC(sem.err));
  } else {
    SYS_STATS_INC(sem.err);
  }
  sem.sem = NULL;
  return ERR_MEM;
}

pub fn 
sys_sem_free(sys_sem_t *sem)
{
  /* parameter check */
  LWIP_ASSERT("sem != NULL", sem != NULL);
  LWIP_ASSERT("sem.sem != NULL", sem.sem != NULL);
  LWIP_ASSERT("sem.sem != INVALID_HANDLE_VALUE", sem.sem != INVALID_HANDLE_VALUE);
  CloseHandle(sem.sem);

  SYS_ARCH_LOCKED(SYS_STATS_DEC(sem.used));

  LWIP_ASSERT("sys_sem_free() closed more than created", lwip_stats.sys.sem.used != -1);

  sem.sem = NULL;
}

u32
sys_arch_sem_wait(sys_sem_t *sem, timeout: u32)
{
  DWORD ret;
  LONGLONG starttime, endtime;
  LWIP_ASSERT("sem != NULL", sem != NULL);
  LWIP_ASSERT("sem.sem != NULL", sem.sem != NULL);
  LWIP_ASSERT("sem.sem != INVALID_HANDLE_VALUE", sem.sem != INVALID_HANDLE_VALUE);
  if (!timeout) {
    /* wait infinite */
    starttime = sys_get_ms_longlong();
    ret = WaitForSingleObject(sem.sem, INFINITE);
    LWIP_ASSERT("Error waiting for semaphore", ret == WAIT_OBJECT_0);
    endtime = sys_get_ms_longlong();
    /* return the time we waited for the sem */
    return (u32)(endtime - starttime);
  } else {
    starttime = sys_get_ms_longlong();
    ret = WaitForSingleObject(sem.sem, timeout);
    LWIP_ASSERT("Error waiting for semaphore", (ret == WAIT_OBJECT_0) || (ret == WAIT_TIMEOUT));
    if (ret == WAIT_OBJECT_0) {
      endtime = sys_get_ms_longlong();
      /* return the time we waited for the sem */
      return (u32)(endtime - starttime);
    } else {
      /* timeout */
      return SYS_ARCH_TIMEOUT;
    }
  }
}

pub fn 
sys_sem_signal(sys_sem_t *sem)
{
  BOOL ret;
  sys_arch_check_not_protected();
  LWIP_ASSERT("sem != NULL", sem != NULL);
  LWIP_ASSERT("sem.sem != NULL", sem.sem != NULL);
  LWIP_ASSERT("sem.sem != INVALID_HANDLE_VALUE", sem.sem != INVALID_HANDLE_VALUE);
  ret = ReleaseSemaphore(sem.sem, 1, NULL);
  LWIP_ASSERT("Error releasing semaphore", ret != 0);
  
}

pub fn 
sys_mutex_new(sys_mutex_t *mutex)
{
  HANDLE new_mut = NULL;

  LWIP_ASSERT("mutex != NULL", mutex != NULL);

  new_mut = CreateMutex(NULL, FALSE, NULL);
  LWIP_ASSERT("Error creating mutex", new_mut != NULL);
  if (new_mut != NULL) {
    SYS_ARCH_LOCKED(SYS_STATS_INC_USED(mutex));

    LWIP_ASSERT("sys_mutex_new() counter overflow", lwip_stats.sys.mutex.used != 0);

    mutex.mut = new_mut;
    return ERR_OK;
  }
   
  /* failed to allocate memory... */
  SYS_ARCH_LOCKED(SYS_STATS_INC(mutex.err));
  mutex.mut = NULL;
  return ERR_MEM;
}

pub fn 
sys_mutex_free(sys_mutex_t *mutex)
{
  /* parameter check */
  LWIP_ASSERT("mutex != NULL", mutex != NULL);
  LWIP_ASSERT("mutex.mut != NULL", mutex.mut != NULL);
  LWIP_ASSERT("mutex.mut != INVALID_HANDLE_VALUE", mutex.mut != INVALID_HANDLE_VALUE);
  CloseHandle(mutex.mut);

  SYS_ARCH_LOCKED(SYS_STATS_DEC(mutex.used));

  LWIP_ASSERT("sys_mutex_free() closed more than created", lwip_stats.sys.mutex.used != -1);

  mutex.mut = NULL;
}

pub fn  sys_mutex_lock(sys_mutex_t *mutex)
{
  DWORD ret;
  LWIP_ASSERT("mutex != NULL", mutex != NULL);
  LWIP_ASSERT("mutex.mut != NULL", mutex.mut != NULL);
  LWIP_ASSERT("mutex.mut != INVALID_HANDLE_VALUE", mutex.mut != INVALID_HANDLE_VALUE);
  /* wait infinite */
  ret = WaitForSingleObject(mutex.mut, INFINITE);
  LWIP_ASSERT("Error waiting for mutex", ret == WAIT_OBJECT_0);
  
}

pub fn 
sys_mutex_unlock(sys_mutex_t *mutex)
{
  sys_arch_check_not_protected();
  LWIP_ASSERT("mutex != NULL", mutex != NULL);
  LWIP_ASSERT("mutex.mut != NULL", mutex.mut != NULL);
  LWIP_ASSERT("mutex.mut != INVALID_HANDLE_VALUE", mutex.mut != INVALID_HANDLE_VALUE);
  /* wait infinite */
  if (!ReleaseMutex(mutex.mut)) {
    LWIP_ASSERT("Error releasing mutex", 0);
  }
}



const DWORD MS_VC_EXCEPTION=0x406D1388;
#pragma pack(push,8)
typedef struct tagTHREADNAME_INFO
{
  DWORD dwType; /* Must be 0x1000. */
  LPCSTR szName; /* Pointer to name (in user addr space). */
  DWORD dwThreadID; /* Thread ID (-1=caller thread). */
  DWORD dwFlags; /* Reserved for future use, must be zero. */
} THREADNAME_INFO;
#pragma pack(pop)

pub fn
SetThreadName(DWORD dwThreadID,  char* threadName)
{
  THREADNAME_INFO info;
  info.dwType = 0x1000;
  info.szName = threadName;
  info.dwThreadID = dwThreadID;
  info.dwFlags = 0;

  __try {
    RaiseException(MS_VC_EXCEPTION, 0, sizeof(info)/sizeof(ULONG_PTR), (ULONG_PTR*)&info);
  }
  __except(EXCEPTION_EXECUTE_HANDLER) {
  }
}
 /* _MSC_VER */
pub fn
SetThreadName(DWORD dwThreadID,  char* threadName)
{
  
  
}


pub fn
sys_thread_function(arg: &mut ())
{
  struct threadlist* t = (struct threadlist*)arg;

  sys_arch_netconn_sem_alloc();

  t.function(t.arg);

  sys_arch_netconn_sem_free();

}

sys_thread_t
sys_thread_new(name: &String, lwip_thread_fn function, arg: &mut Vec<u8>, stacksize: i32, prio: i32)
{
  new_thread: &mut threadlist;
  HANDLE h;
  SYS_ARCH_DECL_PROTECT(lev);

  
  
  

  new_thread = (struct threadlist*)malloc(sizeof(struct threadlist));
  LWIP_ASSERT("new_thread != NULL", new_thread != NULL);
  if (new_thread != NULL) {
    new_thread.function = function;
    new_thread.arg = arg;
    SYS_ARCH_PROTECT(lev);
    new_thread.next = lwip_win32_threads;
    lwip_win32_threads = new_thread;

    h = CreateThread(0, 0, (LPTHREAD_START_ROUTINE)sys_thread_function, new_thread, 0, &(new_thread.id));
    LWIP_ASSERT("h != 0", h != 0);
    LWIP_ASSERT("h != -1", h != INVALID_HANDLE_VALUE);
    
    SetThreadName(new_thread.id, name);

    SYS_ARCH_UNPROTECT(lev);
    return new_thread.id;
  }
  return 0;
}




static DWORD lwip_core_lock_holder_thread_id;

pub fn 
sys_lock_tcpip_core()
{
  sys_mutex_lock(&lock_tcpip_core);
  lwip_core_lock_holder_thread_id = GetCurrentThreadId();
}

pub fn 
sys_unlock_tcpip_core()
{
  lwip_core_lock_holder_thread_id = 0;
  sys_mutex_unlock(&lock_tcpip_core);
}


static DWORD lwip_tcpip_thread_id;

pub fn 
sys_mark_tcpip_thread()
{
  lwip_tcpip_thread_id = GetCurrentThreadId();
}

pub fn 
sys_check_core_locking()
{
  /* Embedded systems should check we are NOT in an interrupt context here */

  if (lwip_tcpip_thread_id != 0) {
    DWORD current_thread_id = GetCurrentThreadId();


    LWIP_ASSERT("Function called without core lock", current_thread_id == lwip_core_lock_holder_thread_id);
 /* LWIP_TCPIP_CORE_LOCKING */
    LWIP_ASSERT("Function called from wrong thread", current_thread_id == lwip_tcpip_thread_id);

     /* for LWIP_NOASSERT */
  }
}


pub fn 
sys_mbox_new(sys_mbox_t *mbox, size: i32)
{
  LWIP_ASSERT("mbox != NULL", mbox != NULL);
  

  mbox.sem = CreateSemaphore(0, 0, MAX_QUEUE_ENTRIES, 0);
  LWIP_ASSERT("Error creating semaphore", mbox.sem != NULL);
  if (mbox.sem == NULL) {
    SYS_ARCH_LOCKED(SYS_STATS_INC(mbox.err));
    return ERR_MEM;
  }
  memset(&mbox.q_mem, 0, sizeof(u32)*MAX_QUEUE_ENTRIES);
  mbox.head = 0;
  mbox.tail = 0;
  SYS_ARCH_LOCKED(SYS_STATS_INC_USED(mbox));

  LWIP_ASSERT("sys_mbox_new() counter overflow", lwip_stats.sys.mbox.used != 0);

  return ERR_OK;
}

pub fn 
sys_mbox_free(sys_mbox_t *mbox)
{
  /* parameter check */
  LWIP_ASSERT("mbox != NULL", mbox != NULL);
  LWIP_ASSERT("mbox.sem != NULL", mbox.sem != NULL);
  LWIP_ASSERT("mbox.sem != INVALID_HANDLE_VALUE", mbox.sem != INVALID_HANDLE_VALUE);

  CloseHandle(mbox.sem);

  SYS_STATS_DEC(mbox.used);

  LWIP_ASSERT( "sys_mbox_free() ", lwip_stats.sys.mbox.used != -1);

  mbox.sem = NULL;
}

pub fn 
sys_mbox_post(sys_mbox_t *q, msg: &mut ())
{
  BOOL ret;
  SYS_ARCH_DECL_PROTECT(lev);
  sys_arch_check_not_protected();

  /* parameter check */
  LWIP_ASSERT("q != SYS_MBOX_NULL", q != SYS_MBOX_NULL);
  LWIP_ASSERT("q.sem != NULL", q.sem != NULL);
  LWIP_ASSERT("q.sem != INVALID_HANDLE_VALUE", q.sem != INVALID_HANDLE_VALUE);

  SYS_ARCH_PROTECT(lev);
  q.q_mem[q.head] = msg;
  q.head+= 1;
  if (q.head >= MAX_QUEUE_ENTRIES) {
    q.head = 0;
  }
  LWIP_ASSERT("mbox is full!", q.head != q.tail);
  ret = ReleaseSemaphore(q.sem, 1, 0);
  LWIP_ASSERT("Error releasing sem", ret != 0);
  

  SYS_ARCH_UNPROTECT(lev);
}

pub fn 
sys_mbox_trypost(sys_mbox_t *q, msg: &mut ())
{
  new_head: u32;
  BOOL ret;
  SYS_ARCH_DECL_PROTECT(lev);
  sys_arch_check_not_protected();

  /* parameter check */
  LWIP_ASSERT("q != SYS_MBOX_NULL", q != SYS_MBOX_NULL);
  LWIP_ASSERT("q.sem != NULL", q.sem != NULL);
  LWIP_ASSERT("q.sem != INVALID_HANDLE_VALUE", q.sem != INVALID_HANDLE_VALUE);

  SYS_ARCH_PROTECT(lev);

  new_head = q.head + 1;
  if (new_head >= MAX_QUEUE_ENTRIES) {
    new_head = 0;
  }
  if (new_head == q.tail) {
    SYS_ARCH_UNPROTECT(lev);
    return ERR_MEM;
  }

  q.q_mem[q.head] = msg;
  q.head = new_head;
  LWIP_ASSERT("mbox is full!", q.head != q.tail);
  ret = ReleaseSemaphore(q.sem, 1, 0);
  LWIP_ASSERT("Error releasing sem", ret != 0);
  

  SYS_ARCH_UNPROTECT(lev);
  return ERR_OK;
}

pub fn 
sys_mbox_trypost_fromisr(sys_mbox_t *q, msg: &mut ())
{
  return sys_mbox_trypost(q, msg);
}

u32
sys_arch_mbox_fetch(sys_mbox_t *q, void **msg, timeout: u32)
{
  DWORD ret;
  LONGLONG starttime, endtime;
  SYS_ARCH_DECL_PROTECT(lev);

  /* parameter check */
  LWIP_ASSERT("q != SYS_MBOX_NULL", q != SYS_MBOX_NULL);
  LWIP_ASSERT("q.sem != NULL", q.sem != NULL);
  LWIP_ASSERT("q.sem != INVALID_HANDLE_VALUE", q.sem != INVALID_HANDLE_VALUE);

  if (timeout == 0) {
    timeout = INFINITE;
  }
  starttime = sys_get_ms_longlong();
  ret = WaitForSingleObject(q.sem, timeout);
  if (ret == WAIT_OBJECT_0) {
    SYS_ARCH_PROTECT(lev);
    if (msg != NULL) {
      *msg  = q.q_mem[q.tail];
    }

    q.tail+= 1;
    if (q.tail >= MAX_QUEUE_ENTRIES) {
      q.tail = 0;
    }
    SYS_ARCH_UNPROTECT(lev);
    endtime = sys_get_ms_longlong();
    return (u32)(endtime - starttime);
  } else {
    LWIP_ASSERT("Error waiting for sem", ret == WAIT_TIMEOUT);
    if (msg != NULL) {
      *msg  = NULL;
    }

    return SYS_ARCH_TIMEOUT;
  }
}

u32
sys_arch_mbox_tryfetch(sys_mbox_t *q, void **msg)
{
  DWORD ret;
  SYS_ARCH_DECL_PROTECT(lev);

  /* parameter check */
  LWIP_ASSERT("q != SYS_MBOX_NULL", q != SYS_MBOX_NULL);
  LWIP_ASSERT("q.sem != NULL", q.sem != NULL);
  LWIP_ASSERT("q.sem != INVALID_HANDLE_VALUE", q.sem != INVALID_HANDLE_VALUE);

  ret = WaitForSingleObject(q.sem, 0);
  if (ret == WAIT_OBJECT_0) {
    SYS_ARCH_PROTECT(lev);
    if (msg != NULL) {
      *msg  = q.q_mem[q.tail];
    }

    q.tail+= 1;
    if (q.tail >= MAX_QUEUE_ENTRIES) {
      q.tail = 0;
    }
    SYS_ARCH_UNPROTECT(lev);
    return 0;
  } else {
    LWIP_ASSERT("Error waiting for sem", ret == WAIT_TIMEOUT);
    if (msg != NULL) {
      *msg  = NULL;
    }

    return SYS_ARCH_TIMEOUT;
  }
}


sys_sem_t*
sys_arch_netconn_sem_get()
{
  LPVOID tls_data = TlsGetValue(netconn_sem_tls_index);
  return (sys_sem_t*)tls_data;
}

pub fn 
sys_arch_netconn_sem_alloc()
{
  sys_sem_t *sem;
  let err: err_t;
  BOOL done;

  sem = (sys_sem_t*)malloc(sizeof(sys_sem_t));
  LWIP_ASSERT("failed to allocate memory for TLS semaphore", sem != NULL);
  err = sys_sem_new(sem, 0);
  LWIP_ASSERT("failed to initialise TLS semaphore", err == ERR_OK);
  done = TlsSetValue(netconn_sem_tls_index, sem);
  
  LWIP_ASSERT("failed to initialise TLS semaphore storage", done == TRUE);
}

pub fn 
sys_arch_netconn_sem_free()
{
  LPVOID tls_data = TlsGetValue(netconn_sem_tls_index);
  if (tls_data != NULL) {
    BOOL done;
    free(tls_data);
    done = TlsSetValue(netconn_sem_tls_index, NULL);
    
    LWIP_ASSERT("failed to de-init TLS semaphore storage", done == TRUE);
  }
}




/* get keyboard state to terminate the debug app on any kbhit event using win32 API */
pub fn lwip_win32_keypressed()
{
  INPUT_RECORD rec;
  DWORD num = 0;
  HANDLE h = GetStdHandle(STD_INPUT_HANDLE);
  BOOL ret = PeekConsoleInput(h, &rec, 1, &num);
  if (ret && num) {
    ReadConsoleInput(h, &rec, 1, &num);
    if (rec.EventType == KEY_EVENT) {
      if (rec.Event.KeyEvent.bKeyDown) {
        /* not a special key? */
        if (rec.Event.KeyEvent.uChar.AsciiChar != 0) {
          return 1;
        }
      }
    }
  }
  return 0;
}



/* This is an example implementation for LWIP_PLATFORM_DIAG:
 * format a string and pass it to your output function.
 */
pub fn 
lwip_win32_platform_diag(format: &String, ...)
{
  va_list ap;
  /* get the varargs */
  va_start(ap, format);
  /* prvia: i32 varargs; to use another output function, you could use
     vsnprintf here */
  vprintf(format, ap);
  va_end(ap);
}
