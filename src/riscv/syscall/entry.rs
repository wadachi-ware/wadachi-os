use super::super::registers::{mepc::MEPC, CSRegister};

pub enum SyscallType {
    Write = 0,
}

macro_rules! helper {
    ($x: expr, $member: ident) => {
        if $x == Self::$member as usize {
            return Some(Self::$member);
        }
    };
}

impl SyscallType {
    pub fn convert(x: usize) -> Option<Self> {
        helper!(x, Write);

        println!("{}", x);
        None
    }
}

#[allow(unused)]
pub extern "C" fn handle_ecall_from_s(
    syscall_type: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
    a7: usize,
) -> usize {
    use super::sys_write::sys_write;
    use core::ffi::c_void;

    match SyscallType::convert(syscall_type) {
        Some(cst) => match cst {
            SyscallType::Write => sys_write(a1 as *const c_void),
        },
        None => {
            unimplemented!("Unknown system call type!!");
        }
    }

    MEPC::operate(|old| {
        let t = old.get();
        old.set(t + 4)
    });

    0
}
