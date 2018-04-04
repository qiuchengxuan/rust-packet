use libc::c_int;
use std::io::Result;
use std::time::Duration;

use afpacket::ring::*;
use afpacket::socket::RawSocket;
use afpacket::stats::*;

const PACKET_RX_RING: c_int = 0x5;
const PACKET_STATISTICS: c_int = 0x6;
const PACKET_VERSION: c_int = 0xa;
const PACKET_TX_RING: c_int = 0xd;

#[derive(Copy, Clone, PartialEq)]
pub enum TPacketVersion {
    V1,
    V2,
    V3,
}

pub enum RingDirection {
    RX = PACKET_RX_RING as isize,
    TX = PACKET_TX_RING as isize,
}

pub struct TPacket {
    socket: RawSocket,
    version: TPacketVersion,
    timeout: Duration,
    mmap: *mut u8,
}

impl TPacket {
    pub fn new() -> Result<TPacket> {
        match { RawSocket::new() } {
            Err(err) => Err(err),
            Ok(socket) => Ok(TPacket {
                socket: socket,
                version: TPacketVersion::V1,
                timeout: DEFAULT_BLOCK_TIMEOUT,
                mmap: std::ptr::null_mut(),
            }),
        }
    }

    pub fn setup_ring(&mut self, direction: RingDirection, options: &RingOptions) -> Result<()> {
        if self.version == TPacketVersion::V3 {
            let req: TPacketReq3 = options.into();
            self.socket
                .setsockopt(direction as i32, &req)
                .and_then({ |_| mmap_rx_ring(self.socket.fd(), &options) })
                .and_then(|mmap| {
                    self.mmap = mmap;
                    Ok(())
                })
                .and_then(|_| self.get_statistics())
                .and_then(|_| Ok(()))
        } else {
            let req: TPacketReq = options.into();
            self.socket.setsockopt(direction as i32, &req)
        }
    }

    pub fn set_version(&mut self, version: TPacketVersion) -> Result<TPacketVersion> {
        let ver = version as i32;
        self.socket.setsockopt(PACKET_VERSION, &ver).and_then(|_| {
            self.version = version;
            Ok(version)
        })
    }

    pub fn set_highest_version(&mut self) -> Result<TPacketVersion> {
        self.set_version(TPacketVersion::V3)
            .or_else(|_| self.set_version(TPacketVersion::V2))
            .or_else(|_| self.set_version(TPacketVersion::V1))
    }

    /// once called, statistics are cleared by linux kernel
    pub fn get_statistics(&self) -> Result<Statistics> {
        if self.version == TPacketVersion::V3 {
            let result: Result<TPacketV3Stats> = self.socket.getsockopt(PACKET_STATISTICS);
            return result.and_then(|s| Ok(s.into()));
        } else {
            let result: Result<TPacketStats> = self.socket.getsockopt(PACKET_STATISTICS);
            return result.and_then(|s| Ok(s.into()));
        }
    }
}
