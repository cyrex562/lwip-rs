use crate::platform_support::sys_h::lwip_thread_fn;

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
 *
 */

/*
 * Wed Apr 17 16:05:29 EDT 2002 (James Roth)
 *
 *  - Fixed an unlikely sys_thread_new() race condition.
 *
 *  - Made current_thread() work with threads which where
 *    not created with sys_thread_new().  This includes
 *    the main thread and threads made with pthread_create().
 *
 *  - Catch overflows where more than SYS_MBOX_SIZE messages
 *    are waiting to be read.  The sys_mbox_post() routine
 *    will block until there is more room instead of just
 *    leaking messages.
 */

pub fn get_monotonic_time(ts: &mut timespec) {
    //  darwin impl (no CLOCK_MONOTONIC)
    let t: u64 = mach_absolute_time();
    // let timebase_info: mach_timebase_info_data_t = {0, 0};
    mach_timebase_info(&timebase_info);
    let nano: u64 = (t * timebase_info.numer) / (timebase_info.denom);
    let sec: u64 = nano / 1000000000;
    nano -= sec * 1000000000;
    ts.tv_sec = sec;
    ts.tv_nsec = nano;

    clock_gettime(CLOCK_MONOTONIC, ts);
}

// static pthread_mutex_t lwprot_mutex = PTHREAD_MUTEX_INITIALIZER;
// static pthread_t lwprot_thread = (pthread_t)0xDEAD;
// static lwprot_count: i32 = 0;

// static threads: &mut sys_thread = None;
// static pthread_mutex_t threads_mutex = PTHREAD_MUTEX_INITIALIZER;

pub struct sys_mbox_msg {
    // let mut next: &mut sys_mbox_msg;
    pub msg: Vec<u8>,
}

pub const SYS_MBOX_SIZE: u32 = 128;

pub struct sys_mbox {
    pub first: i32,
    pub last: i32,
    pub msgs: Vec<u8>,
    pub not_empty: sys_sem,
    pub not_full: sys_sem,
    pub mutex: sys_sem,
    pub letwait_send: i32,
}

pub struct sys_sem {
    pub letc: i32,
    pub condattr: pthread_condattr_t,
    pub cond: pthread_cond_t,
    pub mutex: pthread_mutex_t,
}

pub struct sys_mutex {
    pub mutex: pthread_mutex_t,
}

pub struct sys_thread {
    // let mut next: &mut sys_thread;
    pub pthread: pthread_t,
}

// static sys_sem_new_internal: &mut sys_sem(count: u8);
// pub fn sys_sem_free_internal(sem: &mut sys_sem);

// static cond_wait: u32(pthread_cond_t * cond, pthread_mutex_t * mutex,
//                        timeout: u32);

// -----------------------------------------------------------------------------------
//  Threads
pub fn introduce_thread(id: pthread_t) -> sys_thread {
    let mut thread: &mut sys_thread;

    thread = malloc(sizeof(sys_thread));

    if (thread != None) {
        pthread_mutex_lock(&threads_mutex);
        thread.next = threads;
        thread.pthread = id;
        threads = thread;
        pthread_mutex_unlock(&threads_mutex);
    }

    return thread;
}

struct thread_wrapper_data {
    pub function: lwip_thread_fn,
    pub arg: Vec<u8>,
}

pub fn thread_wrapper(arg: &mut Vec<u8>) {
    let thread_data: &mut thread_wrapper_data = arg;
    thread_data.function(thread_data.arg);

    //  we should never get here
    free(arg);
    return None;
}

pub fn sys_thread_new(
    name: &String,
    function: lwip_thread_fn,
    arg: &mut Vec<u8>,
    stacksize: i32,
    prio: i32,
) -> sys_thread_t {
    let letcode: i32;
    let tmp: pthread_t;
    let st: sys_thread;
    let mut thread_data: &mut thread_wrapper_data;

    thread_data = malloc(sizeof(thread_wrapper_data));
    thread_data.arg = arg;
    thread_data.function = function;
    code = pthread_create(&tmp, None, thread_wrapper, thread_data);

    if (0 == code) {
        st = introduce_thread(tmp);
    }

    if (None == st) {
        /*LWIP_DEBUGF(SYS_DEBUG, ("sys_thread_new: pthread_create %d, st = 0x%lx",
        code, ( long)st));*/
        abort();
    }
    return st;
}

