use axerrno::{LinuxError, LinuxResult};
use axhal::time::{monotonic_time, monotonic_time_nanos, nanos_to_ticks, wall_time};
use linux_raw_sys::general::{
    __kernel_clockid_t, CLOCK_MONOTONIC, CLOCK_REALTIME, CLOCK_PROCESS_CPUTIME_ID, timespec, timeval,
};
use starry_core::task::time_stat_output;
use axhal::time::TimeValue;
use alloc::collections::BTreeMap;
use alloc::string::String;
use axsync::Mutex;
use axtask::{TaskExtRef, current};
use crate::{ptr::UserPtr, time::TimeValueLike};

pub static APP_EXECUTION_TIMES: Mutex<BTreeMap<String, TimeValue>> = Mutex::new(BTreeMap::new());

pub fn sys_clock_gettime(
    clock_id: __kernel_clockid_t,
    ts: UserPtr<timespec>,
) -> LinuxResult<isize> {
    let now = match clock_id as u32 {
        CLOCK_REALTIME => wall_time(),
        CLOCK_MONOTONIC => monotonic_time(),
        CLOCK_PROCESS_CPUTIME_ID => {
            // 获取当前进程名称
            if let task = current() {
                let exe_path = task.task_ext().process_data().exe_path.read().clone();
                info!("sys_clock_gettime: exe_path = {}", exe_path);
                let current_time = monotonic_time();
                
                // 从记录中获取进程的启动时间，计算执行时间差
                let times = APP_EXECUTION_TIMES.lock();
                if let Some(start_time) = times.get(&exe_path) {
                    // 返回从启动到现在的时间差作为CPU时间
                    current_time.saturating_sub(*start_time)
                } else {
                    // 如果没有记录，则返回0时间
                    TimeValue::from_secs(0)
                }
            } else {
                TimeValue::from_secs(0)
            }
        },
        _ => {
            warn!(
                "Called sys_clock_gettime for unsupported clock {}",
                clock_id
            );
            return Err(LinuxError::EINVAL);
        }
    };
    *ts.get_as_mut()? = timespec::from_time_value(now);
    Ok(0)
}

pub fn sys_gettimeofday(ts: UserPtr<timeval>) -> LinuxResult<isize> {
    *ts.get_as_mut()? = timeval::from_time_value(wall_time());
    Ok(0)
}

#[repr(C)]
pub struct Tms {
    /// user time
    tms_utime: usize,
    /// system time
    tms_stime: usize,
    /// user time of children
    tms_cutime: usize,
    /// system time of children
    tms_cstime: usize,
}

pub fn sys_times(tms: UserPtr<Tms>) -> LinuxResult<isize> {
    let (_, utime_us, _, stime_us) = time_stat_output();
    *tms.get_as_mut()? = Tms {
        tms_utime: utime_us,
        tms_stime: stime_us,
        tms_cutime: utime_us,
        tms_cstime: stime_us,
    };
    Ok(nanos_to_ticks(monotonic_time_nanos()) as _)
}
