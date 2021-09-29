/*
 * Copyright (c) 2017 Simon Goldschmidt
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
 * Author: Simon Goldschmidt <goldsimon@gmx.de>
 *
 */

/* This is returned by _fromisr() sys functions to tell the outermost function
* that a higher priority task was woken and the scheduler needs to be invoked.
 */
pub const ERR_NEED_SCHED: u32 = 123;

//  lwIP includes. 

/* Set this to 1 if you want the stack size passed to sys_thread_new() to be
 * interpreted as number of stack words (FreeRTOS-like).
 * Default is that they are interpreted as byte count (lwIP-like).
 */

pub const LWIP_FREERTOS_THREAD_STACKSIZE_IS_STACKWORDS: u32 = 0;

/* Set this to 1 to use a mutex for SYS_ARCH_PROTECT() critical regions.
 * Default is 0 and locks interrupts/scheduler for SYS_ARCH_PROTECT().
 */

pub const LWIP_FREERTOS_SYS_ARCH_PROTECT_USES_MUTEX: u32 = 0;

/* Set this to 1 to include a sanity check that SYS_ARCH_PROTECT() and
 * SYS_ARCH_UNPROTECT() are called matching.
 */

pub const LWIP_FREERTOS_SYS_ARCH_PROTECT_SANITY_CHECK: u32 = 0;

//  Set this to 1 to let sys_mbox_free check that queues are empty when freed 

pub const LWIP_FREERTOS_CHECK_QUEUE_EMPTY_ON_FREE: u32 = 0;

/* Set this to 1 to enable core locking check functions in this port.
 * For this to work, you'll have to define LWIP_ASSERT_CORE_LOCKED()
 * and LWIP_MARK_TCPIP_THREAD() correctly in your lwipopts.h! */

pub const LWIP_FREERTOS_CHECK_CORE_LOCKING: u32 = 0;

/* Set this to 0 to implement sys_now() yourself, e.g. using a hw timer.
 * Default is 1, where FreeRTOS ticks are used to calculate back to ms.
 */

// #define LWIP_FREERTOS_SYS_NOW_FROM_FREERTOS           1

// # error "lwIP FreeRTOS port requires configSUPPORT_DYNAMIC_ALLOCATION"

// # error "lwIP FreeRTOS port requires INCLUDE_vTaskDelay"

// # error "lwIP FreeRTOS port requires INCLUDE_vTaskSuspend"

// # error "lwIP FreeRTOS port requires configUSE_MUTEXES"

// static SemaphoreHandle_t sys_arch_protect_mutex;

// static sys_arch_protect_nesting: sys_prot_t;

//  Initialize this module (see description in sys.h) 
pub fn sys_init() {
    //  initialize sys_arch_protect global mutex 
    sys_arch_protect_mutex = xSemaphoreCreateRecursiveMutex();
    LWIP_ASSERT(
        "failed to create sys_arch_protect mutex",
        sys_arch_protect_mutex != None,
    );
}

// #error This port requires 32 bit ticks or timer overflow will fail

pub fn sys_now() -> u32 {
    return xTaskGetTickCount() * portTICK_PERIOD_MS;
}

pub fn sys_jiffies() -> u32 {
    return xTaskGetTickCount();
}

pub fn sys_arch_protect() -> sys_prot_t {
    let ret: BaseType_t;
    LWIP_ASSERT(
        "sys_arch_protect_mutex != NULL",
        sys_arch_protect_mutex != None,
    );

    ret = xSemaphoreTakeRecursive(sys_arch_protect_mutex, portMAX_DELAY);
    LWIP_ASSERT("sys_arch_protect failed to take the mutex", ret == pdTRUE);
    //  LWIP_FREERTOS_SYS_ARCH_PROTECT_USES_MUTEX 
    taskENTER_CRITICAL();

    {
        //  every nested call to sys_arch_protect() returns an increased number 
        let ret: sys_prot_t = sys_arch_protect_nesting;
        sys_arch_protect_nesting += 1;
        LWIP_ASSERT("sys_arch_protect overflow", sys_arch_protect_nesting > ret);
        return ret;
    }

    return 1;
}

