use seq_macro::seq;

use custom_test::custom_test;

seq!(i in 0..=11 {
    #[repr(C)]
    #[derive(Debug)]
    pub struct RISCV32RawJumpBuffer {
        #(s#i: usize,)*
        sp: usize,
        ra: usize,
    }
    impl RISCV32RawJumpBuffer {
        pub const fn default() -> Self {
            Self {
                #(s#i: 0,)*
                sp: 0,
                ra: 0,
            }
        }
    }
});

extern "C" {
    #[allow(unused)]
    pub fn setjmp(_: &mut RISCV32RawJumpBuffer) -> usize;
    #[allow(unused)]
    pub fn longjmp(_: &RISCV32RawJumpBuffer, _: usize) -> !;
}

#[allow(unused)]
pub static mut JMP_BUF: RISCV32RawJumpBuffer = RISCV32RawJumpBuffer::default();

#[custom_test(ModeMachine)]
fn setjmp_use_local_test() {
    use core::ptr::{read_volatile, write_volatile};

    let mut buf = RISCV32RawJumpBuffer::default();
    let mut flag = false;

    let v = unsafe { setjmp(&mut buf) };
    if v != 0 {
        assert_eq!(v, 1);
        assert_eq!(unsafe { read_volatile(&flag) }, true);
        return;
    }

    unsafe {
        write_volatile(&mut flag, true);
    }

    unsafe {
        longjmp(&buf, 1);
    }
}

#[custom_test(ModeMachine)]
fn setjmp_use_global_test() {
    use core::ptr::{read_volatile, write_volatile};

    let mut flag = false;

    let v = unsafe { setjmp(&mut JMP_BUF) };
    if v != 0 {
        assert_eq!(v, 1);
        assert_eq!(unsafe { read_volatile(&flag) }, true);
        return;
    }

    unsafe {
        write_volatile(&mut flag, true);
    }

    unsafe {
        longjmp(&JMP_BUF, 1);
    }
}
