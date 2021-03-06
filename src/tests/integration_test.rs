#![allow(unused)]

use crate::{
    machine::{default_exception_handler, test_exception_handler, HANDLER_POINTER},
    riscv::{
        helper::setjmp::{longjmp, setjmp, RISCV32RawJumpBuffer, JMP_BUF},
        instructions::{ecall, mret},
        registers::{
            mcause::MCause,
            mepc::MEPC,
            mstatus::{MStatus, MPP},
            mtval::MTVal,
            mtvec::{MTVec, MTVecMode},
            pmpaddr::*,
            pmpcfg::{AddressMatching, PMPCfg},
            satp::{MODE32, SATP},
            sepc::SEPC,
            stval::STVal,
            stvec::STVec,
            CSRegister,
        },
    },
};
use core::ptr::{read_volatile, write_volatile};
use custom_test::custom_test;

static mut TEST_SAVED_MPP: Option<MPP> = None;

#[custom_test(IntegrationMachineToSupervisor)]
fn switch_mode_test_from_machine_to_supervisor() {
    let x = unsafe { setjmp(&mut JMP_BUF) };
    if x != 0 {
        assert_eq!(x, 10);
        println!("mode switch test, ok. ");
        return;
    }

    unsafe { crate::machine::HANDLER_POINTER = mode_switch_test_handler as usize }

    println!("step1: check current status. expect M-Mode");
    assert_eq!(unsafe { TEST_SAVED_MPP }, None);

    MTVec::operate(|old| {
        old.set_addr(crate::machine::test_exception_handler as usize)
            .set_mode(MTVecMode::Direct)
    });
    ecall::ecall();

    assert_eq!(
        unsafe { TEST_SAVED_MPP }.expect("exception was not handled"),
        MPP::Machine
    );

    println!("[b] step1 -- ok. current cpu status is M-Mode");
    println!("step2: switch mode to supervisor");

    MStatus::operate(|old| old.set_mpp(MPP::Supervisor));
    MEPC::operate(|old| old.set(test_supervisor_part as usize));
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
    MTVec::operate(|old| {
        old.set_addr(test_exception_handler as usize)
            .set_mode(MTVecMode::Direct)
    });

    mret::mret();
}

fn test_supervisor_part() {
    println!("mode switched. checking current cpu mode");
    ecall::ecall();

    assert_eq!(
        unsafe { TEST_SAVED_MPP }.expect("exception was not handled"),
        MPP::Supervisor,
    );

    println!("[b] step2 -- ok. current cpu status is S-Mode");

    unsafe {
        HANDLER_POINTER = default_exception_handler as usize;
    }

    unsafe { longjmp(&JMP_BUF, 10) };
}

#[no_mangle]
fn mode_switch_test_handler() {
    unsafe {
        TEST_SAVED_MPP = Some(MStatus::read().get_mpp());
    }

    MEPC::operate(|old| {
        let addr = old.get() + 4;
        old.set(addr)
    });
}
