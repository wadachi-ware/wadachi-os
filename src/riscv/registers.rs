#[allow(unused)]
macro_rules! write_csr {
    ($csr_name: expr) => {
        #[inline]
        #[allow(unused)]
        unsafe fn internal_write(value: usize) {
            llvm_asm!(concat!("csrw ", $csr_name, ", $0") :: "r"(value):: "volatile");
        }
    };
}

#[allow(unused)]
macro_rules! read_csr {
    ($csr_name: expr) => {
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
#[allow(unused)]
macro_rules! make_bit_get_set_method {
    (accessibility = $accessibility: ident, field_name = $field_name: ident, internal_name = $internal_name: ident, bit = $bit: expr) => {
        paste::item! {
            #[inline]
            #[allow(unused)]
            $accessibility fn [<get_ $field_name>](&self) -> bool {
                use bit_field::BitField;
                self.$internal_name.get_bit($bit)
            }
            #[inline]
            #[allow(unused)]
            $accessibility fn [<set_ $field_name>](mut self, v: bool) -> Self {
                use bit_field::BitField;
                self.$internal_name.set_bit($bit, v);
                self
            }
        }
    };
    (field_name = $field_name: ident, bit = $bit: expr) => {
        make_bit_get_set_method!(accessibility = pub, field_name = $field_name, internal_name = value, bit = $bit);
    };
}

pub mod mepc;
pub mod mstatus;
pub mod mtvec;
pub mod pmpaddr;
pub mod pmpcfg;
pub mod satp;
pub mod stvec;

pub trait CSRegister
where
    Self: Sized,
    Self: Default,
{
    /// write back after some operations
    fn operate<F: Fn(Self) -> Self>(f: F) {
        unsafe {
            Self::write(f(Self::read()));
        }
    }
    unsafe fn initialize() {
        // for tests
        Self::write(Self::default());
    }

    unsafe fn write(_: Self);
    fn read() -> Self;
}
