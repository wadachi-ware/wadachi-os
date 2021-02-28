use core::ffi::c_void;
use cstr_core::{c_char, CStr, CString};
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

    let text = unsafe { CStr::from_ptr(buffer as *const c_char) };

    println!("{}", text.to_str().unwrap());
}

#[no_mangle]
#[custom_test(ModeSupervisor)]
fn write_systemcall_test() {
    let obj = CString::new("syscall_test_text").expect("fail");
    let test = obj.as_ptr();
    #[cfg(test)]
    assert_eq!(unsafe { WRITE_SYSCALL_FLAG }, false);
    unsafe {
        llvm_asm!("mv   a1, a0":: "r"(test as usize) :: );
        llvm_asm!("li   a0, 0");
        llvm_asm!("ecall");
    }
    #[cfg(test)]
    assert_eq!(unsafe { WRITE_SYSCALL_FLAG }, true);
}
