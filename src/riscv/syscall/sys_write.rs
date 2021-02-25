use core::ffi::c_void;
use custom_test::custom_test;

#[allow(unused)]
#[cfg(test)]
pub static mut WRITE_SYSCALL_FLAG: bool = false;

pub fn sys_write(buffer: *const c_void) {
    // println!("pointer = {:x}", buffer as usize);

    #[cfg(test)]
    unsafe {
        WRITE_SYSCALL_FLAG = true
    };

    if buffer as usize == 0 {
        panic!("call sys_write with null pointer");
    }

    let mut p: *const u8 = buffer as *const u8;
    loop {
        let v = unsafe { *p };
        if v == 0 {
            break;
        }
        p = unsafe { p.offset(1) };
        print!("{}", v as char);
    }
}

use cstr_core::{CStr, CString};

#[no_mangle]
#[custom_test(ModeSupervisor)]
fn write_systemcall_test() {
    let obj = CString::new("Hello").expect("fail");
    let test = obj.as_ptr();
    #[cfg(test)]
    assert_eq!(unsafe { WRITE_SYSCALL_FLAG }, false);
    unsafe {
        llvm_asm!("mv   a1, $0":: "r"(test as usize) :: " volatile");
        llvm_asm!("li   a0, 0");
        llvm_asm!("ecall");
    }
    #[cfg(test)]
    assert_eq!(unsafe { WRITE_SYSCALL_FLAG }, true);
}
