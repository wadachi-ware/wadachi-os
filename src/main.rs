#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test::test_runner)]
#![reexport_test_harness_main = "test_entry"]

pub mod stdio;
pub mod test;

use core::panic::PanicInfo;

#[no_mangle]
pub fn wadachi_start() -> ! {
    #[cfg(test)]
    test_entry();

    println!("Hello world");

    shutdown(0);
}

pub fn shutdown(exit_code: u32) -> ! {
    // valid exit code length is 9

    use qemu_exit::QEMUExit;

    let qemu_exit_handler = qemu_exit::RISCV64::new(0x100000);
    qemu_exit_handler.exit(exit_code);
}

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
