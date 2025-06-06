use alloc::sync::Arc;
use axerrno::{LinuxError, LinuxResult};
use axnet::{TcpSocket, UdpSocket};
use linux_raw_sys::net::{AF_INET, AF_INET6, SOCK_STREAM, SOCK_DGRAM};
use axtask::{TaskExtRef, current};

pub fn sys_socket(domain: u32, socket_type: u32, protocol: u32) -> LinuxResult<isize> {
    info!("sys_socket called with domain: {}, type: {}, protocol: {}", domain, socket_type, protocol);

    // 检查地址族
    if domain != AF_INET && domain != AF_INET6 {
        return Err(LinuxError::EAFNOSUPPORT);
    }

    // 创建相应类型的 socket
    let socket = match socket_type {  // 去掉标志位，只保留类型
        SOCK_STREAM => {
            // TCP socket
            let tcp_socket = TcpSocket::new();
            crate::file::Socket::Tcp(axsync::Mutex::new(tcp_socket))
        }
        SOCK_DGRAM => {
            // UDP socket
            let udp_socket = UdpSocket::new();
            crate::file::Socket::Udp(axsync::Mutex::new(udp_socket))
        }
        _ => {
            return Err(LinuxError::EPROTONOSUPPORT);
        }
    };

    // 获取当前进程并分配文件描述符
    let fd = crate::file::add_file_like(Arc::new(socket))?;

    info!("sys_socket created socket with fd: {}", fd);
    Ok(fd as isize)
}