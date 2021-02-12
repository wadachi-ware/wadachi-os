#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub fn wadachi_start() -> ! {
    loop {

    }
}

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {

    }
}

#[no_mangle]
pub fn abort() -> ! {
    loop {
    }
}



