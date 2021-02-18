#[allow(unused)]
pub fn mret() -> ! {
    unsafe {
        llvm_asm!("mret");
    }

    unreachable!();
}