pub fn sys_arch_unprotect(pval: sys_prot_t) {
    let ret: BaseType_t;
    LWIP_ASSERT(
        "unexpected sys_arch_protect_nesting",
        sys_arch_protect_nesting > 0,
    );
    sys_arch_protect_nesting -= 1;
    LWIP_ASSERT(
        "unexpected sys_arch_protect_nesting",
        sys_arch_protect_nesting == pval,
    );

    LWIP_ASSERT(
        "sys_arch_protect_mutex != NULL",
        sys_arch_protect_mutex != None,
    );

    ret = xSemaphoreGiveRecursive(sys_arch_protect_mutex);
    LWIP_ASSERT("sys_arch_unprotect failed to give the mutex", ret == pdTRUE);
    //  LWIP_FREERTOS_SYS_ARCH_PROTECT_USES_MUTEX 
    taskEXIT_CRITICAL();
}

pub fn sys_arch_msleep(delay_ms: u32) {
    let delay_ticks: TickType_t = delay_ms / portTICK_RATE_MS;
    vTaskDelay(delay_ticks);
}

//  Create a new mutex
pub fn sys_mutex_new(mutex: &mut sys_mutex_t) {
    LWIP_ASSERT("mutex != NULL", mutex != None);

    mutex.val = xSemaphoreCreateRecursiveMutex();
    if (mutex.val == None) {
        SYS_STATS_INC(mutex.err);
        return ERR_MEM;
    }
    SYS_STATS_INC_USED(mutex);
    return Ok(());
}

pub fn sys_mutex_lock(mutex: &mut sys_mutex_t) {
    let ret: BaseType_t;
    LWIP_ASSERT("mutex != NULL", mutex != None);
    LWIP_ASSERT("mutex.mut != NULL", mutex.val != None);

    ret = xSemaphoreTakeRecursive(mutex.val, portMAX_DELAY);
    LWIP_ASSERT("failed to take the mutex", ret == pdTRUE);
}

pub fn sys_mutex_unlock(mutex: &mut sys_mutex_t) {
    let ret: BaseType_t;
    LWIP_ASSERT("mutex != NULL", mutex != None);
    LWIP_ASSERT("mutex.mut != NULL", mutex.val != None);

    ret = xSemaphoreGiveRecursive(mutex.val);
    LWIP_ASSERT("failed to give the mutex", ret == pdTRUE);
}

pub fn sys_mutex_free(mutex: &mut sys_mutex_t) {
    LWIP_ASSERT("mutex != NULL", mutex != None);
    LWIP_ASSERT("mutex.mut != NULL", mutex.val != None);

    SYS_STATS_DEC(mutex.used);
    vSemaphoreDelete(mutex.val);
    mutex.val = None;
}

pub fn sys_sem_new(sem: &mut sys_sem_t, initial_count: u8) {
    LWIP_ASSERT("sem != NULL", sem != None);
    LWIP_ASSERT(
        "initial_count invalid (not 0 or 1)",
        (initial_count == 0) || (initial_count == 1),
    );

    sem.sem = xSemaphoreCreateBinary();
    if (sem.sem == None) {
        SYS_STATS_INC(sem.err);
        return ERR_MEM;
    }
    SYS_STATS_INC_USED(sem);

    if (initial_count == 1) {
        let ret: BaseType_t = xSemaphoreGive(sem.sem);
        LWIP_ASSERT("sys_sem_new: initial give failed", ret == pdTRUE);
    }
    return Ok(());
}

pub fn sys_sem_signal(sem: &mut sys_sem_t) {
    let ret: BaseType_t;
    LWIP_ASSERT("sem != NULL", sem != None);
    LWIP_ASSERT("sem.sem != NULL", sem.sem != None);

    ret = xSemaphoreGive(sem.sem);
    //  queue full is OK, this is a signal only... 
    LWIP_ASSERT(
        "sys_sem_signal: sane return value",
        (ret == pdTRUE) || (ret == errQUEUE_FULL),
    );
}

