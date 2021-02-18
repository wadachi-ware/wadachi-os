#[allow(unused)]
pub fn wfi() {
    unsafe {
        llvm_asm!("wfi");
    }
}

