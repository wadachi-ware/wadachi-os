#[macro_use]
pub mod stdio;
pub mod test;

use super::{
    riscv::registers::{
        mepc::MEPC,
        mstatus::{MStatus, MPP},
        CSRegister,
    },
    supervisor::supervisor_start,
};

const QEMU_VIRTIO_EXIT_ADDRESS: u64 = 0x100000;
// see http://www.katsuster.net/index.php?arg_act=cmd_show_diary&arg_date=20210203&arg_count_article=20

#[no_mangle]
pub fn machine_start() -> ! {
    #[cfg(test)]
    crate::test_entry();

    println!("Hello Kernel!");
    println!("In machine mode");

    MStatus::operate(|mut old| {
        old.set_mpp(MPP::Supervisor);

        old
    });

    MEPC::operate(|mut old| {
        old.set(supervisor_start as usize);

        old
    });

    shutdown(0);
}

pub fn shutdown(exit_code: u32) -> ! {
    // valid exit code length is 9

    use qemu_exit::QEMUExit;

    let qemu_exit_handler = qemu_exit::RISCV64::new(QEMU_VIRTIO_EXIT_ADDRESS);
    qemu_exit_handler.exit(exit_code);
}
