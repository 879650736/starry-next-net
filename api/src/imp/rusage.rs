use axerrno::{LinuxError, LinuxResult};
use axhal::time::{monotonic_time, monotonic_time_nanos, nanos_to_ticks, wall_time};
use linux_raw_sys::general::{
    __kernel_clockid_t, rusage, timespec, timeval, RUSAGE_SELF, RUSAGE_CHILDREN
};
use starry_core::task::time_stat_output;
use axhal::time::TimeValue;
use alloc::collections::BTreeMap;
use alloc::string::String;
use axsync::Mutex;
use axtask::{TaskExtRef, current};
use crate::{ptr::UserPtr, time::TimeValueLike};

pub struct RUsage {
    ru_utime: timeval,    // 用户CPU时间
    ru_stime: timeval,    // 系统CPU时间
    ru_maxrss: i64,       // 最大常驻集大小
    ru_ixrss: i64,        // 共享内存大小
    ru_idrss: i64,        // 非共享内存大小
    ru_isrss: i64,        // 栈大小
    ru_minflt: i64,       // 页面错误数
    ru_majflt: i64,       // 主要页面错误数
    ru_nswap: i64,        // 交换次数
    ru_inblock: i64,      // 块输入操作
    ru_oublock: i64,      // 块输出操作
    ru_msgsnd: i64,       // 发送的消息数
    ru_msgrcv: i64,       // 接收的消息数
    ru_nsignals: i64,     // 接收的信号数
    ru_nvcsw: i64,        // 自愿上下文切换
    ru_nivcsw: i64,       // 非自愿上下文切换
}

pub fn sys_getrusage(who: i32, usage: UserPtr<RUsage>) -> LinuxResult<isize> {
    if who != RUSAGE_SELF {
        warn!("sys_getrusage: unsupported 'who' parameter: {}", who);
        return Err(LinuxError::EINVAL);
    }
    
    // 获取当前任务的时间统计信息
    // 返回的顺序是：(utime_sec, utime_us, stime_sec, stime_us)
    let (_, utime_us, _, stime_us) = time_stat_output();
    
    // 填充 RUsage 结构体
    let mut rusage = RUsage {
        // 将微秒时间转换为 timeval 结构体
        ru_utime: timeval {
            tv_sec: (utime_us / 1_000_000) as i64,
            tv_usec: (utime_us % 1_000_000) as i64,
        },
        ru_stime: timeval {
            tv_sec: (stime_us / 1_000_000) as i64,
            tv_usec: (stime_us % 1_000_000) as i64,
        },
        // 其他字段设置为0
        ru_maxrss: 0,
        ru_ixrss: 0,
        ru_idrss: 0,
        ru_isrss: 0,
        ru_minflt: 0,
        ru_majflt: 0,
        ru_nswap: 0,
        ru_inblock: 0,
        ru_oublock: 0,
        ru_msgsnd: 0,
        ru_msgrcv: 0,
        ru_nsignals: 0,
        ru_nvcsw: 0,
        ru_nivcsw: 0,
    };
    
    // 将结果写入用户空间
    *usage.get_as_mut()? = rusage;
    
    Ok(0)
}