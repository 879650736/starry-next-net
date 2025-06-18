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

//use crate::ctypes;

const FD_SETSIZE: usize = 1024;
const BITS_PER_USIZE: usize = usize::BITS as usize;
const FD_SETSIZE_USIZES: usize = FD_SETSIZE.div_ceil(BITS_PER_USIZE);

struct FdSets {
    nfds: usize,
    bits: [usize; FD_SETSIZE_USIZES * 3],
}

impl FdSets {
    fn from(
        nfds: usize,
        read_fds: *const __kernel_fd_set,
        write_fds: *const __kernel_fd_set,
        except_fds: *const __kernel_fd_set,
    ) -> Self {
        let nfds = nfds.min(FD_SETSIZE);
        let nfds_usizes = nfds.div_ceil(BITS_PER_USIZE);
        let mut bits = [0usize; FD_SETSIZE_USIZES * 3];
        //let mut bits = core::mem::MaybeUninit::<[usize; FD_SETSIZE_USIZES * 3]>::uninit();
        let bits_ptr: *mut usize = unsafe { core::mem::transmute(bits.as_mut_ptr()) };

        let copy_from_fd_set = |dst: &mut [usize], fds: *const __kernel_fd_set| unsafe {
            if !fds.is_null()  {
                let fds_ptr = (*fds).fds_bits.as_ptr() as *const usize;
                let src = core::slice::from_raw_parts(fds_ptr, nfds_usizes);
                dst[..nfds_usizes].copy_from_slice(src);
            }
        };

         // 复制位到各个部分
        copy_from_fd_set(&mut bits[0..FD_SETSIZE_USIZES], read_fds);
        copy_from_fd_set(&mut bits[FD_SETSIZE_USIZES..2*FD_SETSIZE_USIZES], write_fds);
        copy_from_fd_set(&mut bits[2*FD_SETSIZE_USIZES..3*FD_SETSIZE_USIZES], except_fds);
        Self { nfds, bits }
    }

    fn poll_all(
        &self,
        res_read_fds: *mut __kernel_fd_set,
        res_write_fds: *mut __kernel_fd_set,
        res_except_fds: *mut __kernel_fd_set,
    ) -> LinuxResult<usize> {
        let mut read_bits_ptr = self.bits.as_ptr();
        let mut write_bits_ptr = unsafe { read_bits_ptr.add(FD_SETSIZE_USIZES) };
        let mut execpt_bits_ptr = unsafe { read_bits_ptr.add(FD_SETSIZE_USIZES * 2) };
        let mut i = 0;
        let mut res_num = 0;
        while i < self.nfds {
            let read_bits = unsafe { *read_bits_ptr };
            let write_bits = unsafe { *write_bits_ptr };
            let except_bits = unsafe { *execpt_bits_ptr };
            info!(
                "polling bits: read: {:b}, write: {:b}, except: {:b}",
                read_bits, write_bits, except_bits
            );
            unsafe {
                read_bits_ptr = read_bits_ptr.add(1);
                write_bits_ptr = write_bits_ptr.add(1);
                execpt_bits_ptr = execpt_bits_ptr.add(1);
            }

            let all_bits = read_bits | write_bits | except_bits;
            if all_bits == 0 {
                i += BITS_PER_USIZE;
                continue;
            }
            let mut j = 0;
            while j < BITS_PER_USIZE && i + j < self.nfds {
                let bit = 1 << j;
                if all_bits & bit == 0 {
                    j += 1;
                    continue;
                }
                let fd = i + j;
                match get_file_like(fd as _)?.poll() {
                    Ok(state) => {
                        debug!("    fd: {}, state: {:?}", fd, state);
                        if state.readable && read_bits & bit != 0 {
                            debug!("    readable: {}", fd);
                            unsafe { set_fd_set(res_read_fds, fd) };
                            res_num += 1;
                        }
                        if state.writable && write_bits & bit != 0 {
                            debug!("    writable: {}", fd);
                            unsafe { set_fd_set(res_write_fds, fd) };
                            res_num += 1;
                        }
                    }
                    Err(e) => {
                        debug!("    except: {} {:?}", fd, e);
                        if except_bits & bit != 0 {
                            unsafe { set_fd_set(res_except_fds, fd) };
                            res_num += 1;
                        }
                    }
                }
                j += 1;
            }
            i += BITS_PER_USIZE;
        }
        Ok(res_num)
    }
}

