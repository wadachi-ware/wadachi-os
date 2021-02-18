#[macro_use]
pub mod stdio;
pub mod test;

use super::{
    riscv::{
        instructions::{mret, wfi},
        registers::{
            mepc::MEPC,
            mstatus::{MStatus, MPP},
            pmpaddr::*,
            pmpcfg::{AddressMatching, PMPCfg, PMPRule},
            satp::{MODE32, SATP},
            CSRegister,
        },
    },
    supervisor::supervisor_start,
};

#[no_mangle]
#[allow(unreachable_code)]
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

    SATP::operate(|mut old| {
        old.set_mode(MODE32::Bare);

        old
    });

    PMPCfg::operate(|mut old| {
        let rule0: &mut PMPRule = old.get_mut_rule_at(0);
        rule0.set_adr_mth(AddressMatching::TOR);
        rule0.set_read(true);
        rule0.set_write(true);
        rule0.set_execute(true);

        old
    });

    PMPAddr0::operate(|mut old| {
        old.set_addr(0xffffffff);

        old
    });

    mret::mret();
}

const QEMU_VIRTIO_EXIT_ADDRESS: u64 = 0x100000;
// see https://github.com/qemu/qemu/blob/master/hw/riscv/virt.c#L52

const QEMU_VIRTIO_EXIT_CDOE_FAIL: u32 = 0x3333;
// see https://github.com/qemu/qemu/blob/master/hw/misc/sifive_test.c#L42

#[no_mangle]
pub fn shutdown(exit_code: u16) -> ! {
    use core::ptr::write_volatile;

    let return_code: u32 = (exit_code as u32) << 16 | QEMU_VIRTIO_EXIT_CDOE_FAIL;

    unsafe {
        // *(QEMU_VIRTIO_EXIT_ADDRESS as *mut u32) = exit_code;
        write_volatile(QEMU_VIRTIO_EXIT_ADDRESS as *mut u32, return_code);
    }

    loop {
        wfi::wfi();
    }
}
