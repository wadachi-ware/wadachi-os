use super::CSRegister;
use bit_field::BitField;

#[allow(unused)]
#[derive(Debug, PartialEq, Default, Clone)]
pub struct MCause {
    value: usize,
}

#[allow(unused)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TrapType {
    Exception(ExceptionType),
    Interrupt(InterruptType),
}

#[allow(unused)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ExceptionType {
    // see p35. https://riscv.org/wp-content/uploads/2017/05/riscv-privileged-v1.10.pdf
    InstructionAddressMisaligned = 0,
    InstructionAccessFault = 1,
    IllegalInstruction = 2,
    BreakPoint = 3,
    LoadAddressMisaligned = 4,
    LoadAccessFault = 5,
    StoreAMOAddressMisaligned = 6,
    StoreAMOAccessFault = 7,
    EnvironmentCallFromUMode = 8,
    EnvironmentCallFromSMode = 9,
    // 10: Reserved
    EnvironmentCallFromMMode = 11,
    InstructionPageFault = 12,
    LoadPageFault = 13,
    // 14: Reserved
    StoreAMOPageFault = 15,
}

macro_rules! helper {
    ($x: ident, $member: ident) => {
        if $x == Self::$member as usize {
            return Ok(Self::$member);
        }
    };
}

impl ExceptionType {
    fn convert(x: usize) -> Result<ExceptionType, ()> {
        helper!(x, InstructionAddressMisaligned);
        helper!(x, InstructionAccessFault);
        helper!(x, IllegalInstruction);
        helper!(x, BreakPoint);
        helper!(x, LoadAddressMisaligned);
        helper!(x, LoadAccessFault);
        helper!(x, StoreAMOAddressMisaligned);
        helper!(x, StoreAMOAccessFault);
        helper!(x, EnvironmentCallFromUMode);
        helper!(x, EnvironmentCallFromSMode);
        helper!(x, EnvironmentCallFromMMode);
        helper!(x, InstructionPageFault);
        helper!(x, LoadPageFault);
        helper!(x, StoreAMOPageFault);

        Err(())
    }
}

#[allow(unused)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum InterruptType {
    // see p35. https://riscv.org/wp-content/uploads/2017/05/riscv-privileged-v1.10.pdf
    UserSoftwareInterrupt = 0,
    SupervisorSoftwareInterrupt = 1,
    // 2: Reserved
    MachineSoftwareInterrupt = 3,

    UserTimerInterrupt = 4,
    SupervisorTimerInterrupt = 5,
    // 6: Reserved
    MachineTimerInterrupt = 7,

    UserExternalInterrupt = 8,
    SupervisorExternalInterrupt = 9,
    // 10: Reserved
    MachineExternalInterrupt = 10,
}

impl InterruptType {
    fn convert(x: usize) -> Result<InterruptType, ()> {
        helper!(x, UserSoftwareInterrupt);
        helper!(x, SupervisorSoftwareInterrupt);
        helper!(x, MachineSoftwareInterrupt);
        helper!(x, UserTimerInterrupt);
        helper!(x, SupervisorTimerInterrupt);
        helper!(x, MachineTimerInterrupt);
        helper!(x, UserExternalInterrupt);
        helper!(x, SupervisorExternalInterrupt);
        helper!(x, MachineExternalInterrupt);

        Err(())
    }
}

impl MCause {
    #[allow(unused)]
    #[inline]
    pub fn get_interrupt(&self) -> usize {
        self.value.get_bits(31..=31)
    }
    #[allow(unused)]
    #[inline]
    pub fn get_code(&self) -> usize {
        // TODO: Use 'exception code enum' instead.
        self.value.get_bits(0..31)
    }
    #[allow(unused)]
    #[inline]
    pub fn get_trap_type(&self) -> TrapType {
        match self.value.get_bit(31) {
            // see p34. https://riscv.org/wp-content/uploads/2017/05/riscv-privileged-v1.10.pdf
            // Check MSB of mcause
            true => {
                // Interrupt
                TrapType::Interrupt(match InterruptType::convert(self.value.get_bits(0..32)) {
                    Ok(x) => x,
                    Err(_) => panic!("Unknown exception code!"),
                })
            }
            false => {
                // Exception
                TrapType::Exception(match ExceptionType::convert(self.value.get_bits(0..32)) {
                    Ok(x) => x,
                    Err(_) => panic!("Unknown interrupt code!"),
                })
            }
        }
    }
}

impl CSRegister for MCause {
    unsafe fn write(_: Self) {
        panic!("not writable!");
        // TODO: Fix CSRegister.
    }
    fn read() -> Self {
        read_csr!("mcause");
        Self {
            value: internal_read(),
        }
    }
}
