use crate::machine::shutdown;
use crate::riscv::instructions::ecall;

#[allow(unused)]
pub fn supervisor_start() -> ! {
    println!("In supervisor mode");

    ecall::ecall();

    println!("Return to supervisor!");

    shutdown(0);
}
