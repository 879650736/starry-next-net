use crate::file::get_file_like;
use crate::ptr::{UserConstPtr, UserPtr};
use crate::socket::SocketAddrExt;
use alloc::sync::Arc;
use axerrno::{LinuxError, LinuxResult};
use axnet::{TcpSocket, UdpSocket};
use axtask::{TaskExtRef, current};
use core::ffi::c_void;
use core::{
    fmt::Error,
    mem::{MaybeUninit, size_of},
    net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
};
use linux_raw_sys::net::{
    __kernel_sa_family_t, AF_INET, AF_INET6, in_addr, in6_addr, sockaddr, sockaddr_in,
    sockaddr_in6, socklen_t,
};
use linux_raw_sys::general::pollfd;
use linux_raw_sys::net::{SOCK_DGRAM, SOCK_STREAM};
use axhal::time::{monotonic_time, monotonic_time_nanos};
use linux_raw_sys::general::{timespec, sigset_t};

pub fn sys_poll(
    fds: UserPtr<pollfd>, 
    nfds: u32, 
    timeout: i32
) -> LinuxResult<isize> {
    info!("sys_poll called with nfds: {}, timeout: {}", nfds, timeout);
    info!("Current task: {:?}", current().id());  // 添加当前任务信息

    // 检查参数有效性
    if nfds == 0 {
        info!("sys_poll early return: nfds is 0");  // 添加提前返回原因
        return Ok(0);
    }

    // 读取文件描述符数组
    // 使用显式初始化而不是default()
    let mut poll_fds = alloc::vec![
        pollfd { fd: 0, events: 0, revents: 0 }; 
        nfds as usize
    ];
    let user_poll_fds = fds.get_as_mut_slice(nfds as usize)?;
    for i in 0..nfds as usize {  // 使用usize类型进行索引
        poll_fds[i] = user_poll_fds[i];
    }
    
    // 打印初始的 poll_fds 和 user_poll_fds
    info!("Initial poll_fds:");
    for (i, pfd) in poll_fds.iter().enumerate() {
        info!("  [{}] fd={}, events=0x{:x}, revents=0x{:x}", i, pfd.fd, pfd.events, pfd.revents);
    }
    info!("Initial user_poll_fds:");
    for (i, pfd) in user_poll_fds.iter().enumerate() {
        info!("  [{}] fd={}, events=0x{:x}, revents=0x{:x}", i, pfd.fd, pfd.events, pfd.revents);
    }
    
    // 如果timeout为0，立即检查一次并返回
    if timeout == 0 {
        info!("sys_poll with timeout=0, performing single non-blocking check");  // 添加非阻塞检查说明
        return poll_once(&mut poll_fds, fds);
    }

    // 计算绝对截止时间（如果有超时）
    let deadline = if timeout > 0 {
        let now = monotonic_time_nanos();  // 记录当前时间
        let deadline = now + timeout as u64 * 1_000_000;
        info!("Setting deadline: current={}ns + timeout={}ms = {}ns", now, timeout, deadline);  // 添加详细的超时计算
        Some(deadline)
    } else {
        info!("No deadline set (infinite wait)");  // 添加无限等待说明
        None // timeout < 0 表示无限等待
    };

    // 轮询直到有事件发生或超时
    let mut poll_count = 0;  // 添加轮询计数器
    loop {
        poll_count += 1;
        info!("Poll iteration {}", poll_count);  // 记录轮询次数
        
        let ready_count = poll_once(&mut poll_fds, fds)?;
        if ready_count > 0 {
            info!("sys_poll returning with {} ready events after {} iterations", ready_count, poll_count);  // 添加结果摘要
            return Ok(ready_count);
        }

        // 检查是否超时
        if let Some(deadline) = deadline {
            let now = monotonic_time_nanos();
            if now >= deadline {
                info!("sys_poll timeout reached after {} iterations", poll_count);  // 添加超时返回信息
                return Ok(0); // 超时返回0
            }
            info!("Not timed out yet, remaining: {}ns", deadline - now);  // 添加剩余时间信息
        }

        // 让出CPU时间片，等待下一次轮询
        info!("Yielding CPU before next poll iteration");  // 添加让出CPU说明
        axtask::yield_now();
    }
}

