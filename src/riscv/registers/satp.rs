use super::CSRegister;
use bit_field::BitField;

#[allow(unused)]
#[derive(Debug, PartialEq)]
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
    pub fn get_mode(&self) -> MODE32 {
        // for 32bit mode
        match self.value.get_bit(31) {
            false => MODE32::Bare,
            true => MODE32::Sv32,
        }
    }
    #[allow(unused)]
    pub fn set_mode(&mut self, mode: MODE32) {
        self.value.set_bit(
            31,
            match mode {
                MODE32::Bare => false,
                MODE32::Sv32 => true,
            },
        );
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
    fn get_unset() -> Self {
        Self { value: 0 }
    }
}

#[test_case]
fn mode_write_test() {
    let mut satp = SATP::get_unset();
    satp.set_mode(MODE32::Bare);

    assert_eq!(satp.value, 0);
    assert_eq!(satp.get_mode(), MODE32::Bare);

    satp.set_mode(MODE32::Sv32);
    assert_eq!(satp.value, 1 << 31);
    assert_eq!(satp.get_mode(), MODE32::Sv32);
}

#[test_case]
fn write_mode_test() {
    unsafe {
        SATP::initialize();
    }

    let mut satp = SATP::read();
    satp.set_mode(MODE32::Sv32);
    unsafe {
        SATP::write(satp);
    }

    assert_eq!(SATP::read().value, 1 << 31);
}
