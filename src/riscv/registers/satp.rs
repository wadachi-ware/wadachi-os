use super::CSRegister;
use bit_field::BitField;

use custom_test::custom_test;

#[allow(unused)]
#[derive(Debug, PartialEq, Default, Clone)]
pub struct SATP {
    // Supervisor Address Translatiion and Protection
    // see p66 at https://riscv.org/wp-content/uploads/2017/05/riscv-privileged-v1.10.pdf
    value: usize,
}

#[derive(Debug, PartialEq)]
pub enum MODE32 {
    Bare = 0,
    Sv32 = 1,
}

impl SATP {
    #[allow(unused)]
    #[inline]
    pub fn get_mode(&self) -> MODE32 {
        // for 32bit mode
        match self.value.get_bit(31) {
            false => MODE32::Bare,
            true => MODE32::Sv32,
        }
    }
    #[allow(unused)]
    #[inline]
    pub fn set_mode(mut self, mode: MODE32) -> Self {
        self.value.set_bit(
            31,
            match mode {
                MODE32::Bare => false,
                MODE32::Sv32 => true,
            },
        );
        self
    }
    #[allow(unused)]
    #[inline]
    pub fn get_asid(&self) -> usize {
        // TODO: usize --> ASID
        self.value.get_bits(22..=30)
    }
    #[allow(unused)]
    #[inline]
    pub fn set_asid(mut self, v: usize) -> Self {
        self.value.set_bits(22..=30, v);
        self
    }
    #[allow(unused)]
    #[inline]
    pub fn get_ppn(&self) -> usize {
        // TODO: usize --> PPN
        self.value.get_bits(0..=21)
    }
    #[allow(unused)]
    #[inline]
    pub fn set_ppn(mut self, v: usize) -> Self {
        self.value.set_bits(0..=21, v);
        self
    }
}

impl CSRegister for SATP {
    unsafe fn write(satp: Self) {
        write_csr!("satp");
        internal_write(satp.value);
    }
    fn read() -> Self {
        read_csr!("satp");
        Self {
            value: internal_read(),
        }
    }
}

#[custom_test(ModeMachine)]
fn mode_write_test() {
    let mut satp = SATP::default();
    satp = satp.set_mode(MODE32::Bare);

    assert_eq!(satp.value, 0);
    assert_eq!(satp.get_mode(), MODE32::Bare);

    satp = satp.set_mode(MODE32::Sv32);
    assert_eq!(satp.value, 1 << 31);
    assert_eq!(satp.get_mode(), MODE32::Sv32);
}

#[custom_test(ModeMachine)]
fn write_mode_test() {
    unsafe {
        SATP::initialize();
    }

    SATP::operate(|old| old.set_mode(MODE32::Sv32));

    assert_eq!(SATP::read().value, 1 << 31);
}
