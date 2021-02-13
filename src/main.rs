#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test::test_runner)]

pub mod stdio;
pub mod test;

use core::fmt::Write;
use core::panic::PanicInfo;

use stdio::UARTBuffer;

#[no_mangle]
pub fn wadachi_start() -> ! {
    let mut buf: UARTBuffer = UARTBuffer::new(0x1000_0000 as *mut u8);
    write!(buf, "Hello from wadachi-os").unwrap();

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
