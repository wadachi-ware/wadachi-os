use crate::machine::shutdown;

#[allow(unused)]
pub fn supervisor_start() -> ! {
    unsafe {
        llvm_asm!("li   a0, 0");
        llvm_asm!("ecall");
    }

    println!("Return to supervisor!");

    shutdown(0);
}
