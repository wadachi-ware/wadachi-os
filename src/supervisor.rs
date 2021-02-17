use super::*;

pub fn supervisor_start() -> ! {
    println!("In supervisor mode");

    shutdown(0);
}
