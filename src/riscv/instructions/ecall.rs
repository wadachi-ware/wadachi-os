#[allow(unused)]
pub fn ecall() {
    // not unreachable!!
    unsafe {
        llvm_asm!("ecall");
    }
}
