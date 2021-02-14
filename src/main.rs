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

    loop {}
}

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub fn abort() -> ! {
    loop {}
}
