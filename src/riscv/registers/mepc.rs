use super::CSRegister;

#[derive(Debug, PartialEq)]
pub struct MEPC {
    // see p42 in https://people.eecs.berkeley.edu/~krste/papers/riscv-privileged-v1.9.1.pdf
    value: usize,
}
impl MEPC {
    #[allow(unused)]
    pub fn set(&mut self, addr: usize) {
        self.value = addr;
    }
}

impl CSRegister for MEPC {
    unsafe fn write(mepc: Self) {
        write_csr!("mepc");
        internal_write(mepc.value);
    }
    fn read() -> Self {
        read_csr!("mepc");
        Self {
            value: internal_read(),
        }
    }
    fn get_unset() -> Self {
        Self { value: 0 }
    }
}

#[test_case]
fn write_mepc_test() {
    unsafe {
        MEPC::initialize();
    }
    MEPC::operate(|mut old| {
        old.set(0xdeadbeef);

        old
    });

    assert_eq!(MEPC::read().value, 0xdeadbeef);
}
