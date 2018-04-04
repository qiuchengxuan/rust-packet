pub const MAC_ADDR_LEN: usize = 6;

#[derive(Debug)]
pub struct MacAddr<'a> {
    raw: &'a [u8],
}

impl<'a> PartialEq for MacAddr<'a> {
    fn eq(&self, other: &MacAddr) -> bool {
        self.raw == other.raw
    }
}

impl<'a> From<&'a [u8]> for MacAddr<'a> {
    fn from(raw: &'a [u8]) -> MacAddr {
        MacAddr {
            raw: &raw[..MAC_ADDR_LEN],
        }
    }
}