pub fn sys_arch_sem_wait(sem: sys_sem_t, timeout_ms: u32) -> u32 {
    let ret: BaseType_t;
    LWIP_ASSERT("sem != NULL", sem != None);
    LWIP_ASSERT("sem.sem != NULL", sem.sem != None);

    if (!timeout_ms) {
        //  wait infinite 
        ret = xSemaphoreTake(sem.sem, portMAX_DELAY);
        LWIP_ASSERT("taking semaphore failed", ret == pdTRUE);
    } else {
        let timeout_ticks: TickType_t = timeout_ms / portTICK_RATE_MS;
        ret = xSemaphoreTake(sem.sem, timeout_ticks);
        if (ret == errQUEUE_EMPTY) {
            //  timed out 
            return SYS_ARCH_TIMEOUT;
        }
        LWIP_ASSERT("taking semaphore failed", ret == pdTRUE);
    }

    /* Old versions of lwIP required us to return the time waited.
    This is not the case any more. Just returning != SYS_ARCH_TIMEOUT
    here is enough. */
    return 1;
}

pub fn sys_sem_free(sem: &mut sys_sem_t) {
    LWIP_ASSERT("sem != NULL", sem != None);
    LWIP_ASSERT("sem.sem != NULL", sem.sem != None);

    SYS_STATS_DEC(sem.used);
    vSemaphoreDelete(sem.sem);
    sem.sem = None;
}

pub fn sys_mbox_new(mbox: &mut sys_mbox_t, size: i32) {
    LWIP_ASSERT("mbox != NULL", mbox != None);
    LWIP_ASSERT("size > 0", size > 0);

    // mbox.mbx = xQueueCreate((UBaseType_t)size, sizeof);
    if (mbox.mbx == None) {
        SYS_STATS_INC(mbox.err);
        return ERR_MEM;
    }
    SYS_STATS_INC_USED(mbox);
    return Ok(());
}

pub fn sys_mbox_post(mbox: &mut sys_mbox_t, msg: &mut Vec<u8>) {
    let ret: BaseType_t;
    LWIP_ASSERT("mbox != NULL", mbox != None);
    LWIP_ASSERT("mbox.mbx != NULL", mbox.mbx != None);

    ret = xQueueSendToBack(mbox.mbx, &msg, portMAX_DELAY);
    LWIP_ASSERT("mbox post failed", ret == pdTRUE);
}

pub fn sys_mbox_trypost(mbox: &mut sys_mbox_t, msg: &mut Vec<u8>) {
    let ret: BaseType_t;
    LWIP_ASSERT("mbox != NULL", mbox != None);
    LWIP_ASSERT("mbox.mbx != NULL", mbox.mbx != None);

    ret = xQueueSendToBack(mbox.mbx, &msg, 0);
    if (ret == pdTRUE) {
        return Ok(());
    } else {
        LWIP_ASSERT("mbox trypost failed", ret == errQUEUE_FULL);
        SYS_STATS_INC(mbox.err);
        return ERR_MEM;
    }
}

pub fn sys_mbox_trypost_fromisr(mbox: &mut sys_mbox_t, msg: &mut Vec<u8>) {
    let ret: BaseType_t;
    let xHigherPriorityTaskWoken: BaseType_t = pdFALSE;
    LWIP_ASSERT("mbox != NULL", mbox != None);
    LWIP_ASSERT("mbox.mbx != NULL", mbox.mbx != None);

    ret = xQueueSendToBackFromISR(mbox.mbx, &msg, &xHigherPriorityTaskWoken);
    if (ret == pdTRUE) {
        if (xHigherPriorityTaskWoken == pdTRUE) {
            return ERR_NEED_SCHED;
        }
        return Ok(());
    } else {
        LWIP_ASSERT("mbox trypost failed", ret == errQUEUE_FULL);
        SYS_STATS_INC(mbox.err);
        return ERR_MEM;
    }
}