// 执行一次轮询操作，返回就绪的文件描述符数量
fn poll_once(poll_fds: &mut [pollfd], user_fds: UserPtr<pollfd>) -> LinuxResult<isize> {
    let mut ready_count = 0;
    let user_poll_fds = user_fds.get_as_mut_slice(poll_fds.len())?;
    
    info!("poll_once begins, checking {} fds", poll_fds.len());
    info!("Input poll_fds:");
    for (i, pfd) in poll_fds.iter().enumerate() {
        info!("  [{}] fd={}, events=0x{:x}, revents=0x{:x}", i, pfd.fd, pfd.events, pfd.revents);
    }

    for (i, pfd) in poll_fds.iter_mut().enumerate() {
        if pfd.fd < 0 {
            // 忽略负的文件描述符
            info!("  [{}] fd={} (negative, ignoring)", i, pfd.fd);
            continue;
        }

        // 清除返回事件
        pfd.revents = 0;
        info!("  [{}] fd={}, checking events=0x{:x}...", i, pfd.fd, pfd.events);  // 添加events值

        // 获取文件描述符对应的对象
        match crate::file::get_file_like(pfd.fd as i32) {
            Ok(file) => {
                info!("    got file for fd={}", pfd.fd);  // 添加文件类型信息
                // 检查文件的poll状态
                if let Ok(poll_state) = file.poll() {
                    info!("    poll_state: readable={}, writable={}", 
                          poll_state.readable, poll_state.writable);
                    
                    // 设置返回事件，使用正确的类型转换
                    // 将常量转换为i16类型以匹配pfd.events和pfd.revents
                    if poll_state.readable && (pfd.events & (linux_raw_sys::general::POLLIN as i16) != 0) {
                        pfd.revents |= linux_raw_sys::general::POLLIN as i16;
                        info!("    setting POLLIN (0x{:x})", linux_raw_sys::general::POLLIN as i16);  // 添加具体值
                    }
                    if poll_state.writable && (pfd.events & (linux_raw_sys::general::POLLOUT as i16) != 0) {
                        pfd.revents |= linux_raw_sys::general::POLLOUT as i16;
                        info!("    setting POLLOUT (0x{:x})", linux_raw_sys::general::POLLOUT as i16);  // 添加具体值
                    }
                } else {
                    // 发生错误
                    pfd.revents |= linux_raw_sys::general::POLLERR as i16;
                    info!("    setting POLLERR (0x{:x})", linux_raw_sys::general::POLLERR as i16);  // 添加具体值
                }
            },
            Err(e) => {
                // 无效的文件描述符
                pfd.revents |= linux_raw_sys::general::POLLNVAL as i16;
                info!("    setting POLLNVAL (0x{:x}): {:?}", linux_raw_sys::general::POLLNVAL as i16, e);  // 添加错误详情
            }
        }

        // 将结果写回到用户空间
        user_poll_fds[i].revents = pfd.revents;
        info!("  [{}] updated: revents=0x{:x}", i, pfd.revents);

        // 统计就绪的文件描述符数量
        if pfd.revents != 0 {
            ready_count += 1;
        }
    }

    info!("poll_once results: {} fds ready", ready_count);
    info!("Final poll_fds:");
    for (i, pfd) in poll_fds.iter().enumerate() {
        info!("  [{}] fd={}, events=0x{:x}, revents=0x{:x}", i, pfd.fd, pfd.events, pfd.revents);
    }
    info!("Final user_poll_fds:");
    for (i, pfd) in user_poll_fds.iter().enumerate() {
        info!("  [{}] fd={}, events=0x{:x}, revents=0x{:x}", i, pfd.fd, pfd.events, pfd.revents);
    }

    Ok(ready_count)
}

