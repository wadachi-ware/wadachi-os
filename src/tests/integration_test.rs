use crate::riscv::{
    helper::setjmp::{longjmp, setjmp, RISCV32RawJumpBuffer},
    instructions::{ecall, mret},
    registers::{
        mepc::MEPC,
        mstatus::{MStatus, MPP},
        mtvec::{MTVec, MTVecMode},
        pmpaddr::*,
        pmpcfg::{AddressMatching, PMPCfg},
        satp::{MODE32, SATP},
        CSRegister,
    },
};
use core::ptr::{read_volatile, write_volatile};
use custom_test::custom_test;

static mut TEST_JMP_BUFFER: RISCV32RawJumpBuffer = RISCV32RawJumpBuffer::default();

#[custom_test(IntegrationMachineToSupervisor)]
fn switch_mode_test_from_machine_to_supervisor() {
    let mut flag = false;

    println!("Switching mode from machine to supervisor");
    println!("step1: check current mode");

    let ret = unsafe { setjmp(&mut TEST_JMP_BUFFER) };
    match ret {
        0 => {
            assert_eq!(unsafe { read_volatile(&flag) }, false);
            MTVec::operate(|old| {
                old.set_addr(jmp_relay_machine as usize)
                    .set_mode(MTVecMode::Direct)
            });

            unsafe {
                write_volatile(&mut flag, true);
            }

            ecall::ecall();
        }
        x if x == MPP::Machine as usize + 10 => {
            assert_eq!(unsafe { read_volatile(&flag) }, true);
            println!("In machine mode. step1: ok");
        }
        _ => {
            panic!("unknown setjmp return code in step1");
        }
    }

    println!("step2: switch mode");

    MStatus::operate(|old| old.set_mpp(MPP::Supervisor).set_spie(true));

    MEPC::operate(|old| old.set(for_test_supervisor as usize));

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

fn for_test_supervisor() {
    let mut flag = false;

    println!("mret. check current mode");
    let ret = unsafe { setjmp(&mut TEST_JMP_BUFFER) };
    match ret {
        0 => {
            assert_eq!(unsafe { read_volatile(&flag) }, false);
            unsafe {
                write_volatile(&mut flag, true);
            }
            ecall::ecall();
        }
        x if x == MPP::Supervisor as usize + 10 => {
            assert_eq!(unsafe { read_volatile(&flag) }, true);
            println!("In Supervisor mode. step2: ok");
        }
        c => {
            panic!("unknown setjmp return code in step2. got {}", c);
        }
    }
    println!("mode switching all ok");
}

#[allow(dead_code)]
fn jmp_relay_machine() -> ! {
    unsafe {
        llvm_asm!(".align 4");
    }
    unsafe { longjmp(&TEST_JMP_BUFFER, MStatus::read().get_mpp() as usize + 10) };
}
