use bit_field::BitField;

#[allow(unused)]
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub struct PPN {
    // Physical Page Number
    // unsigned 22 bits
    value: usize,
}

impl PPN {
    #[allow(unused)]
    #[inline]
    pub fn from_page(num: usize) -> Self {
        PPN {value: num.get_bits(0..21)}
    }

    #[allow(unused)]
    #[inline]
    pub fn to_page(&self) -> usize {
        self.value
    }

    #[allow(unused)]
    #[inline]
    pub fn from_address(addr: usize) -> Self {
        PPN {value: addr.get_bits(12..31)}
    }

    #[allow(unused)]
    #[inline]
    pub fn to_address(&self) -> usize {
        self.value << 12
    }
}