//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus,acuqire_task_info},
    timer::get_time_us,
};


#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    pub status: TaskStatus,
    /// The numbers of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    /// //// Total  time of task in ms since first scheduled
    pub time: usize,
    
    // first_scheduled_passed_time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    // let mut inner = TASK_MANAGER.inner.exclusive_access();
    // // let mut inner = self.inner.exclusive_access();
    // let current_task_id = inner.current_task;
    // let current_task = &inner.tasks[current_task_id];
    // // inner.tasks[current].task_status = TaskStatus::Ready;
    // unsafe {
    //     if ti.is_null() { return -1; }

    //     let task_info = &mut *ti;
    //     task_info.status = current_task.task_status;
    //     task_info.syscall_times.copy_from_slice(&current_task.syscall_times);
    //     task_info.time = get_time_ms() - current_task.first_scheduled_time + current_task.total_run_time;
    // }
    acuqire_task_info(ti)
    // trace!("kernel: sys_task_info");
    // -1
}
