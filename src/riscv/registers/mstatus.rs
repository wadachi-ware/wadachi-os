use bit_field::BitField;

use super::CSRegister;

#[allow(unused)]
#[derive(Debug, PartialEq, Default, Clone)]
pub struct MStatus {
    // see p21 in https://people.eecs.berkeley.edu/~krste/papers/riscv-privileged-v1.9.1.pdf
    value: usize,
}

#[allow(unused)]
#[derive(Debug, PartialEq, Clone)]
pub enum MPP {
    User = 0b00,
    Supervisor = 0b01,
    // Hypervisor = 0b10,
    Machine = 0b11,
}

impl MStatus {
    #[allow(unused)]
    #[inline]
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
    #[inline]
    pub fn set_mpp(&self, mpp: MPP) -> Self {
        let mut ret = self.clone();
        ret.value.set_bits(11..=12, mpp as usize);
        ret
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
    let mut ms = MStatus::default();
    ms = ms.set_mpp(MPP::Supervisor);

    assert_eq!(ms.value, 0b100000000000);
    assert_eq!(ms.get_mpp(), MPP::Supervisor);

    ms = ms.set_mpp(MPP::Machine);

    assert_eq!(ms.value, 0b1100000000000);
    assert_eq!(ms.get_mpp(), MPP::Machine);
}

#[test_case]
fn write_mstatus_test() {
    unsafe {
        MStatus::initialize();
    }

    let mut ms = MStatus::read();
    ms = ms.set_mpp(MPP::Machine);
    unsafe {
        MStatus::write(ms);
    }

    assert_eq!(MStatus::read().value, 0b1100000000000);
}

#[test_case]
fn operate_mstatus_test() {
    unsafe {
        MStatus::initialize();
    }

    MStatus::operate(|old| old.set_mpp(MPP::Machine));

    assert_eq!(MStatus::read().value, 0b1100000000000);
}
