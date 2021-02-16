#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(llvm_asm)]
#![test_runner(machine::test::test_runner)]
#![reexport_test_harness_main = "test_entry"]

#[macro_use]
mod machine;
mod riscv;
mod supervisor;

use machine::*;

use riscv::mstatus::*;

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
