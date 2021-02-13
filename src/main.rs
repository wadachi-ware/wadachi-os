#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub fn wadachi_start() -> ! {
    let uart = 0x1000_0000 as *mut u8;

    for c in b"hello wadashi-os".iter() {
        unsafe {
            *uart = *c as u8;
        }
    }

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
