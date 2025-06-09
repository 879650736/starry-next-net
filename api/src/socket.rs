//! 安全地处理 socket 地址的工具函数，使用 UserPtr 与用户空间交互。

use core::{
    mem::{size_of, MaybeUninit},
    net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
};

use axerrno::{LinuxError, LinuxResult};
use linux_raw_sys::net::{
    __kernel_sa_family_t, AF_INET, AF_INET6, in_addr, in6_addr, sockaddr, sockaddr_in,
    sockaddr_in6, socklen_t,
};
use crate::ptr::{UserPtr, UserConstPtr};

/// 为 socket 地址类型提供与用户空间交互的扩展 trait
pub trait SocketAddrExt: Sized {
    /// 从用户空间读取 socket 地址
    fn read_from_user(addr: UserConstPtr<sockaddr>, addrlen: socklen_t) -> LinuxResult<Self>;

    /// 将 socket 地址写入用户空间
    fn write_to_user(&self, addr: UserPtr<sockaddr>) -> LinuxResult<socklen_t>;

    /// 获取地址族
    fn family(&self) -> u16;

    /// 获取编码后的地址长度
    fn addr_len(&self) -> socklen_t;
}

impl SocketAddrExt for SocketAddr {
     fn read_from_user(addr: UserConstPtr<sockaddr>, addrlen: socklen_t) -> LinuxResult<Self> {
        if size_of::<__kernel_sa_family_t>() > addrlen as usize || addrlen as usize > size_of::<sockaddr>() {
            return Err(LinuxError::EINVAL);
        }

        let mut storage = MaybeUninit::<sockaddr>::uninit();
        let sock_addr= addr.get_as_ref()?;
        unsafe {
            core::ptr::copy_nonoverlapping(
                sock_addr as *const sockaddr as *const u8,
                storage.as_mut_ptr() as *mut u8,
                addrlen as usize,
            )
        };
        let family = unsafe { storage.assume_init_ref().__storage.__bindgen_anon_1.__bindgen_anon_1.ss_family as u32 };
        
        match family {
            AF_INET => SocketAddrV4::read_from_user(addr, addrlen).map(SocketAddr::V4),
            AF_INET6 => SocketAddrV6::read_from_user(addr, addrlen).map(SocketAddr::V6),
            _ => Err(LinuxError::EAFNOSUPPORT),
        }
    }

    fn write_to_user(&self, addr: UserPtr<sockaddr>) -> LinuxResult<socklen_t> {
        if addr.is_null() {
            return Err(LinuxError::EINVAL);
        }

        match self {
            SocketAddr::V4(v4) => v4.write_to_user(addr),
            SocketAddr::V6(v6) => v6.write_to_user(addr),
        }
    }

    fn family(&self) -> u16 {
        match self {
            SocketAddr::V4(_) => AF_INET as u16,
            SocketAddr::V6(_) => AF_INET6 as u16,
        }
    }

    fn addr_len(&self) -> socklen_t {
        match self {
            SocketAddr::V4(_) => size_of::<sockaddr_in>() as socklen_t,
            SocketAddr::V6(_) => size_of::<sockaddr_in6>() as socklen_t,
        }
    }
}

impl SocketAddrExt for SocketAddrV4 {
   fn read_from_user(addr: UserConstPtr<sockaddr>, addrlen: socklen_t) -> LinuxResult<Self> {
        if addrlen < size_of::<sockaddr_in>() as socklen_t {
            return Err(LinuxError::EINVAL);
        }

        let mut storage = MaybeUninit::<sockaddr>::uninit();
        let sock_addr= addr.get_as_ref()?;
        unsafe {
            core::ptr::copy_nonoverlapping(
                sock_addr as *const sockaddr as *const u8,
                storage.as_mut_ptr() as *mut u8,
                addrlen as usize,
            )
        };
        
        let addr_in = unsafe { &*(storage.as_ptr() as *const sockaddr_in) };
        if addr_in.sin_family as u32 != AF_INET {
            return Err(LinuxError::EAFNOSUPPORT);
        }
        
        Ok(SocketAddrV4::new(
            Ipv4Addr::from_bits(u32::from_be(addr_in.sin_addr.s_addr)),
            u16::from_be(addr_in.sin_port),
        ))
    }

