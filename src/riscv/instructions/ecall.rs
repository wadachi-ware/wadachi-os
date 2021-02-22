#[allow(unused)]
pub fn ecall() -> ! {
    unsafe {
        llvm_asm!("ecall");
    }

    unreachable!();
}