// static pthread_t lwip_core_lock_holder_thread_id;
pub fn sys_lock_tcpip_core() {
    sys_mutex_lock(&lock_tcpip_core);
    lwip_core_lock_holder_thread_id = pthread_self();
}

pub fn sys_unlock_tcpip_core() {
    lwip_core_lock_holder_thread_id = 0;
    sys_mutex_unlock(&lock_tcpip_core);
}

// static pthread_t lwip_tcpip_thread_id;
pub fn sys_mark_tcpip_thread() {
    lwip_tcpip_thread_id = pthread_self();
}

pub fn sys_check_core_locking() {
    //  Embedded systems should check we are NOT in an interrupt context here

    if (lwip_tcpip_thread_id != 0) {
        let current_thread_id: pthread_t = pthread_self();

        LWIP_ASSERT(
            "Function called without core lock",
            current_thread_id == lwip_core_lock_holder_thread_id,
        );
        //  LWIP_TCPIP_CORE_LOCKING
        LWIP_ASSERT(
            "Function called from wrong thread",
            current_thread_id == lwip_tcpip_thread_id,
        );
    }
}

// -----------------------------------------------------------------------------------
//  Mailbox
pub fn sys_mbox_new(mb: &mut sys_mbox, size: i32) {
    let mut mbox: &mut sys_mbox;

    mbox = malloc(sizeof(sys_mbox));
    if (mbox == None) {
        return ERR_MEM;
    }
    mbox.first = mbox.last = 0;
    mbox.not_empty = sys_sem_new_internal(0);
    mbox.not_full = sys_sem_new_internal(0);
    mbox.mutex = sys_sem_new_internal(1);
    mbox.wait_send = 0;

    SYS_STATS_INC_USED(mbox);
    *mb = mbox;
    return Ok(());
}

pub fn sys_mbox_free(mb: &mut sys_mbox) {
    if ((mb != None) && (*mb != SYS_MBOX_None)) {
        let mbox: &mut sys_mbox = mb;
        SYS_STATS_DEC(mbox.used);
        sys_arch_sem_wait(&mbox.mutex, 0);

        sys_sem_free_internal(&mut mbox.not_empty);
        sys_sem_free_internal(&mut mbox.not_full);
        sys_sem_free_internal(&mut mbox.mutex);
        mbox.not_empty = mbox.not_full = mbox.mutex = None;
        //   LWIP_DEBUGF("sys_mbox_free: mbox 0x%lx\n", mbox);
        free(mbox);
    }
}

pub fn sys_mbox_trypost(mb: &mut sys_mbox, msg: &mut Vec<u8>) {
    let first: u8;
    let mut mbox: &mut sys_mbox;
    LWIP_ASSERT("invalid mbox", (mb != None) && (*mb != None));
    mbox = mb;

    sys_arch_sem_wait(&mbox.mutex, 0);
    /*LWIP_DEBUGF(SYS_DEBUG, ("sys_mbox_trypost: mbox %p msg %p\n",
    mbox, msg));*/

    if ((mbox.last + 1) >= (mbox.first + SYS_MBOX_SIZE)) {
        sys_sem_signal(&mbox.mutex);
        return ERR_MEM;
    }

    mbox.msgs[mbox.last % SYS_MBOX_SIZE] = msg;

    if (mbox.last == mbox.first) {
        first = 1;
    } else {
        first = 0;
    }

    mbox.last += 1;

    if (first) {
        sys_sem_signal(&mbox.not_empty);
    }

    sys_sem_signal(&mbox.mutex);

    return Ok(());
}

pub fn sys_mbox_trypost_fromisr(q: &mut sys_mbox_t, msg: &mut Vec<u8>) {
    return sys_mbox_trypost(q, msg);
}