pub fn sys_arch_mbox_fetch(mbox: &mut sys_mbox_t, msg: &mut Vec<u8>, timeout_ms: u32) -> u32 {
    let ret: BaseType_t;
    let msg_dummy: &mut Vec<u8>;
    LWIP_ASSERT("mbox != NULL", mbox != None);
    LWIP_ASSERT("mbox.mbx != NULL", mbox.mbx != None);

    if (!msg) {
        msg = &msg_dummy;
    }

    if (!timeout_ms) {
        //  wait infinite 
        ret = xQueueReceive(mbox.mbx, &(*msg), portMAX_DELAY);
        LWIP_ASSERT("mbox fetch failed", ret == pdTRUE);
    } else {
        let timeout_ticks: TickType_t = timeout_ms / portTICK_RATE_MS;
        ret = xQueueReceive(mbox.mbx, &(*msg), timeout_ticks);
        if (ret == errQUEUE_EMPTY) {
            //  timed out 
            *msg = None;
            return SYS_ARCH_TIMEOUT;
        }
        LWIP_ASSERT("mbox fetch failed", ret == pdTRUE);
    }

    /* Old versions of lwIP required us to return the time waited.
    This is not the case any more. Just returning != SYS_ARCH_TIMEOUT
    here is enough. */
    return 1;
}

pub fn sys_arch_mbox_tryfetch(mbox: &mut sys_mbox_t, msg: &mut Vec<u8>) -> u32 {
    let ret: BaseType_t;
    let msg_dummy: &mut Vec<u8>;
    LWIP_ASSERT("mbox != NULL", mbox != None);
    LWIP_ASSERT("mbox.mbx != NULL", mbox.mbx != None);

    if (!msg) {
        msg = &msg_dummy;
    }

    ret = xQueueReceive(mbox.mbx, &(*msg), 0);
    if (ret == errQUEUE_EMPTY) {
        *msg = None;
        return SYS_MBOX_EMPTY;
    }
    LWIP_ASSERT("mbox fetch failed", ret == pdTRUE);

    /* Old versions of lwIP required us to return the time waited.
    This is not the case any more. Just returning != SYS_ARCH_TIMEOUT
    here is enough. */
    return 1;
}

pub fn sys_mbox_free(mbox: &mut sys_mbox_t) {
    LWIP_ASSERT("mbox != NULL", mbox != None);
    LWIP_ASSERT("mbox.mbx != NULL", mbox.mbx != None);

    {
        let msgs_waiting: UBaseType_t = uxQueueMessagesWaiting(mbox.mbx);
        LWIP_ASSERT("mbox quence not empty", msgs_waiting == 0);

        if (msgs_waiting != 0) {
            SYS_STATS_INC(mbox.err);
        }
    }

    vQueueDelete(mbox.mbx);

    SYS_STATS_DEC(mbox.used);
}

pub fn sys_thread_new(
    name: &String,
    thread: lwip_thread_fn,
    arg: &mut Vec<u8>,
    stacksize: i32,
    prio: i32,
) -> sys_thread_t {
    let rtos_task: TaskHandle_t;
    let ret: BaseType_t;
    let lwip_thread: sys_thread_t;
    let rtos_stacksize: usize;

    LWIP_ASSERT("invalid stacksize", stacksize > 0);

    rtos_stacksize = stacksize;

    rtos_stacksize = stacksize / sizeof(StackType_t);

    /* lwIP's lwip_thread_fn matches FreeRTOS' TaskFunction_t, so we can pass the
    thread function without adaption here. */
    ret = xTaskCreate(thread, name, rtos_stacksize, arg, prio, &rtos_task);
    LWIP_ASSERT("task creation failed", ret == pdTRUE);
    lwip_thread.thread_handle = rtos_task;
    return lwip_thread;
}

pub fn sys_arch_netconn_sem_get() -> sys_sem_t {
    let ret: &mut Vec<u8>;
    let task: TaskHandle_t = xTaskGetCurrentTaskHandle();
    LWIP_ASSERT("task != NULL", task != None);

    ret = pvTaskGetThreadLocalStoragePointer(task, 0);
    return ret;
}

pub fn sys_arch_netconn_sem_alloc() {
    let ret: &mut Vec<u8>;
    let task: TaskHandle_t = xTaskGetCurrentTaskHandle();
    LWIP_ASSERT("task != NULL", task != None);

    ret = pvTaskGetThreadLocalStoragePointer(task, 0);
    if (ret == None) {
        sys_sem_t * sem;
        let err: err_t;
        //  need to allocate the memory for this semaphore 
        sem = mem_malloc(sizeof(sys_sem_t));
        LWIP_ASSERT("sem != NULL", sem != None);
        err = sys_sem_new(sem, 0);
        LWIP_ASSERT("err == ERR_OK", err == ERR_OK);
        LWIP_ASSERT("sem invalid", sys_sem_valid(sem));
        vTaskSetThreadLocalStoragePointer(task, 0, sem);
    }
}

