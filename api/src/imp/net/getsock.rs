use crate::file::get_file_like;
use crate::socket::SocketAddrExt;
use crate::{
    imp::sys,
    ptr::{UserConstPtr, UserPtr},
};
use alloc::sync::Arc;
use axerrno::{LinuxError, LinuxResult};
use axhal::time::wall_time;
use axnet::{TcpSocket, UdpSocket};
use axtask::{TaskExtRef, current};
use core::ffi::{c_int, c_void};
use core::time::Duration;
use core::{
    fmt::Error,
    mem::{MaybeUninit, size_of},
    net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
};
use linux_raw_sys::general::{__kernel_fd_set, timeval};
use linux_raw_sys::net::{
    __kernel_sa_family_t, AF_INET, AF_INET6, in_addr, in6_addr, sockaddr, sockaddr_in,
    sockaddr_in6, socklen_t,
};
use linux_raw_sys::net::{SOCK_DGRAM, SOCK_STREAM};

pub fn sys_getsockname(
    fd: isize,
    addr: UserPtr<sockaddr>,
    addrlen: UserPtr<socklen_t>,
) -> LinuxResult<isize> {
    info!("sys_getsockname called with fd: {}", fd);

    // 检查参数有效性
    if addr.is_null() || addrlen.is_null() {
        return Err(LinuxError::EFAULT);
    }

    // 获取文件描述符对应的 socket
    let socket = get_file_like(fd as i32)?;
    info!("sys_getsockname found socket for fd {}", fd);

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

    // 获取 socket 的本地地址
    let local_addr = match socket.as_ref() {
        crate::file::Socket::Tcp(tcp_socket) => {
            let socket = tcp_socket.lock();
            socket.local_addr().map_err(|e| {
                error!("Failed to get TCP local address: {:?}", e);
                LinuxError::EINVAL
            })?
        }
        crate::file::Socket::Udp(udp_socket) => {
            let socket = udp_socket.lock();
            socket.local_addr().map_err(|e| {
                error!("Failed to get UDP local address: {:?}", e);
                LinuxError::EINVAL
            })?
        }
    };
    info!("local_addr.family: {}", local_addr.family());
    info!("local_addr: {}", local_addr);
    info!("local_addr.port().to_be(): {}", local_addr.port().to_be());
    // 从用户空间读取 addrlen
    let mut len = addrlen.get_as_mut()?;

    // 写入地址到用户空间
    let written_len = local_addr.write_to_user(addr)?;

    // 更新 addrlen
    *len = written_len;

    info!("sys_getsockname successfully returned local address");
    Ok(0)
}
