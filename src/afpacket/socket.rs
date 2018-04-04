use libc::{
    c_void, getsockopt, setsockopt, sockaddr, sockaddr_ll, sockaddr_storage, socket, socklen_t,
};
use libc::{AF_PACKET, ETH_P_ALL, SOCK_RAW, SOL_PACKET};
use std::io::{Error, ErrorKind, Result};
use std::mem::size_of;
use std::os::unix::io::RawFd;

pub struct RawSocket {
    fd: RawFd,
}

fn ifindex_by_name(name: &str) -> Result<i32> {
    if name.len() > libc::IFNAMSIZ {
        return Err(ErrorKind::InvalidInput.into());
    }
    let mut buf = [0u8; libc::IFNAMSIZ];
    buf[..name.len()].copy_from_slice(name.as_bytes());
    match unsafe { libc::if_nametoindex(buf.as_ptr() as *const libc::c_char) } {
        0 => Err(Error::last_os_error()),
        ifindex @ _ => Ok(ifindex as i32),
    }
}

impl RawSocket {
    pub fn new() -> Result<RawSocket> {
        match unsafe { socket(AF_PACKET, SOCK_RAW, (ETH_P_ALL as u16).to_be() as i32) } {
            -1 => Err(Error::last_os_error()),
            fd @ _ => Ok(RawSocket { fd }),
        }
    }

    pub fn close(&mut self) -> Result<()> {
        if self.fd != -1 {
            let ret = unsafe { libc::close(self.fd) };
            if ret == -1 {
                return Err(Error::last_os_error());
            }
            self.fd = -1;
        }
        Ok(())
    }

    pub fn fd(&self) -> RawFd {
        self.fd
    }

    pub fn getsockopt<T: Default>(&self, opt: i32) -> Result<T> {
        let mut t = T::default();
        let t_ptr = (&mut t as *mut T) as *mut c_void;
        let mut len: u32 = 0;
        let len_ptr = &mut len as *mut socklen_t;
        match unsafe { getsockopt(self.fd, SOL_PACKET, opt, t_ptr, len_ptr) } {
            -1 => Err(Error::last_os_error()),
            _ => Ok(t),
        }
    }

    pub fn setsockopt<T>(&self, opt: i32, val: &T) -> Result<()> {
        let val_ptr = (val as *const T) as *const c_void;
        match unsafe { setsockopt(self.fd, SOL_PACKET, opt, val_ptr, size_of::<T>() as u32) } {
            -1 => Err(Error::last_os_error()),
            _ => Ok(()),
        }
    }

    pub fn set_nonblocking(&self) -> Result<()> {
        let mut nonblocking = 1 as libc::c_ulong;
        match unsafe { libc::ioctl(self.fd, libc::FIONBIO, &mut nonblocking) } {
            -1 => Err(Error::last_os_error()),
            _ => Ok(()),
        }
    }

    pub fn bind_interface(&self, ifname: &str) -> Result<()> {
        let ifindex = ifindex_by_name(ifname)?;
        match unsafe {
            let mut ss: sockaddr_storage = std::mem::zeroed();
            let sll: *mut sockaddr_ll = std::mem::transmute(&mut ss);
            (*sll).sll_family = AF_PACKET as u16;
            (*sll).sll_protocol = (ETH_P_ALL as u16).to_be();
            (*sll).sll_ifindex = ifindex;

            let sa = (&ss as *const sockaddr_storage) as *const sockaddr;
            libc::bind(self.fd, sa, size_of::<sockaddr_ll>() as u32)
        } {
            -1 => Err(Error::last_os_error()),
            _ => Ok(()),
        }
    }
}
