use super::CSRegister;
use custom_test::custom_test;

#[derive(Debug, PartialEq, Default, Clone)]
pub struct SEPC {
    // see p42 in https://people.eecs.berkeley.edu/~krste/papers/riscv-privileged-v1.9.1.pdf
    value: usize,
}
impl SEPC {
    #[allow(unused)]
    #[inline]
    pub fn get(&self) -> usize {
        self.value
    }
    #[allow(unused)]
    #[inline]
    pub fn set(mut self, addr: usize) -> Self {
        self.value = addr;
        self
    }
}

impl CSRegister for SEPC {
    unsafe fn write(sepc: Self) {
        write_csr!("sepc");
        internal_write(sepc.value);
    }
    fn read() -> Self {
        read_csr!("sepc");
        Self {
            value: internal_read(),
        }
    }
}

#[custom_test(ModeMachine)]
fn write_sepc_test() {
    unsafe {
        SEPC::initialize();
    }
    SEPC::operate(|old| old.set(0xdeadbeef));

    assert_eq!(SEPC::read().value, 0xdeadbeef);
}
