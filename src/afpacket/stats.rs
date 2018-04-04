use libc::c_uint;

pub struct Statistics {
    pub packets: u32,
    pub drops: u32,
    pub queue_freezes: u32,
}

pub(super) struct TPacketV3Stats {
    pub tp_packets: c_uint,
    pub tp_drops: c_uint,
    pub tp_freeze_q_cnt: c_uint,
}

impl Default for TPacketV3Stats {
    fn default() -> TPacketV3Stats {
        unsafe { std::mem::zeroed() }
    }
}

impl From<TPacketV3Stats> for Statistics {
    fn from(stats: TPacketV3Stats) -> Statistics {
        Statistics {
            packets: stats.tp_packets,
            drops: stats.tp_drops,
            queue_freezes: stats.tp_freeze_q_cnt,
        }
    }
}

pub(super) struct TPacketStats {
    pub tp_packets: c_uint,
    pub tp_drops: c_uint,
}

impl Default for TPacketStats {
    fn default() -> TPacketStats {
        unsafe { std::mem::zeroed() }
    }
}

impl From<TPacketStats> for Statistics {
    fn from(stats: TPacketStats) -> Statistics {
        Statistics {
            packets: stats.tp_packets,
            drops: stats.tp_drops,
            queue_freezes: 0,
        }
    }
}
