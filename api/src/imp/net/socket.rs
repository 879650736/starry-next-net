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
    let socket = match socket_type { 
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

pub fn sys_socketpair(domain: u32, socket_type: u32, protocol: u32, sv: &mut [isize; 2]) -> LinuxResult<isize> {
    info!("sys_socketpair called with domain: {}, type: {}, protocol: {}", domain, socket_type, protocol);

    // 检查地址族
    if domain != AF_INET && domain != AF_INET6 {
        return Err(LinuxError::EAFNOSUPPORT);
    }

    // 创建两个相同类型的 socket
    let socket1 = match socket_type {
        SOCK_STREAM => {
            let tcp_socket = TcpSocket::new();
            crate::file::Socket::Tcp(axsync::Mutex::new(tcp_socket))
        }
        SOCK_DGRAM => {
            let udp_socket = UdpSocket::new();
            crate::file::Socket::Udp(axsync::Mutex::new(udp_socket))
        }
        _ => {
            return Err(LinuxError::EPROTONOSUPPORT);
        }
    };

    let socket2 = match socket_type {
        SOCK_STREAM => {
            let tcp_socket = TcpSocket::new();
            crate::file::Socket::Tcp(axsync::Mutex::new(tcp_socket))
        }
        SOCK_DGRAM => {
            let udp_socket = UdpSocket::new();
            crate::file::Socket::Udp(axsync::Mutex::new(udp_socket))
        }
        _ => {
            return Err(LinuxError::EPROTONOSUPPORT);
        }
    };

    // 获取当前进程并分配文件描述符
    sv[0] = crate::file::add_file_like(Arc::new(socket1))? as isize;
    sv[1] = crate::file::add_file_like(Arc::new(socket2))? as isize;

    info!("sys_socketpair created sockets with fds: {}, {}", sv[0], sv[1]);
    Ok(0)
}
pub fn sys_bind(fd: isize, addr: &crate::sockaddr::SockAddr, addr_len: u32) -> LinuxResult<isize> {
    info!("sys_bind called with fd: {}, addr_len: {}", fd, addr_len);

    // 获取文件描述符对应的 socket
    let socket = crate::file::get_file_like(fd as i32)?;
    info!("sys_bind found socket for fd {}", fd);
    // 检查是否为 Socket 类型
     let any_socket = socket.into_any();
    if any_socket.is::<crate::file::Socket>() {
        info!("Object is Socket type");
    } else {
        error!("Object is NOT Socket type");
        if any_socket.is::<crate::file::File>() {
            error!("Object is File type");
        } else if any_socket.is::<crate::file::Pipe>() {
            error!("Object is Pipe type");
        } else {
            error!("Object is unknown type");
        }
        return Err(LinuxError::ENOTSOCK);
    }
    
    // 检查是否为 Socket 类型
    let socket = any_socket
        .downcast::<crate::file::Socket>()
        .map_err(|_| {
            error!("Failed to downcast to Socket for fd {}", fd);
            LinuxError::ENOTSOCK
        })?;
    info!("sys_bind successfully downcasted to Socket for fd {}", fd);
    // 将 SockAddr 转换为标准的 SocketAddr
    let socket_addr: core::net::SocketAddr = addr.clone().try_into()?;
    info!("sys_bind converted to SocketAddr: {}", socket_addr);

    // 调用 socket 的 bind 方法
    socket.bind(socket_addr)?;

    info!("sys_bind successfully bound fd {}", fd);
    Ok(0)
}
pub fn sys_connect(fd: isize, addr: &crate::sockaddr::SockAddr, addr_len: u32) -> LinuxResult<isize> {
    info!("sys_connect called with fd: {}, addr_len: {}", fd, addr_len);

    // 获取文件描述符对应的 socket
    let socket = crate::file::get_file_like(fd as i32)?;
    info!("sys_connect found socket for fd {}", fd);

    // 检查是否为 Socket 类型
    let any_socket = socket.into_any();
    if any_socket.is::<crate::file::Socket>() {
        info!("Object is Socket type");
    } else {
        error!("Object is NOT Socket type");
        return Err(LinuxError::ENOTSOCK);
    }

    // 将 SockAddr 转换为标准的 SocketAddr
    let socket_addr: core::net::SocketAddr = addr.clone().try_into()?;
    info!("sys_connect converted to SocketAddr: {}", socket_addr);

    // 调用 socket 的 connect 方法
    let socket = any_socket
        .downcast::<crate::file::Socket>()
        .map_err(|_| {
            error!("Failed to downcast to Socket for fd {}", fd);
            LinuxError::ENOTSOCK
        })?;
    
    socket.connect(socket_addr)?;

    info!("sys_connect successfully connected fd {}", fd);
    Ok(0)
}

