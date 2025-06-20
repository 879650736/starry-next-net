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

pub fn sys_recvfrom(
    fd: isize,
    buf: UserPtr<u8>,
    len: usize,
    flags: u32,
    addr: UserPtr<sockaddr>,
    addr_len: UserPtr<socklen_t>,
) -> LinuxResult<isize> {
    info!(
        "sys_recvfrom called with fd: {}, len: {}, flags: {}",
        fd, len, flags
    );

    // 获取文件描述符对应的 socket
    let socket = get_file_like(fd as i32)?;
    info!("sys_recvfrom found socket for fd {}", fd);

    // 检查是否为 Socket 类型
    let any_socket = socket.into_any();
    if !any_socket.is::<crate::file::Socket>() {
        error!("Object is NOT Socket type");
        return Err(LinuxError::ENOTSOCK);
    }

    // 转换为 Socket
    let socket = any_socket.downcast::<crate::file::Socket>().map_err(|_| {
        error!("Failed to downcast to Socket for fd {}", fd);
        LinuxError::ENOTSOCK
    })?;

    // 准备用户空间缓冲区
    let user_buf = buf.get_as_mut_slice(len)?;
    
    // 调用 recvfrom 方法接收数据
    let (received_bytes, src_addr) = socket.recvfrom(user_buf)?;
    info!("sys_recvfrom received {} bytes", received_bytes);
    
    // 如果提供了地址指针并且有源地址，则将源地址写入用户空间
    if !addr.is_null() && !addr_len.is_null() && src_addr.is_some() {
        let src_addr = src_addr.unwrap();
        info!("sys_recvfrom from address: {}", src_addr);
        
        let user_addr_len = addr_len.get_as_mut()?;
        
        // 检查用户提供的缓冲区大小是否足够
        let addr_size = src_addr.addr_len() as usize;
        if *user_addr_len as usize >= addr_size {
            // 使用write_to_user方法写入源地址到用户空间
            if let Ok(written_len) = src_addr.write_to_user(addr) {
                *user_addr_len = written_len;
                info!("sys_recvfrom updated source address information");
            }
        }
    }
    
    Ok(received_bytes as isize)
}