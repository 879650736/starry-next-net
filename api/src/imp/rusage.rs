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
use starry_core::task::{APP_EXECUTION_TIMES, AppExecutionStats};

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

pub fn sys_getrusage(who: u32, usage: UserPtr<RUsage>) -> LinuxResult<isize> {
    if who != RUSAGE_SELF {
        warn!("sys_getrusage: unsupported 'who' parameter: {}", who);
        return Err(LinuxError::EINVAL);
    }
    let mut adjusted_utime_ns = 0;
    let mut adjusted_stime_ns = 0;
   
    if let task = current() {
        let exe_path = task.task_ext().process_data().exe_path.read().clone();

        // 从记录中获取进程的初始时间统计数据
        let times = APP_EXECUTION_TIMES.lock();
        if let Some(stats) = times.get(&exe_path) {
            // 减去初始时间，只计算应用程序实际运行时间
            info!("stats.user_time_ns.unwrap(): {}, stats.system_time_ns.unwrap(): {}",
                  stats.user_time_ns.unwrap(), stats.system_time_ns.unwrap());
             // 获取当前任务的时间统计信息
            let (_, utime_us, _, stime_us) 
            = time_stat_output();
            
            // 将当前时间转换为纳秒并减去初始时间
            let current_utime_ns = utime_us * 1_000;
            let current_stime_ns = stime_us * 1_000;
            info!("current_utime_ns: {}, current_stime_ns: {}",
                  current_utime_ns, current_stime_ns);
            
            adjusted_utime_ns = current_utime_ns.saturating_sub(stats.user_time_ns.unwrap());
            adjusted_stime_ns = current_stime_ns.saturating_sub(stats.system_time_ns.unwrap());
            info!("adjusted_utime_ns: {}, adjusted_stime_ns: {}",
                  adjusted_utime_ns, adjusted_stime_ns);
        }
    }
    
    // 计算调整后的秒和微秒
    let adjusted_utime_sec = adjusted_utime_ns / 1_000_000_000;
    let adjusted_utime_usec = (adjusted_utime_ns % 1_000_000_000) / 1_000;
    let adjusted_stime_sec = adjusted_stime_ns / 1_000_000_000;
    let adjusted_stime_usec = (adjusted_stime_ns % 1_000_000_000) / 1_000;

    info!("调整后 utime_sec:{}, utime_usec:{}, stime_sec:{}, stime_usec:{}", 
          adjusted_utime_sec, adjusted_utime_usec, adjusted_stime_sec, adjusted_stime_usec);
    
    // 填充 RUsage 结构体
    let mut rusage = RUsage {
        // 使用调整后的秒和微秒分量填充 timeval
        ru_utime: timeval {
            tv_sec: adjusted_utime_sec as i64,
            tv_usec: adjusted_utime_usec as i64,
        },
        ru_stime: timeval {
            tv_sec: adjusted_stime_sec as i64,
            tv_usec: adjusted_stime_usec as i64,
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