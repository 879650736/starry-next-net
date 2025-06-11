use alloc::sync::Arc;
use axerrno::{LinuxError, LinuxResult};
use axnet::{TcpSocket, UdpSocket};
use linux_raw_sys::net::{SOCK_STREAM, SOCK_DGRAM};
use axtask::{TaskExtRef, current};
use crate::ptr::{UserPtr, UserConstPtr};
use linux_raw_sys::net::{
    __kernel_sa_family_t, AF_INET, AF_INET6, in_addr, in6_addr, sockaddr, sockaddr_in,
    sockaddr_in6, socklen_t
};
use crate::socket::SocketAddrExt;
use core::{
    fmt::Error, mem::{size_of, MaybeUninit}, net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6}
};
use core::ffi::c_void;
use crate::file::get_file_like;

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
pub fn sys_bind(fd: isize, addr: UserConstPtr<sockaddr>, addr_len: u32) -> LinuxResult<isize> {
    info!("sys_bind called with fd: {}, addr_len: {}", fd, addr_len);

    // 获取文件描述符对应的 socket
    let socket = get_file_like(fd as i32)?;
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
    let socket_addr = SocketAddr::read_from_user(addr, addr_len as socklen_t)?;
    info!("sys_bind converted to SocketAddr: {}", socket_addr);

    // 调用 socket 的 bind 方法
    socket.bind(socket_addr)?;

    info!("sys_bind successfully bound fd {}", fd);
    Ok(0)
}
pub fn sys_connect(fd: isize, addr: UserConstPtr<sockaddr>, addr_len: u32) -> LinuxResult<isize> {
    info!("sys_connect called with fd: {}, addr_len: {}", fd, addr_len);

    // 获取文件描述符对应的 socket
    let socket = get_file_like(fd as i32)?;
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
    let socket_addr = SocketAddr::read_from_user(addr, addr_len as socklen_t)?;
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

pub fn sys_setsockopt(
    fd: isize,
    level: u32,
    optname: u32,
    optval: UserConstPtr<u32>,
    optlen: u32,
) -> LinuxResult<isize> {
    info!(
        "sys_setsockopt called with fd: {}, level: {}, optname: {}, optlen: {}",
        fd, level, optname, optlen
    );

    // 获取文件描述符对应的 socket
    let socket = get_file_like(fd as i32)?;
    info!("sys_setsockopt found socket for fd {}", fd);

    // 检查是否为 Socket 类型
    let any_socket = socket.into_any();
    if !any_socket.is::<crate::file::Socket>() {
        error!("Object is NOT Socket type");
        return Err(LinuxError::ENOTSOCK);
    }

    // 转换为 Socket
    let socket = any_socket
        .downcast::<crate::file::Socket>()
        .map_err(|_| {
            error!("Failed to downcast to Socket for fd {}", fd);
            LinuxError::ENOTSOCK
        })?;

    // 处理不同的选项
    match (level, optname) {
        // SOL_SOCKET 层的选项
        (linux_raw_sys::net::SOL_SOCKET, linux_raw_sys::net::SO_REUSEADDR) => {
            if optlen < core::mem::size_of::<i32>() as u32 {
                return Err(LinuxError::EINVAL);
            }
            unimplemented!("SO_REUSEADDR option is not implemented yet");
            // // 从用户空间读取 int 值
            // let mut value = 0i32;
            // optval.read_to_slice(
            //     core::slice::from_raw_parts_mut(
            //         &mut value as *mut i32 as *mut u8, 
            //         core::mem::size_of::<i32>()
            //     )
            // )?;
            
            // // 设置 SO_REUSEADDR 选项
            // socket.set_reuse_addr(value != 0)?;
            // info!("Set SO_REUSEADDR to {}", value != 0);
        }
        
        // TCP 层的选项
        (val, linux_raw_sys::net::TCP_NODELAY) if val == linux_raw_sys::net::IPPROTO_TCP as u32 => {
            if optlen < core::mem::size_of::<i32>() as u32 {
                return Err(LinuxError::EINVAL);
            }
            let value = optval.get_as_ref()?;
            
            let enable_nagle = *value == 0;
            info!("Setting TCP_NODELAY to {}, enable_nagle={}", value, enable_nagle);
            
            // 根据socket类型设置Nagle算法
            match socket.as_ref() {
                crate::file::Socket::Tcp(tcp_socket) => {
                    let socket = tcp_socket.lock();
                    socket.set_nagle_enabled(enable_nagle);
                    info!("Nagle algorithm is now {}", if enable_nagle { "enabled" } else { "disabled" });
                }
                _ => {
                    error!("TCP_NODELAY only applicable to TCP sockets");
                    return Err(LinuxError::ENOPROTOOPT);
                }
            }
            Ok(0)
        }

        // SO_RCVBUF: 设置接收缓冲区大小
        (linux_raw_sys::net::SOL_SOCKET, linux_raw_sys::net::SO_RCVBUF) => {
            if optlen < core::mem::size_of::<i32>() as u32 {
                return Err(LinuxError::EINVAL);
            }
            unimplemented!("SO_RCVBUF option is not implemented yet");
            // let mut value = 0i32;
            // optval.read_to_slice(
            //     core::slice::from_raw_parts_mut(
            //         &mut value as *mut i32 as *mut u8, 
            //         core::mem::size_of::<i32>()
            //     )
            // )?;
            
            // // 忽略但记录接收缓冲区大小设置请求
            // info!("Ignoring SO_RCVBUF set to {} bytes", value);
        }
        
        // SO_SNDBUF: 设置发送缓冲区大小
        (linux_raw_sys::net::SOL_SOCKET, linux_raw_sys::net::SO_SNDBUF) => {
            unimplemented!("SO_SNDBUF option is not implemented yet");
            // if optlen < core::mem::size_of::<i32>() as u32 {
            //     return Err(LinuxError::EINVAL);
            // }
            
            // let mut value = 0i32;
            // optval.read_to_slice(
            //     core::slice::from_raw_parts_mut(
            //         &mut value as *mut i32 as *mut u8, 
            //         core::mem::size_of::<i32>()
            //     )
            // )?;
            
            // // 忽略但记录发送缓冲区大小设置请求
            // info!("Ignoring SO_SNDBUF set to {} bytes", value);
        }
        
        // 其他未实现的选项
        _ => {
            info!("Unsupported socket option: level {}, optname {}", level, optname);
            Err(LinuxError::ENOPROTOOPT)
        }
    }
}

pub fn sys_getsockopt(
    fd: isize,
    level: u32,
    optname: u32,
    optval: UserPtr<u32>,
    optlen: UserPtr<socklen_t>,
) -> LinuxResult<isize> {
    info!(
        "sys_getsockopt called with fd: {}, level: {}, optname: {}",
        fd, level, optname
    );

    // 获取文件描述符对应的 socket
    let socket = get_file_like(fd as i32)?;
    info!("sys_getsockopt found socket for fd {}", fd);

    // 检查是否为 Socket 类型
    let any_socket = socket.into_any();
    if !any_socket.is::<crate::file::Socket>() {
        error!("Object is NOT Socket type");
        return Err(LinuxError::ENOTSOCK);
    }

    // 转换为 Socket
    let socket = any_socket
        .downcast::<crate::file::Socket>()
        .map_err(|_| {
            error!("Failed to downcast to Socket for fd {}", fd);
            LinuxError::ENOTSOCK
        })?;

    // 处理不同的选项
    match (level, optname) {
        // SOL_SOCKET 层的选项
        (linux_raw_sys::net::SOL_SOCKET, linux_raw_sys::net::SO_SNDBUF) => {
            let len = optlen.get_as_mut()?;
            if *len < core::mem::size_of::<i32>() as socklen_t {
                return Err(LinuxError::EINVAL);
            }

            let buf_size = match socket.as_ref() {
                crate::file::Socket::Tcp(tcp_socket) => {
                    info!("Getting SO_SNDBUF for TCP socket");
                    let socket = tcp_socket.lock();
                    socket.send_capacity() 
                }
                crate::file::Socket::Udp(udp_socket) => {
                    info!("Getting SO_SNDBUF for UDP socket");
                    let socket = udp_socket.lock();
                    socket.send_capacity() 
                }
            };

            let buffer = optval.get_as_mut()?;
            *buffer = buf_size as u32;
            let buffer_len = optlen.get_as_mut()?;
            *buffer_len = core::mem::size_of::<i32>() as socklen_t;
            info!("SO_SNDBUF returning buffer size: {} bytes", buf_size);
            Ok(0)
        }
        (linux_raw_sys::net::SOL_SOCKET, linux_raw_sys::net::SO_RCVBUF) => {
                let len = optlen.get_as_mut()?;
                if *len < core::mem::size_of::<i32>() as socklen_t {
                    return Err(LinuxError::EINVAL);
                }

                let buf_size = match socket.as_ref() {
                    crate::file::Socket::Tcp(tcp_socket) => {
                        let socket = tcp_socket.lock();
                        socket.recv_capacity() 
                    }
                    crate::file::Socket::Udp(udp_socket) => {
                        let socket = udp_socket.lock();
                        socket.recv_capacity() 
                    }
                };

                let buffer = optval.get_as_mut()?;
                *buffer = buf_size as u32;
                let buffer_len = optlen.get_as_mut()?;
                *buffer_len = core::mem::size_of::<i32>() as socklen_t;
                info!("SO_RCVBUF returning buffer size: {} bytes", buf_size);
                Ok(0)
        }

        (val, linux_raw_sys::net::TCP_MAXSEG) if val == linux_raw_sys::net::IPPROTO_TCP as u32 => {
            let len = optlen.get_as_mut()?;
            if *len < core::mem::size_of::<i32>() as socklen_t {
                return Err(LinuxError::EINVAL);
            }

            let mss_size = match socket.as_ref() {
                crate::file::Socket::Tcp(tcp_socket) => {
                    let socket = tcp_socket.lock();
                    socket.get_remote_mss() 
                }
                crate::file::Socket::Udp(udp_socket) => {
                    return Err(LinuxError::ENOPROTOOPT);
                }
            };

            let buffer = optval.get_as_mut()?;
            *buffer = mss_size as u32;
            let buffer_len = optlen.get_as_mut()?;
            *buffer_len = core::mem::size_of::<i32>() as socklen_t;
            info!("TCP_MAXSEG returning buffer size: {} bytes", mss_size);
            Ok(0)
            
        }
        // 其他未实现的选项
        _ => {
            info!("Unsupported socket option: level {}, optname {}", level, optname);
            Err(LinuxError::ENOPROTOOPT)
        }
    }
}