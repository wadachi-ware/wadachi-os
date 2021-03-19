use super::ppn::PPN;
use bit_field::BitField;
use bitflags::bitflags;

/// Page Table Entry
/// see p. 59 at https://riscv.org/wp-content/uploads/2017/05/riscv-privileged-v1.10.pdf
#[allow(unused)]
#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
pub struct PTE {
    value: usize,
}

bitflags! {
    pub struct Attributes: u8 {
        const V = 0b00000001; // valid
        const R = 0b00000010; // readable
        const W = 0b00000100; // writable
        const X = 0b00001000; // executable
        const U = 0b00010000; // accessibile in user mode
        const G = 0b00100000; // global mapping
        const A = 0b01000000; // accessed
        const D = 0b10000000; // dirty
    }
}

impl PTE {
    #[allow(unused)]
    #[inline]
    pub fn new(ppn: &PPN, attrs: &Attributes) -> Self {
        PTE {value: ppn.to_page() << 10 | attrs.bits() as usize}
    }

    #[allow(unused)]
    #[inline]
    pub fn get_ppn(&self) -> PPN {
        PPN::from_page(self.value >> 10)
    }

    #[allow(unused)]
    #[inline]
    pub fn get_attrs(&self) -> Attributes {
        match Attributes::from_bits(self.value.get_bits(0..8) as u8) {
            Some(attrs) => attrs,
            None => Attributes::empty(),
        }
    }
}
