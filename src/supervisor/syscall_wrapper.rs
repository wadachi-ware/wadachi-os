use core::fmt::{Error, Write};
use cstr_core::CString;

struct PrintBufferDummy;

impl Write for PrintBufferDummy {
    fn write_str(&mut self, msg: &str) -> Result<(), Error> {
        let cstr = CString::new(msg).expect("Could not convert c string");
        let ptr = cstr.as_ptr();

        unsafe {
            llvm_asm!("mv   a1, a0":: "r"(ptr as usize) ::);
            llvm_asm!("li   a0, 0");
            llvm_asm!("ecall");
        };

        Ok(())
    }
}

pub fn _print(args: core::fmt::Arguments) {
    let mut dummy = PrintBufferDummy;
    dummy.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! sprint {
    ($($arg:tt)*) => ($crate::supervisor::syscall_wrapper::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! sprintln {
    () => ($crate::sprint!("\n"));
    ($($arg:tt)*) => ($crate::sprint!("{}\n", format_args!($($arg)*)));
}