    fn write_to_user(&self, addr: UserPtr<sockaddr>) -> LinuxResult<socklen_t> {
        if addr.is_null() {
            return Err(LinuxError::EINVAL);
        }

        let len = size_of::<sockaddr_in>() as socklen_t;
        let sockin_addr = sockaddr_in {
            sin_family: AF_INET as _,
            sin_port: self.port().to_be(),
            sin_addr: in_addr {
                s_addr: u32::from_ne_bytes(self.ip().octets()),
            },
            __pad: [0_u8; 8],
        };
        let mut storage = MaybeUninit::<sockaddr>::uninit();
        unsafe {
            core::ptr::copy_nonoverlapping(
                &sockin_addr as *const sockaddr_in as *const u8,
                storage.as_mut_ptr() as *mut u8,
                len as usize,
            )
        };
        
        
        Ok(len)
    }

    fn family(&self) -> u16 {
        AF_INET as u16
    }

    fn addr_len(&self) -> socklen_t {
        size_of::<sockaddr_in>() as socklen_t
    }
}

impl SocketAddrExt for SocketAddrV6 {
    fn read_from_user(addr: UserConstPtr<sockaddr>, addrlen: socklen_t) -> LinuxResult<Self> {
        if addrlen < size_of::<sockaddr_in6>() as socklen_t {
            return Err(LinuxError::EINVAL);
        }

        let mut storage = MaybeUninit::<sockaddr>::uninit();
        let sock_addr= addr.get_as_ref()?;
        unsafe {
            core::ptr::copy_nonoverlapping(
                sock_addr as *const sockaddr as *const u8,
                storage.as_mut_ptr() as *mut u8,
                addrlen as usize,
            )
        };
        
        let addr_in6 = unsafe { &*(storage.as_ptr() as *const sockaddr_in6) };
        if addr_in6.sin6_family as u32 != AF_INET6 {
            return Err(LinuxError::EAFNOSUPPORT);
        }
        
        Ok(SocketAddrV6::new(
            Ipv6Addr::from(unsafe { addr_in6.sin6_addr.in6_u.u6_addr8 }),
            u16::from_be(addr_in6.sin6_port),
            u32::from_be(addr_in6.sin6_flowinfo),
            addr_in6.sin6_scope_id,
        ))
    }

    fn write_to_user(&self, addr: UserPtr<sockaddr>) -> LinuxResult<socklen_t> {
        if addr.is_null() {
            return Err(LinuxError::EINVAL);
        }

        let len = size_of::<sockaddr_in6>() as socklen_t;
        let sockin_addr = sockaddr_in6 {
            sin6_family: AF_INET6 as _,
            sin6_port: self.port().to_be(),
            sin6_flowinfo: self.flowinfo().to_be(),
            sin6_addr: in6_addr {
                in6_u: linux_raw_sys::net::in6_addr__bindgen_ty_1 {
                    u6_addr8: self.ip().octets(),
                },
            },
            sin6_scope_id: self.scope_id(),
        };
        
        let mut storage = MaybeUninit::<sockaddr>::uninit();
        unsafe {
            core::ptr::copy_nonoverlapping(
                &sockin_addr as *const sockaddr_in6 as *const u8,
                storage.as_mut_ptr() as *mut u8,
                len as usize,
            )
        };
        
        Ok(len)
    }

    fn family(&self) -> u16 {
        AF_INET6 as u16
    }

    fn addr_len(&self) -> socklen_t {
        size_of::<sockaddr_in6>() as socklen_t
    }
}

