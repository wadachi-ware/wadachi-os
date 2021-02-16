pub trait CSRegister {
    /// write back after some operations
    fn writes(f: fn(Self) -> Self)
    where
        Self: Sized,
    {
        unsafe {
            Self::write(f(Self::read()));
        }
    }

    unsafe fn write(_: Self);
    fn read() -> Self;
}

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
