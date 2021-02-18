#[allow(unused)]
macro_rules! write_csr {
    ($csr_name: literal) => {
        #[inline]
        #[allow(unused)]
        unsafe fn internal_write(value: usize) {
            llvm_asm!(concat!("csrw ", $csr_name, ", $0") :: "r"(value):: "volatile");
        }
    };
}

#[allow(unused)]
macro_rules! read_csr {
    ($csr_name: literal) => {
        #[inline]
        #[allow(unused)]
        fn internal_read() -> usize {
            let ret;
            unsafe {
                    llvm_asm!(concat!("csrr $0, ", $csr_name) : "=r"(ret)::: "volatile");
            }
            ret
        }
    };
}

pub mod mepc;
pub mod mstatus;
pub mod satp;

pub trait CSRegister
where
    Self: Sized,
{
    /// write back after some operations
    fn operate<F: Fn(Self) -> Self>(f: F) {
        unsafe {
            Self::write(f(Self::read()));
        }
    }
    unsafe fn initialize() {
        // for tests
        Self::write(Self::get_unset());
    }

    unsafe fn write(_: Self);
    fn read() -> Self;
    fn get_unset() -> Self;
}
