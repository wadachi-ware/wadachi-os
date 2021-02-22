use bit_field::BitField;

use super::CSRegister;

use custom_test::custom_test;

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
pub enum SPP {
    User = 0,
    Supervisor = 1,
}
#[allow(unused)]
pub enum FS {
    Off = 0,
    Initial = 1,
    Clean = 2,
    Dirty = 3,
}
pub enum XS {
    AllOff = 0,
    NoneDirtyOrClean = 1,
    NoneDirtySomeClean = 2,
    SomeDirty = 3,
}

impl MStatus {
    make_bit_get_set_method!(field_name = uie, bit = 0);
    make_bit_get_set_method!(field_name = sie, bit = 1);
    make_bit_get_set_method!(field_name = mie, bit = 3);

    make_bit_get_set_method!(field_name = upie, bit = 4);
    make_bit_get_set_method!(field_name = spie, bit = 5);
    make_bit_get_set_method!(field_name = mpie, bit = 7);

    make_bit_get_set_method!(field_name = mprv, bit = 17);
    make_bit_get_set_method!(field_name = sum, bit = 18);
    make_bit_get_set_method!(field_name = mxr, bit = 19);
    make_bit_get_set_method!(field_name = tvm, bit = 20);
    make_bit_get_set_method!(field_name = tw, bit = 21);
    make_bit_get_set_method!(field_name = tsr, bit = 22);

    make_bit_get_set_method!(field_name = sd, bit = 31);

    #[allow(unused)]
    #[inline]
    pub fn get_spp(&self) -> SPP {
        match self.value.get_bit(8) {
            false => SPP::User,
            true => SPP::Supervisor,
        }
    }
    #[inline]
    #[allow(unused)]
    pub fn set_spp(mut self, spp: SPP) -> Self {
        self.value.set_bit(
            8,
            match spp {
                SPP::User => false,
                SPP::Supervisor => true,
            },
        );
        self
    }
    #[inline]
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
    #[inline]
    #[allow(unused)]
    #[inline]
    pub fn set_mpp(mut self, mpp: MPP) -> Self {
        self.value.set_bits(11..=12, mpp as usize);
        self
    }
    #[inline]
    #[allow(unused)]
    pub fn set_fs(mut self, fs: FS) -> Self {
        self.value.set_bits(13..=14, fs as usize);
        self
    }
    #[inline]
    #[allow(unused)]
    pub fn get_xs(&self) -> XS {
        match self.value.get_bits(15..=16) {
            0b00 => XS::AllOff,
            0b01 => XS::NoneDirtyOrClean,
            0b10 => XS::NoneDirtySomeClean,
            0b11 => XS::SomeDirty,
            _ => unreachable!(),
        }
    }
    #[inline]
    #[allow(unused)]
    pub fn set_xs(mut self, xs: XS) -> Self {
        self.value.set_bits(15..=16, xs as usize);
        self
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

#[custom_test(ModeMachine)]
fn mpp_write_test() {
    let mut ms = MStatus::default();
    ms = ms.set_mpp(MPP::Supervisor);

    assert_eq!(ms.value, 0b100000000000);
    assert_eq!(ms.get_mpp(), MPP::Supervisor);

    ms = ms.set_mpp(MPP::Machine);

    assert_eq!(ms.value, 0b1100000000000);
    assert_eq!(ms.get_mpp(), MPP::Machine);
}

#[custom_test(ModeMachine)]
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

#[custom_test(ModeMachine)]
fn operate_mstatus_test() {
    unsafe {
        MStatus::initialize();
    }

    MStatus::operate(|old| old.set_mpp(MPP::Machine));

    assert_eq!(MStatus::read().value, 0b1100000000000);
}
