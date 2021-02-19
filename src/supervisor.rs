use crate::machine::shutdown;

#[allow(unused)]
pub fn supervisor_start() -> ! {
    println!("In supervisor mode");

    shutdown(0);
}