pub fn sys_ppoll(
    fds: UserPtr<pollfd>,
    nfds: u32,
    timeout_ts: UserConstPtr<timespec>,
    sigmask: UserConstPtr<sigset_t>,
    sigsetsize: usize,
) -> LinuxResult<isize> {
    info!("sys_ppoll called with nfds: {}", nfds);
    info!("Current task: {:?}", current().id());  // 添加当前任务信息
    
    // 添加更详细的超时信息
    if !timeout_ts.is_null() {
        if let Ok(ts) = timeout_ts.get_as_ref() {
            info!("  timeout: {}.{:09}s", ts.tv_sec, ts.tv_nsec);
        } else {
            info!("  timeout: [error reading]");
        }
    } else {
        info!("  timeout: NULL (infinite)");
    }
    
    info!("  sigmask: {}, sigsetsize: {}", 
          if sigmask.is_null() { "NULL" } else { "present" }, 
          sigsetsize);

    // 检查参数有效性
    if nfds == 0 {
        info!("sys_ppoll early return: nfds is 0");  // 添加提前返回原因
        return Ok(0);
    }

    // 处理超时参数
    let timeout_ns = if !timeout_ts.is_null() {
        let ts = timeout_ts.get_as_ref()?;
        if ts.tv_sec < 0 || ts.tv_nsec < 0 || ts.tv_nsec >= 1_000_000_000 {
            info!("Invalid timespec values: tv_sec={}, tv_nsec={}", ts.tv_sec, ts.tv_nsec);  // 添加无效参数详情
            return Err(LinuxError::EINVAL);
        }
        let ns = (ts.tv_sec as u64) * 1_000_000_000 + (ts.tv_nsec as u64);
        info!("Timeout converted to {}ns", ns);  // 添加转换后的超时值
        Some(ns)
    } else {
        None // NULL 表示无限等待
    };

    if !sigmask.is_null() && sigsetsize != size_of::<sigset_t>() {
        info!("Invalid sigsetsize: {} (expected {})", sigsetsize, size_of::<sigset_t>());  // 添加无效参数详情
        return Err(LinuxError::EINVAL);
    }

    // 读取文件描述符数组
    let mut poll_fds = alloc::vec![
        pollfd { fd: 0, events: 0, revents: 0 }; 
        nfds as usize
    ];
    let user_poll_fds = fds.get_as_mut_slice(nfds as usize)?;
    for i in 0..nfds as usize {
        poll_fds[i] = user_poll_fds[i];
    }
    
    info!("Initial poll_fds in ppoll:");
    for (i, pfd) in poll_fds.iter().enumerate() {
        info!("  [{}] fd={}, events=0x{:x}, revents=0x{:x}", i, pfd.fd, pfd.events, pfd.revents);
    }
    info!("Initial user_poll_fds:");
    for (i, pfd) in user_poll_fds.iter().enumerate() {
        info!("  [{}] fd={}, events=0x{:x}, revents=0x{:x}", i, pfd.fd, pfd.events, pfd.revents);
    }
    
    // 如果timeout为0，立即检查一次并返回
    if let Some(timeout) = timeout_ns {
        if timeout == 0 {
            info!("sys_ppoll with timeout=0, performing single non-blocking check");  // 添加非阻塞检查说明
            return poll_once(&mut poll_fds, fds);
        }
    }

    // 计算绝对截止时间（如果有超时）
    let deadline = timeout_ns.map(|ns| {
        let now = monotonic_time_nanos();
        let deadline = now + ns;
        info!("Setting deadline: current={}ns + timeout={}ns = {}ns", now, ns, deadline);  // 添加详细的超时计算
        deadline
    });

    // 轮询直到有事件发生或超时
    let mut poll_count = 0;  // 添加轮询计数器
    loop {
        poll_count += 1;
        info!("Poll iteration {}", poll_count);  // 记录轮询次数
        
        let ready_count = poll_once(&mut poll_fds, fds)?;
        if ready_count > 0 {
            info!("sys_ppoll returning with {} ready events after {} iterations", ready_count, poll_count);  // 添加结果摘要
            return Ok(ready_count);
        }

        // 检查是否超时
        if let Some(deadline) = deadline {
            let now = monotonic_time_nanos();
            if now >= deadline {
                info!("sys_ppoll timeout reached after {} iterations", poll_count);  // 添加超时返回信息
                return Ok(0); // 超时返回0
            }
            info!("Not timed out yet, remaining: {}ns", deadline - now);  // 添加剩余时间信息
        }

        // 让出CPU时间片，等待下一次轮询
        info!("Yielding CPU before next ppoll iteration");  // 添加让出CPU说明
        axtask::yield_now();
    }
}