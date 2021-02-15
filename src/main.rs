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

const QEMU_VIRTIO_EXIT_ADDRESS: u64 = 0x100000;
// see http://www.katsuster.net/index.php?arg_act=cmd_show_diary&arg_date=20210203&arg_count_article=20

pub fn shutdown(exit_code: u32) -> ! {
    // valid exit code length is 9

    use qemu_exit::QEMUExit;

    let qemu_exit_handler = qemu_exit::RISCV64::new(QEMU_VIRTIO_EXIT_ADDRESS);
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
