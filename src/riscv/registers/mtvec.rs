use super::CSRegister;
use bit_field::BitField;

use custom_test::custom_test;

#[allow(unused)]
#[derive(PartialEq, Debug, Default)]
pub struct MTVec {
    value: usize,
}

pub enum MTVecMode {
    Direct = 0,
    Vectored = 1,
}

impl MTVec {
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
    pub fn get_mode(&self) -> MTVecMode {
        match self.value.get_bits(0..=1) {
            0b00 => MTVecMode::Direct,
            0b01 => MTVecMode::Vectored,
            _ => unimplemented!("Reserved"),
        }
    }
    #[allow(unused)]
    #[inline]
    pub fn set_mode(mut self, mode: MTVecMode) -> Self {
        self.value.set_bits(0..=1, mode as usize);
        self
    }
}

impl CSRegister for MTVec {
    unsafe fn write(mtv: Self) {
        write_csr!("mtvec");
        internal_write(mtv.value);
    }
    fn read() -> Self {
        read_csr!("mtvec");
        Self {
            value: internal_read(),
        }
    }
}

#[custom_test(ModeMachine)]
fn mtvec_write_test() {
    let mtv = MTVec::default()
        .set_addr(0xfffffff0)
        .set_mode(MTVecMode::Vectored);
    assert_eq!(mtv.value, 0xfffffff1);
}

#[custom_test(ModeMachine)]
fn write_mtvec_test() {
    unsafe {
        MTVec::initialize();
    }
    MTVec::operate(|old| old.set_addr(0xfffffff0).set_mode(MTVecMode::Vectored));

    assert_eq!(MTVec::read().value, 0xfffffff1);
}
