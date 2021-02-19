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

#[derive(Clone, Copy)]
pub union PMPCfg {
    csrs: [PMPCfgRegister; 4],
    // 32bit RISCV ISA has 4 pmpcfg registers.
    #[allow(unused)]
    // for tests
    value: u128,
}

impl Default for PMPCfg {
    fn default() -> Self {
        Self { value: 0 }
    }
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
    pub fn set_adr_mth(&self, adm: AddressMatching) -> Self {
        let mut ret = self.clone();
        ret.value.set_bits(3..=4, adm as u8);
        ret
    }
}

impl PMPCfg {
    #[allow(unused)]
    #[inline]
    pub fn rule_operate<F: Fn(PMPRule) -> PMPRule>(&self, index: usize, f: F) -> Self {
        let mut ret = self.clone();
        unsafe {
            ret.csrs[index / 4].rules[index % 4] = f(ret.csrs[index / 4].rules[index % 4]);
        }

        ret
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
}

#[test_case]
fn write_method_test() {
    let pmpcfg = PMPCfg::default();
    let pmpcfg = pmpcfg.rule_operate(0, |rule| {
        let rule = rule.set_lock(true);
        let rule = rule.set_adr_mth(AddressMatching::TOR);
        let rule = rule.set_read(true);
        let rule = rule.set_write(true);
        rule.set_execute(true)
    });

    unsafe {
        assert_eq!(pmpcfg.csrs[0].rules[0].value, 0b10001111);
    }

    let pmpcfg = pmpcfg.rule_operate(5, |rule| {
        let rule = rule.set_adr_mth(AddressMatching::NA4);
        let rule = rule.set_read(true);
        rule.set_execute(true)
    });

    assert_eq!(unsafe { pmpcfg.value }, 0b00010101 << (8 * 5) | 0b10001111);
}

/// Lock field cannot reset!!
#[test_case]
fn write_pmpcfg_test() {
    unsafe {
        PMPCfg::initialize();
    }

    PMPCfg::operate(|pmpcfg| {
        let pmpcfg = pmpcfg.rule_operate(0, |rule| {
            let rule = rule.set_adr_mth(AddressMatching::TOR);
            let rule = rule.set_read(true);
            let rule = rule.set_write(true);
            rule.set_execute(true)
        });

        pmpcfg.rule_operate(5, |rule| {
            let rule = rule.set_adr_mth(AddressMatching::NA4);
            let rule = rule.set_read(true);
            rule.set_execute(true)
        })
    });

    assert_eq!(
        unsafe { PMPCfg::read().value },
        0b00010101 << (8 * 5) | 0b00001111
    );

    unsafe {
        PMPCfg::initialize();
    }
    assert_eq!(unsafe { PMPCfg::read().value }, 0);
}
