use bit_field::BitField;

use super::csr::CSRegister;

#[derive(Debug)]
pub struct MStatus {
    value: usize,
}

#[allow(unused)]
#[derive(Debug, PartialEq)]
pub enum MPP {
    User = 0b00,
    Supervisor = 0b01,
    // Hypervisor = 0b10,
    Machine = 0b11,
}

impl MStatus {
    #[allow(unused)]
    pub fn get_mpp(&self) -> MPP {
        match self.value.get_bits(11..=12) {
            0b00 => MPP::User,
            0b01 => MPP::Supervisor,
            0b10 => unimplemented!(),
            0b11 => MPP::Machine,
            _ => unreachable!(),
        }
    }
    #[allow(unused)]
    pub fn set_mpp(&mut self, mpp: MPP) {
        self.value.set_bits(11..=12, mpp as usize);
    }
}

impl CSRegister for MStatus {
    unsafe fn write(ms: Self) {
        write_csr!("mstatus");
        internal_write(ms.value);
    }
    fn read() -> Self {
        read_csr!("mstatus");
        Self {
            value: internal_read(),
        }
    }
}

#[test_case]
fn mpp_write_test() {
    let mut ms = MStatus { value: 0 };
    ms.set_mpp(MPP::Supervisor);

    assert_eq!(ms.value, 0b100000000000);
    assert_eq!(ms.get_mpp(), MPP::Supervisor);

    ms.set_mpp(MPP::Machine);

    assert_eq!(ms.value, 0b1100000000000);
    assert_eq!(ms.get_mpp(), MPP::Machine);
}

#[test_case]
fn write_mstatus_test() {
    let mut ms = MStatus::read();
    ms.set_mpp(MPP::Machine);
    unsafe {
        MStatus::write(ms);
    }

    ms = MStatus::read();
    assert_eq!(ms.value, 0b1100000000000);
}
