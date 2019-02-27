/// A type of variable-length integer commonly used in the Bitcoin P2P protocol and Bitcoin serialized data structures.
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct CompactInteger(pub u64);

impl From<CompactInteger> for usize {
    fn from(i: CompactInteger) -> Self {
        i.0 as usize
    }
}

impl From<CompactInteger> for u64 {
    fn from(i: CompactInteger) -> Self {
        i.0
    }
}

impl From<u8> for CompactInteger {
    fn from(i: u8) -> Self {
        CompactInteger(i as u64)
    }
}

impl From<u16> for CompactInteger {
    fn from(i: u16) -> Self {
        CompactInteger(i as u64)
    }
}

impl From<u32> for CompactInteger {
    fn from(i: u32) -> Self {
        CompactInteger(i as u64)
    }
}

impl From<usize> for CompactInteger {
    fn from(i: usize) -> Self {
        CompactInteger(i as u64)
    }
}

impl From<u64> for CompactInteger {
    fn from(i: u64) -> Self {
        CompactInteger(i)
    }
}
