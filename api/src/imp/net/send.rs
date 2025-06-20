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

pub fn sys_sendto(
    fd: isize,
    buf: UserConstPtr<u8>,
    len: usize,
    flags: u32,
    addr: UserConstPtr<sockaddr>,
    addr_len: socklen_t,
) -> LinuxResult<isize> {
    
    info!("sys_sendto called with fd: {}, len: {}, flags: {}, addr_len: {}", fd, len, flags, addr_len);

    // 获取文件描述符对应的 socket
    let socket = get_file_like(fd as i32)?;
    info!("sys_sendto found socket for fd {}", fd);

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

    // 从用户空间读取缓冲区数据
    let user_buf = buf.get_as_slice(len)?;
    
    
    // 如果提供了地址，则发送到指定地址
    if !addr.is_null() && addr_len > 0 {
        // 将 SockAddr 转换为标准的 SocketAddr
        let socket_addr = SocketAddr::read_from_user(addr, addr_len)?;
        info!("sys_sendto to address: {}", socket_addr);

        // 使用 sendto 方法发送数据
        let sent_bytes = socket.sendto(user_buf, socket_addr)?;
        info!("sys_sendto sent {} bytes", sent_bytes);
        Ok(sent_bytes as isize)
    } else {
        // 没有提供地址，使用 send 方法发送数据
        info!("sys_sendto without address, using send");
        let sent_bytes = socket.send(user_buf)?;
        info!("sys_sendto sent {} bytes", sent_bytes);
        Ok(sent_bytes as isize)
    }
}