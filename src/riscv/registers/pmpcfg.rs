use bit_field::BitField;

use super::CSRegister;

#[derive(Clone, Copy)]
pub struct PMPRule {
    value: u8,
}

pub enum AddressMatching {
    OFF = 0b00,
    TOR = 0b01,
    NA4 = 0b10,
    NAPOT = 0b11,
}

#[derive(Clone, Copy)]
union PMPCfgRegister {
    // not public
    rules: [PMPRule; 4],
    // 32bit RISCV ISA has 4 pmpcfg rules.
    value: usize,
}

pub union PMPCfg {
    csrs: [PMPCfgRegister; 4],
    // 32bit RISCV ISA has 4 pmpcfg registers.
    #[allow(unused)]
    // for tests
    value: u128,
}

impl PMPRule {
    // see p56 in https://riscv.org/wp-content/uploads/2017/05/riscv-privileged-v1.10.pdf
    make_bit_get_set_method!(field_name = read, bit = 0);
    make_bit_get_set_method!(field_name = write, bit = 1);
    make_bit_get_set_method!(field_name = execute, bit = 2);
    make_bit_get_set_method!(field_name = lock, bit = 7);

    #[allow(unused)]
    #[inline]
    pub fn get_adr_mth(&self) -> AddressMatching {
        match self.value.get_bits(3..=4) {
            0b00 => AddressMatching::OFF,
            0b01 => AddressMatching::TOR,
            0b10 => AddressMatching::NA4,
            0b11 => AddressMatching::NAPOT,
            _ => unreachable!(),
        }
    }
    #[allow(unused)]
    #[inline]
    pub fn set_adr_mth(&mut self, adm: AddressMatching) {
        self.value.set_bits(3..=4, adm as u8);
    }
}

impl PMPCfg {
    #[allow(unused)]
    pub fn get_mut_rule_at(&mut self, index: usize) -> &mut PMPRule {
        unsafe { &mut self.csrs[index / 4].rules[index % 4] }
    }
}

impl CSRegister for PMPCfg {
    #[allow(unused)]
    #[inline]
    unsafe fn write(s: Self) {
        // TODO: Split per Register
        {
            write_csr!("pmpcfg0");
            internal_write(s.csrs[0].value);
        }
        {
            write_csr!("pmpcfg1");
            internal_write(s.csrs[1].value);
        }
        {
            write_csr!("pmpcfg2");
            internal_write(s.csrs[2].value);
        }
        {
            write_csr!("pmpcfg3");
            internal_write(s.csrs[3].value);
        }
    }

    #[allow(unused)]
    #[inline]
    fn read() -> Self {
        Self {
            csrs: [
                {
                    read_csr!("pmpcfg0");
                    PMPCfgRegister {
                        value: internal_read(),
                    }
                },
                {
                    read_csr!("pmpcfg1");
                    PMPCfgRegister {
                        value: internal_read(),
                    }
                },
                {
                    read_csr!("pmpcfg2");
                    PMPCfgRegister {
                        value: internal_read(),
                    }
                },
                {
                    read_csr!("pmpcfg3");
                    PMPCfgRegister {
                        value: internal_read(),
                    }
                },
            ],
        }
    }

    #[allow(unused)]
    #[inline]
    fn get_unset() -> Self {
        Self {
            csrs: [
                PMPCfgRegister { value: 0 },
                PMPCfgRegister { value: 0 },
                PMPCfgRegister { value: 0 },
                PMPCfgRegister { value: 0 },
            ],
        }
    }
}

#[test_case]
fn write_method_test() {
    let mut pmpcfg = PMPCfg::get_unset();
    let rule: &mut PMPRule = pmpcfg.get_mut_rule_at(0);
    rule.set_lock(true);
    rule.set_adr_mth(AddressMatching::TOR);
    rule.set_read(true);
    rule.set_write(true);
    rule.set_execute(true);

    assert_eq!(rule.value, 0b10001111);

    let rule: &mut PMPRule = pmpcfg.get_mut_rule_at(5);
    rule.set_adr_mth(AddressMatching::NA4);
    rule.set_read(true);
    rule.set_execute(true);

    assert_eq!(unsafe { pmpcfg.value }, 0b00010101 << (8 * 5) | 0b10001111);
}

/// Lock field cannot reset!!
#[test_case]
fn write_pmpcfg_test() {
    unsafe {
        PMPCfg::initialize();
    }

    let mut pmpcfg = PMPCfg::read();
    pmpcfg.get_mut_rule_at(0).set_adr_mth(AddressMatching::TOR);
    pmpcfg.get_mut_rule_at(0).set_read(true);
    pmpcfg.get_mut_rule_at(0).set_write(true);
    pmpcfg.get_mut_rule_at(0).set_execute(true);

    pmpcfg.get_mut_rule_at(5).set_adr_mth(AddressMatching::NA4);
    pmpcfg.get_mut_rule_at(5).set_read(true);
    pmpcfg.get_mut_rule_at(5).set_execute(true);

    unsafe {
        PMPCfg::write(pmpcfg);
    }

    assert_eq!(
        unsafe { PMPCfg::read().value },
        0b00010101 << (8 * 5) | 0b00001111
    );

    unsafe {
        PMPCfg::initialize();
    }
    assert_eq!(unsafe { PMPCfg::read().value }, 0);
}
