use libc::{c_uint, mmap, MAP_SHARED, PROT_READ, PROT_WRITE};
use std::io::{Error, Result};
use std::os::unix::io::RawFd;
use std::ptr::null_mut;
use std::time::Duration;

pub const DEFAULT_BLOCK_TIMEOUT: Duration = Duration::from_millis(64);

pub struct RingOptions {
    pub frame_size: u32,
    pub num_of_frames: u32,
    pub block_size: u32,
    pub num_of_blocks: u32,
    pub retire_block_timeout: Option<Duration>,
}

impl Default for RingOptions {
    fn default() -> RingOptions {
        RingOptions {
            frame_size: 2048,
            num_of_frames: 160000,
            block_size: 2048 * 128,
            num_of_blocks: 128,
            retire_block_timeout: Some(DEFAULT_BLOCK_TIMEOUT),
        }
    }
}

#[repr(C)]
pub(super) struct TPacketReq {
    pub block_size: c_uint,
    pub num_of_blocks: c_uint,
    pub frame_size: c_uint,
    pub num_of_frames: c_uint,
}

impl From<&RingOptions> for TPacketReq {
    fn from(options: &RingOptions) -> TPacketReq {
        TPacketReq {
            block_size: options.block_size,
            num_of_blocks: options.num_of_blocks,
            frame_size: options.frame_size,
            num_of_frames: options.num_of_frames,
        }
    }
}

#[repr(C)]
pub(super) struct TPacketReq3 {
    pub frame_size: libc::c_uint,
    pub num_of_frames: libc::c_uint,
    pub block_size: libc::c_uint,
    pub num_of_blocks: libc::c_uint,
    pub retire_block_timeout: libc::c_uint, // in millsecond
    pub sizeof_private: libc::c_uint,
    pub feature_req_word: libc::c_uint,
}

impl From<&RingOptions> for TPacketReq3 {
    fn from(options: &RingOptions) -> TPacketReq3 {
        TPacketReq3 {
            block_size: options.block_size,
            num_of_blocks: options.num_of_blocks,
            frame_size: options.frame_size,
            num_of_frames: options.num_of_frames,
            retire_block_timeout: options
                .retire_block_timeout
                .unwrap_or(DEFAULT_BLOCK_TIMEOUT)
                .subsec_millis(),
            sizeof_private: 0,
            feature_req_word: 0,
        }
    }
}

pub fn mmap_rx_ring(fd: RawFd, req: &RingOptions) -> Result<*mut u8> {
    let protection = PROT_READ | PROT_WRITE;
    let total_size = (req.block_size * req.num_of_blocks) as usize;
    match unsafe { mmap(null_mut(), total_size, protection, MAP_SHARED, fd, 0) as isize } {
        -1 => Err(Error::last_os_error()),
        map => Ok(map as *mut u8),
    }
}

macro_rules! le {
    ($x:expr) => {
        u32::from_be($x)
    };
}

trait TPacketHeader {
    fn timestamp(&self) -> Duration;
}

#[repr(C)]
struct TPacketV3Header {
    tp_next_offset: u32,
    tp_sec: u32,
    tp_nsec: u32,
    tp_snaplen: u32,
    tp_len: u32,
    tp_status: u32,
    tp_mac: u16,
    tp_net: u16,
}

struct TPacketV3HeaderParser<'a> {
    raw: &'a TPacketV3Header,
}

impl<'a> TPacketV3HeaderParser<'a> {
    fn from_raw(raw_header: &'a [u8]) -> TPacketV3HeaderParser {
        TPacketV3HeaderParser {
            raw: unsafe { std::mem::transmute(&raw_header) },
        }
    }
}

impl<'a> TPacketHeader for TPacketV3HeaderParser<'a> {
    fn timestamp(&self) -> Duration {
        Duration::new(le!(self.raw.tp_sec) as u64, le!(self.raw.tp_nsec))
    }
}