/// Monitor multiple file descriptors, waiting until one or more of the file descriptors become "ready" for some class of I/O operation
pub unsafe fn sys_select1(
    nfds: c_int,
    readfds: *mut __kernel_fd_set,
    writefds: *mut __kernel_fd_set,
    exceptfds: *mut __kernel_fd_set,
    timeout: *mut timeval,
) -> LinuxResult<c_int> {
    info!(
        "sys_select1 called with nfds: {}, readfds: {:?}, writefds: {:?}, exceptfds: {:?}, timeout: {:?}",
        nfds, *readfds, *writefds, *exceptfds, *timeout
    );
    if nfds < 0 {
        return Err(LinuxError::EINVAL);
    }
    let nfds = (nfds as usize).min(FD_SETSIZE);
    let deadline = unsafe {
        timeout
            .as_ref()
            .map(|t| wall_time() + Duration::new(t.tv_sec as u64, t.tv_usec as u32 * 1000))
    };
    let fd_sets = FdSets::from(nfds, readfds, writefds, exceptfds);

    unsafe {
        zero_fd_set(readfds, nfds);
        zero_fd_set(writefds, nfds);
        zero_fd_set(exceptfds, nfds);
    }
    info!("readfds: {:?}, writefds: {:?}, exceptfds: {:?}", *readfds, *writefds, *exceptfds);
    loop {
        axnet::poll_interfaces();
        let res = fd_sets.poll_all(readfds, writefds, exceptfds)?;
        debug!("    res: {}", res);
        if res > 0 {
            info!("readfds: {:?}, writefds: {:?}, exceptfds: {:?}", *readfds, *writefds, *exceptfds);
            return Ok(res as i32);
        }
        info!("deadline = {:?}, wall_time() = {:?}", deadline, wall_time());
        if deadline.is_some_and(|ddl| wall_time() >= ddl) {
            debug!("    timeout!");
            return Ok(0);
        }
        crate::sys_sched_yield();
    }
}

unsafe fn zero_fd_set(fds: *mut __kernel_fd_set, nfds: usize) {
    if !fds.is_null() {
        let nfds_usizes = nfds.div_ceil(BITS_PER_USIZE);
        for i in 0..nfds_usizes {
            (*fds).fds_bits[i] = 0;
        }
    }
}

unsafe fn set_fd_set(fds: *mut __kernel_fd_set, fd: usize) {
    if !fds.is_null() {
        let fd_set = &mut *fds;  // 正确地解引用指针，获取可变引用
        fd_set.fds_bits[fd / BITS_PER_USIZE] |= 1 << (fd % BITS_PER_USIZE);
    }
}

pub fn sys_select(
    nfds: i32,
    readfds: UserPtr<__kernel_fd_set>,
    writefds: UserPtr<__kernel_fd_set>,
    exceptfds: UserPtr<__kernel_fd_set>,
    timeout: UserPtr<timeval>,
) -> LinuxResult<isize> {
    info!("sys_select called with nfds: {}", nfds);
    let mut readfds_local = __kernel_fd_set { fds_bits: [0; 16] };
    let readfds1: &mut __kernel_fd_set = if !readfds.is_null() {
        readfds.get_as_mut()?
    } else {
        &mut readfds_local
    };
    debug!("readfds: {:?}", readfds1);

    let mut writefds_local = __kernel_fd_set { fds_bits: [0; 16] };
    let writefds1: &mut __kernel_fd_set = if !writefds.is_null() {
        writefds.get_as_mut()?
    } else {
        &mut writefds_local
    };
    debug!("writefds: {:?}", writefds1);

    let mut exceptfds_local = __kernel_fd_set { fds_bits: [0; 16] };
    let exceptfds1: &mut __kernel_fd_set = if !exceptfds.is_null() {
        exceptfds.get_as_mut()?
    } else {
        &mut exceptfds_local
    };
    debug!("exceptfds: {:?}", exceptfds1);

    let mut timeout_local = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let timeout1: &mut timeval = if !timeout.is_null() {
        timeout.get_as_mut()?
    } else {
        &mut timeout_local
    };
    debug!("timeout: {:?}", timeout1);
    let ret: i32;
    unsafe {
        ret = sys_select1(nfds, readfds1, writefds1, exceptfds1, timeout1)?;
    }

    Ok(ret as isize)
}

pub fn sys_pselect6(
    nfds: i32,
    readfds: UserPtr<__kernel_fd_set>,
    writefds: UserPtr<__kernel_fd_set>,
    exceptfds: UserPtr<__kernel_fd_set>,
    timeout: UserPtr<timeval>,
    sigmask: UserConstPtr<u8>,
) -> LinuxResult<isize> {
    info!("sys_pselect6 called with nfds: {}", nfds);
    let sigmask1 = sigmask.get_as_ref()?;
    let ret = sys_select(nfds, readfds, writefds, exceptfds, timeout)?;

    Ok(ret)
}
