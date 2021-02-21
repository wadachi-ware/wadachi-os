#[macro_use]
pub mod stdio;
pub mod malloc;

use super::{
    riscv::{
        instructions::{mret, wfi},
        registers::{
            mepc::MEPC,
            mstatus::{MStatus, MPP},
            pmpaddr::*,
            pmpcfg::{AddressMatching, PMPCfg},
            satp::{MODE32, SATP},
            CSRegister,
        },
    },
    supervisor::supervisor_start,
};

#[no_mangle]
#[allow(unreachable_code)]
pub fn machine_start() -> ! {
    unsafe {
        println!("-- Stack -- ");
        println!(" +-- start: {:x}", crate::KERNEL_STACK_START_ADDR);
        println!(" +-- end  : {:x}", crate::KERNEL_STACK_END_ADDR);
        println!("-- Heap  -- ");
        println!(" +-- start: {:x}", crate::KERNEL_HEAP_START_ADDR);
        println!(" +-- end  : {:x}", crate::KERNEL_HEAP_END_ADDR);
    }

    println!("Initializing heap...");
    malloc::init_heap();

    #[cfg(test)]
    crate::test_entry();

    println!("Hello Kernel!");
    println!("In machine mode");

    MStatus::operate(|old| old.set_mpp(MPP::Supervisor));

    MEPC::operate(|old| old.set(supervisor_start as usize));

    SATP::operate(|old| old.set_mode(MODE32::Bare));

    PMPCfg::operate(|old| {
        old.rule_operate(0, |rule| {
            rule.set_adr_mth(AddressMatching::TOR)
                .set_read(true)
                .set_write(true)
                .set_execute(true)
        })
    });

    PMPAddr0::operate(|old| old.set_addr(0xffffffff));

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
