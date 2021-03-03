use super::super::registers::{mepc::MEPC, CSRegister};

macro_rules! syscall_types {
    (
        pub enum $Typename: ident {
        $($name:ident = $num:expr, )*
        }
    ) => {
        #[derive(Debug, PartialEq, Clone, Copy)]
        pub enum $Typename {
            $($name = $num,)*
        }
        impl SyscallType {
            pub fn from(i: usize) -> Option<Self> {
                match i {
                    $($num => Some(Self::$name),)*
                    _ => None
                }
            }
        }
    }
}

syscall_types! {
    pub enum SyscallType {
        Write = 0,
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

    match SyscallType::from(syscall_type) {
        Some(cst) => match cst {
            SyscallType::Write => sys_write(a1 as *const c_void),
            _ => unimplemented!("Unimplemented system call."),
        },
        None => {
            unimplemented!("Unknown system call type.");
        }
    }

    // MEPC indicates address that THROWED exception.
    // so, return address is next to that.
    MEPC::operate(|old| {
        let t = old.get();
        old.set(t + 4)
    });

    0
}
