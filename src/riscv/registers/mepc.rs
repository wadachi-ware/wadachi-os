use super::CSRegister;

use custom_test::custom_test;

#[derive(Debug, PartialEq, Default, Clone)]
pub struct MEPC {
    // see p42 in https://people.eecs.berkeley.edu/~krste/papers/riscv-privileged-v1.9.1.pdf
    value: usize,
}
impl MEPC {
    #[allow(unused)]
    #[inline]
    pub fn set(mut self, addr: usize) -> Self {
        self.value = addr;
        self
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
}

#[custom_test(ModeMachine)]
fn write_mepc_test() {
    unsafe {
        MEPC::initialize();
    }
    MEPC::operate(|old| old.set(0xdeadbeef));

    assert_eq!(MEPC::read().value, 0xdeadbeef);
}