pub fn sys_arch_netconn_sem_free() {
    let ret: &mut Vec<u8>;
    let task: TaskHandle_t = xTaskGetCurrentTaskHandle();
    LWIP_ASSERT("task != NULL", task != None);

    ret = pvTaskGetThreadLocalStoragePointer(task, 0);
    if (ret != None) {
        sys_sem_t * sem = ret;
        sys_sem_free(sem);
        mem_free(sem);
        vTaskSetThreadLocalStoragePointer(task, 0, None);
    }
}

//  configNUM_THREAD_LOCAL_STORAGE_POINTERS > 0 
//#error LWIP_NETCONN_SEM_PER_THREAD needs configNUM_THREAD_LOCAL_STORAGE_POINTERS

//  Flag the core lock held. A counter for recursive locks. 
// static lwip_core_lock_count: u8;
// static TaskHandle_t lwip_core_lock_holder_thread;

pub fn sys_lock_tcpip_core() {
    sys_mutex_lock(&lock_tcpip_core);
    if (lwip_core_lock_count == 0) {
        lwip_core_lock_holder_thread = xTaskGetCurrentTaskHandle();
    }
    lwip_core_lock_count += 1;
}

pub fn sys_unlock_tcpip_core() {
    lwip_core_lock_count -= 1;
    if (lwip_core_lock_count == 0) {
        lwip_core_lock_holder_thread = 0;
    }
    sys_mutex_unlock(&lock_tcpip_core);
}

// static TaskHandle_t lwip_tcpip_thread;

pub fn sys_mark_tcpip_thread() {
    lwip_tcpip_thread = xTaskGetCurrentTaskHandle();
}

pub fn sys_check_core_locking() {
    //  Embedded systems should check we are NOT in an interrupt context here 
    /* E.g. core Cortex-M3/M4 ports:
        configASSERT( ( portNVIC_INT_CTRL_REG & portVECTACTIVE_MASK ) == 0 );

    Instead, we use more generic FreeRTOS functions here, which should fail from ISR: */
    taskENTER_CRITICAL();
    taskEXIT_CRITICAL();

    if (lwip_tcpip_thread != 0) {
        let current_thread: TaskHandle_t = xTaskGetCurrentTaskHandle();

        LWIP_ASSERT(
            "Function called without core lock",
            current_thread == lwip_core_lock_holder_thread && lwip_core_lock_count > 0,
        );
        //  LWIP_TCPIP_CORE_LOCKING 
        LWIP_ASSERT(
            "Function called from wrong thread",
            current_thread == lwip_tcpip_thread,
        );
    }
}

pub struct _sys_mut {
    pub val: u32,
}
// typedef struct _sys_mut sys_mutex_t;
// #define sys_mutex_valid_val(mutex)   (mutex.mut != None)
// #define sys_mutex_valid(mutex)       (((mutex) != None) && sys_mutex_valid_val(*(mutex)))
// #define sys_mutex_set_invalid(mutex) ((mutex).mut = None)

pub struct _sys_sem {
    pub sem: u32,
}
// typedef struct _sys_sem sys_sem_t;
// #define sys_sem_valid_val(sema)   (sema.sem != None)
// #define sys_sem_valid(sema)       (((sema) != None) && sys_sem_valid_val(*(sema)))
// #define sys_sem_set_invalid(sema) ((sema).sem = None)

pub struct _sys_mbox {
    pub mbx: u32,
}
// typedef struct _sys_mbox sys_mbox_t;
// #define sys_mbox_valid_val(mbox)   (mbox.mbx != None)
// #define sys_mbox_valid(mbox)       (((mbox) != None) && sys_mbox_valid_val(*(mbox)))
// #define sys_mbox_set_invalid(mbox) ((mbox).mbx = None)

pub struct _sys_thread {
    thread_handle: u32,
}
// typedef struct _sys_thread sys_thread_t;
