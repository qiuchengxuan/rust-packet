use num_traits::FromPrimitive;

use eth::macaddr::{MacAddr, MAC_ADDR_LEN};

// use header::Header;

pub const ETH_TYPE_LEN: usize = 2;

pub const SOURCE_MAC_OFFSET: usize = MAC_ADDR_LEN;
pub const ETH_TYPE_OFFSET: usize = MAC_ADDR_LEN * 2;

pub const ETH_HEADER_MIN_SIZE: usize = MAC_ADDR_LEN * 2 + ETH_TYPE_LEN;

#[derive(Debug, PartialEq, Primitive)]
pub enum EthernetType {
    Unknown = 0x0,
    Ipv4 = 0x0800,
    Arp = 0x0806,
    Dot1qVlanTag = 0x8100,
    QinQVlanTag = 0x9100,
}

impl<'a> From<&'a [u8]> for EthernetType {
    fn from(bytes: &'a [u8]) -> EthernetType {
        let value = (bytes[0] as u16) << 8u16 | bytes[1] as u16;
        match EthernetType::from_u16(value) {
            Some(x) => x,
            None => EthernetType::Unknown,
        }
    }
}

pub struct EthernetHeader<'a> {
    raw: &'a [u8],
}

impl<'a> EthernetHeader<'a> {
    pub fn new(raw: &'a [u8]) -> Option<Self> {
        if raw.len() < ETH_HEADER_MIN_SIZE {
            return None;
        }
        Some(EthernetHeader { raw })
    }

    pub fn dst(&self) -> MacAddr {
        self.raw.into()
    }

    pub fn src(&self) -> MacAddr {
        self.raw[SOURCE_MAC_OFFSET..ETH_TYPE_OFFSET].into()
    }

    pub fn eth_type(&self) -> EthernetType {
        self.raw[ETH_TYPE_OFFSET..ETH_TYPE_OFFSET + ETH_TYPE_LEN].into()
    }
}

#[cfg(test)]
mod tests {
    use byteorder::{BigEndian, ByteOrder};
    use eth::ethernet::*;
    use hwaddr::HwAddr;
    use std::str::FromStr;

    #[test]
    fn test_works() {
        let da = HwAddr::from_str("01:80:c2:00:00:00").unwrap();
        let sa = HwAddr::from_str("00:00:00:00:00:01").unwrap();
        let mut header = [0u8; 14];
        header[..MAC_ADDR_LEN].clone_from_slice(&da.octets());
        header[SOURCE_MAC_OFFSET..ETH_TYPE_OFFSET].clone_from_slice(&sa.octets());
        BigEndian::write_u16(&mut header[ETH_TYPE_OFFSET..], EthernetType::Ipv4 as u16);
        let ethernet = EthernetHeader::new(&header).unwrap();
        assert_eq!(ethernet.dst(), da.octets()[..].into());
        assert_eq!(ethernet.src(), sa.octets()[..].into());
        assert_eq!(ethernet.eth_type(), EthernetType::Ipv4);
    }
}
