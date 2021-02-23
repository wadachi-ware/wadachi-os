use super::CSRegister;
use bit_field::BitField;
use custom_test::custom_test;

#[derive(PartialEq, Default, Clone, Debug)]
pub struct STVec {
    value: usize,
}

pub enum STVecMode {
    Direct = 0,
    Vectored = 1,
}

impl STVec {
    #[allow(unused)]
    #[inline]
    pub fn get_addr(&self) -> usize {
        self.value.get_bits(2..=31) << 2
    }
    #[allow(unused)]
    #[inline]
    pub fn set_addr(mut self, addr: usize) -> Self {
        assert_eq!(addr.get_bits(0..=1), 0);
        self.value.set_bits(2..=31, addr >> 2);
        self
    }
    #[allow(unused)]
    #[inline]
    pub fn get_mode(&self) -> STVecMode {
        match self.value.get_bits(0..=1) {
            0b00 => STVecMode::Direct,
            0b01 => STVecMode::Vectored,
            _ => unimplemented!("Reserved"),
        }
    }
    #[allow(unused)]
    #[inline]
    pub fn set_mode(mut self, mode: STVecMode) -> Self {
        self.value.set_bits(0..=1, mode as usize);
        self
    }
}

impl CSRegister for STVec {
    unsafe fn write(stvec: Self) {
        write_csr!("stvec");
        internal_write(stvec.value);
    }
    fn read() -> Self {
        read_csr!("stvec");
        Self {
            value: internal_read(),
        }
    }
}

#[custom_test(ModeMachine)]
fn write_stvec_test() {
    unsafe {
        STVec::initialize();
    }
    STVec::operate(|old| old.set_addr(0xfffffff0).set_mode(STVecMode::Vectored));
    assert_eq!(STVec::read().value, 0xfffffff1);
}