pub fn sys_mbox_post(mb: &mut sys_mbox, msg: &mut Vec<u8>) {
    let first: u8;
    let mut mbox: &mut sys_mbox;
    LWIP_ASSERT("invalid mbox", (mb != None) && (*mb != None));
    mbox = mb;

    sys_arch_sem_wait(&mbox.mutex, 0);

    //  LWIP_DEBUGF(SYS_DEBUG, ("sys_mbox_post: mbox %p msg %p\n", mbox, msg));

    while ((mbox.last + 1) >= (mbox.first + SYS_MBOX_SIZE)) {
        mbox.wait_send += 1;
        sys_sem_signal(&mbox.mutex);
        sys_arch_sem_wait(&mbox.not_full, 0);
        sys_arch_sem_wait(&mbox.mutex, 0);
        mbox.wait_send -= 1;
    }

    mbox.msgs[mbox.last % SYS_MBOX_SIZE] = msg;

    if (mbox.last == mbox.first) {
        first = 1;
    } else {
        first = 0;
    }

    mbox.last += 1;

    if (first) {
        sys_sem_signal(&mbox.not_empty);
    }

    sys_sem_signal(&mbox.mutex);
}

pub fn sys_arch_mbox_tryfetch(mb: &mut sys_mbox, msg: &mut Vec<u8>) -> u32 {
    let mut mbox: &mut sys_mbox;
    LWIP_ASSERT("invalid mbox", (mb != None) && (*mb != None));
    mbox = mb;

    sys_arch_sem_wait(&mbox.mutex, 0);

    if (mbox.first == mbox.last) {
        sys_sem_signal(&mbox.mutex);
        return SYS_MBOX_EMPTY;
    }

    if (msg != None) {
        //    LWIP_DEBUGF(SYS_DEBUG, ("sys_mbox_tryfetch: mbox %p msg %p\n", mbox, *msg));
        *msg = mbox.msgs[mbox.first % SYS_MBOX_SIZE];
    } else {
        //    LWIP_DEBUGF(SYS_DEBUG, ("sys_mbox_tryfetch: mbox %p, null msg\n", mbox));
    }

    mbox.first += 1;

    if (mbox.wait_send) {
        sys_sem_signal(&mbox.not_full);
    }

    sys_sem_signal(&mbox.mutex);

    return 0;
}

pub fn sys_arch_mbox_fetch(mb: &mut sys_mbox, msg: &mut Vec<u8>, timeout: u32) -> u32 {
    let time_needed: u32 = 0;
    let mut mbox: &mut sys_mbox;
    LWIP_ASSERT("invalid mbox", (mb != None) && (*mb != None));
    mbox = mb;

    /* The mutex lock is quick so we don't bother with the timeout
    stuff here. */
    sys_arch_sem_wait(&mbox.mutex, 0);

    while (mbox.first == mbox.last) {
        sys_sem_signal(&mbox.mutex);

        /* We block while waiting for a mail to arrive in the mailbox. We
        must be prepared to timeout. */
        if (timeout != 0) {
            time_needed = sys_arch_sem_wait(&mbox.not_empty, timeout);

            if (time_needed == SYS_ARCH_TIMEOUT) {
                return SYS_ARCH_TIMEOUT;
            }
        } else {
            sys_arch_sem_wait(&mbox.not_empty, 0);
        }

        sys_arch_sem_wait(&mbox.mutex, 0);
    }

    if (msg != None) {
        //    LWIP_DEBUGF(SYS_DEBUG, ("sys_mbox_fetch: mbox %p msg %p\n", mbox, *msg));
        *msg = mbox.msgs[mbox.first % SYS_MBOX_SIZE];
    } else {
        //    LWIP_DEBUGF(SYS_DEBUG, ("sys_mbox_fetch: mbox %p, null msg\n", mbox));
    }

    mbox.first += 1;

    if (mbox.wait_send) {
        sys_sem_signal(&mbox.not_full);
    }

    sys_sem_signal(&mbox.mutex);

    return time_needed;
}

