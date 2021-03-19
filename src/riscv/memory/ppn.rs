use bit_field::BitField;
use alloc::fmt;

/// Physical Page Number devides physical memory addresses into 4 KB pages.
#[allow(unused)]
#[derive(PartialEq, Eq, Default, Clone)]
pub struct PPN {
    /// unsigned 22 bits
    value: usize,
}

impl PPN {
    /// Returns the PPN indicating the specified page number
    #[allow(unused)]
    #[inline]
    pub fn from_page(num: usize) -> Self {
        PPN {value: num.get_bits(0..=21)}
    }

    /// Returns the PPN corresponding to the specified address
    #[allow(unused)]
    #[inline]
    pub fn from_addr(addr: usize) -> Self {
        Self::from_page(addr >> 12)
    }

    /// Returns the page number indicated the specified PPN
    #[allow(unused)]
    #[inline]
    pub fn to_page(&self) -> usize {
        self.value
    }

    /// Returns the lower 10 bits indicated the specified PPN
    #[allow(unused)]
    #[inline]
    pub fn get_0(&self) -> usize {
        self.value.get_bits(0..10)
    }

    /// Returns the higher 10 bits indicated the specified PPN
    #[allow(unused)]
    #[inline]
    pub fn get_1(&self) -> usize {
        self.value.get_bits(10..22)
    }
}

impl fmt::Debug for PPN {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:09X}", self.value)
    }
}