#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(llvm_asm)]
#![test_runner(crate::tests::test::test_runner)]
#![reexport_test_harness_main = "test_entry"]
#![feature(alloc_error_handler)]

extern "C" {
    static KERNEL_HEAP_START_ADDR: usize;
    static KERNEL_HEAP_END_ADDR: usize;
    static KERNEL_STACK_START_ADDR: usize;
    static KERNEL_STACK_END_ADDR: usize;
}

#[macro_use]
mod machine;
mod riscv;
mod supervisor;
mod tests;

extern crate alloc;

use machine::*;

use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("[!] Kernel Panic!");
    println!("info: {}", info);

    shutdown(1);
}

#[no_mangle]
pub fn abort() -> ! {
    println!("[!] Kernel Abort!");
    loop {}
}