// -----------------------------------------------------------------------------------
//  Semaphore
pub fn sys_sem_new_internal(count: u8) -> sys_sem {
    let mut sem: &mut sys_sem;

    sem = malloc(sizeof(sys_sem));
    if (sem != None) {
        sem.c = count;
        pthread_condattr_init(&(sem.condattr));

        pthread_condattr_setclock(&(sem.condattr), CLOCK_MONOTONIC);

        pthread_cond_init(&(sem.cond), &(sem.condattr));
        pthread_mutex_init(&(sem.mutex), None);
    }
    return sem;
}

pub fn sys_sem_new(sem: &mut sys_sem, count: u8) {
    SYS_STATS_INC_USED(sem);
    *sem = sys_sem_new_internal(count);
    if (*sem == None) {
        return ERR_MEM;
    }
    return Ok(());
}

pub fn cond_wait(cond: &mut pthread_cond_t, mutex: &mut pthread_mutex_t, timeout: u32) {
    // struct timespec rtime1, rtime2, ts;
    let rtime1: timespec;
    let rtime2: timespec;
    let ts: timespec;
    let letret: i32;

    // pub const pthread_cond_wait: u32 = pthread_hurd_cond_wait_np;
    // pub const pthread_cond_timedwait: u32 = pthread_hurd_cond_timedwait_np;

    if (timeout == 0) {
        pthread_cond_wait(cond, mutex);
        return 0;
    }

    //  Get a timestamp and add the timeout value.
    get_monotonic_time(&rtime1);

    ts.tv_sec = timeout / 1000;
    ts.tv_nsec = (timeout % 1000) * 1000000;
    ret = pthread_cond_timedwait_relative_np(cond, mutex, &ts);

    ts.tv_sec = rtime1.tv_sec + timeout / 1000;
    ts.tv_nsec = rtime1.tv_nsec + (timeout % 1000) * 1000000;
    if (ts.tv_nsec >= 1000000000) {
        ts.tv_sec += 1;
        ts.tv_nsec -= 1000000000;
    }

    ret = pthread_cond_timedwait(cond, mutex, &ts);

    if (ret == ETIMEDOUT) {
        return SYS_ARCH_TIMEOUT;
    }

    //  Calculate for how long we waited for the cond.
    get_monotonic_time(&rtime2);
    ts.tv_sec = rtime2.tv_sec - rtime1.tv_sec;
    ts.tv_nsec = rtime2.tv_nsec - rtime1.tv_nsec;
    if (ts.tv_nsec < 0) {
        ts.tv_sec -= 1;
        ts.tv_nsec += 1000000000;
    }
    return (ts.tv_sec * 1000 + ts.tv_nsec / 1000000);
}

pub fn sys_arch_sem_wait(s: sys_sem, timeout: u32) -> u32 {
    let time_needed: u32 = 0;
    let mut sem: &mut sys_sem;
    LWIP_ASSERT("invalid sem", (s != None) && (*s != None));
    sem = *s;

    pthread_mutex_lock(&(sem.mutex));
    while (sem.c <= 0) {
        if (timeout > 0) {
            time_needed = cond_wait(&(sem.cond), &(sem.mutex), timeout);

            if (time_needed == SYS_ARCH_TIMEOUT) {
                pthread_mutex_unlock(&(sem.mutex));
                return SYS_ARCH_TIMEOUT;
            }
            /*      pthread_mutex_unlock(&(sem.mutex));
            return time_needed; */
        } else {
            cond_wait(&(sem.cond), &(sem.mutex), 0);
        }
    }
    sem.c -= 1;
    pthread_mutex_unlock(&(sem.mutex));
    return time_needed;
}

pub fn sys_sem_signal(s: sys_sem) {
    let mut sem: &mut sys_sem;
    LWIP_ASSERT("invalid sem", (s != None) && (*s != None));
    sem = *s;

    pthread_mutex_lock(&(sem.mutex));
    sem.c += 1;

    if (sem.c > 1) {
        sem.c = 1;
    }

    pthread_cond_broadcast(&(sem.cond));
    pthread_mutex_unlock(&(sem.mutex));
}

