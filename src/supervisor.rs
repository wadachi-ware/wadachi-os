#[macro_use]
pub mod syscall_wrapper;

use crate::machine::shutdown;

#[allow(unused)]
pub fn supervisor_start() -> ! {
    println!("In supervisor mode");

    sprintln!("Hello by syscall 1: Write");

    shutdown(0);
}
