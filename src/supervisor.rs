#[allow(unused)]
pub fn supervisor_start() -> ! {
    println!("In supervisor mode");

    unsafe {
        loop {
            llvm_asm!("wfi");
        }
    }

    loop {}
}