pub fn sys_sem_free_internal(sem: &mut sys_sem) {
    pthread_cond_destroy(&(sem.cond));
    pthread_condattr_destroy(&(sem.condattr));
    pthread_mutex_destroy(&(sem.mutex));
    free(sem);
}

pub fn sys_sem_free(sem: &mut sys_sem) {
    if ((sem != None) && (*sem != SYS_SEM_None)) {
        SYS_STATS_DEC(sem.used);
        sys_sem_free_internal(sem);
    }
}

// -----------------------------------------------------------------------------------
//  Mutex
/* Create a new mutex
 * @param mutex pointer to the mutex to create
 * @return a new mutex */
pub fn sys_mutex_new(mutex: &mut sys_mutex) {
    let mut mtx: &mut sys_mutex;

    mtx = malloc(sizeof(sys_mutex));
    if (mtx != None) {
        pthread_mutex_init(&(mtx.mutex), None);
        *mutex = mtx;
        return Ok(());
    } else {
        return ERR_MEM;
    }
}

/* Lock a mutex
 * @param mutex the mutex to lock */
pub fn sys_mutex_lock(mutex: &mut sys_mutex) {
    pthread_mutex_lock(&((*mutex).mutex));
}

/* Unlock a mutex
 * @param mutex the mutex to unlock */
pub fn sys_mutex_unlock(mutex: &mut sys_mutex) {
    pthread_mutex_unlock(&((*mutex).mutex));
}

/* Delete a mutex
 * @param mutex the mutex to delete */
pub fn sys_mutex_free(mutex: &mut sys_mutex) {
    pthread_mutex_destroy(&((*mutex).mutex));
    free(*mutex);
}

// -----------------------------------------------------------------------------------
//  Time
pub fn sys_now() -> u32 {
    let ts: timespec;

    get_monotonic_time(&ts);
    return (ts.tv_sec * 1000 + ts.tv_nsec / 1000000);
}

pub fn sys_jiffies() -> u32 {
    let ts: timespec;

    get_monotonic_time(&ts);
    return (ts.tv_sec * 1000000000 + ts.tv_nsec);
}

// -----------------------------------------------------------------------------------
//  Init

pub fn sys_init() {}

// -----------------------------------------------------------------------------------
//  Critical section

/* sys_arch_protect: sys_prot_t()

This optional function does a "fast" critical region protection and returns
the previous protection level. This function is only called during very short
critical regions. An embedded system which supports ISR-based drivers might
want to implement this function by disabling interrupts. Task-based systems
might want to implement this by using a mutex or disabling tasking. This
function should support recursive calls from the same task or interrupt. In
other words, sys_arch_protect() could be called while already protected. In
that case the return value indicates that it is already protected.

sys_arch_protect() is only required if your port is supporting an operating
system.
*/
pub fn sys_arch_protect() -> sys_prot_t {
    /* Note that for the UNIX port, we are using a lightweight mutex, and our
     * own counter (which is locked by the mutex). The return code is not actually
     * used. */
    if (lwprot_thread != pthread_self()) {
        /* We are locking the mutex where it has not been locked before *
         * or is being locked by another thread */
        pthread_mutex_lock(&lwprot_mutex);
        lwprot_thread = pthread_self();
        lwprot_count = 1;
    } else {
        //  It is already locked by THIS thread
        lwprot_count += 1;
    }
    return 0;
}

/* void sys_arch_unprotect(pval: sys_prot_t)

This optional function does a "fast" set of critical region protection to the
value specified by pval. See the documentation for sys_arch_protect() for
more information. This function is only required if your port is supporting
an operating system.
*/
pub fn sys_arch_unprotect(pval: sys_prot_t) {
    if (lwprot_thread == pthread_self()) {
        lwprot_count -= 1;
        if (lwprot_count == 0) {
            lwprot_thread = 0xDEAD;
            pthread_mutex_unlock(&lwprot_mutex);
        }
    }
}
