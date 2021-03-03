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

macro_rules! enum_with_from {
    (
        pub enum $TypeName: ident {
            $($name: ident = $num: expr,)*
        }
    ) => {
        #[allow(unused)]
        #[derive(Debug, PartialEq, Clone, Copy)]
        pub enum $TypeName {
            $($name = $num,)*
        }
        impl $TypeName {
            #[allow(unused)]
            pub fn from(i: usize) -> Option<Self> {
                match i {
                    $($num => Some(Self::$name),)*
                    _ => None,
                }
            }
        }
    }
}

enum_with_from!(
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
);

enum_with_from!(
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
);

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
                TrapType::Interrupt(match InterruptType::from(self.value.get_bits(0..32)) {
                    Some(x) => x,
                    None => panic!("Unknown exception code!"),
                })
            }
            false => {
                // Exception
                TrapType::Exception(match ExceptionType::from(self.value.get_bits(0..32)) {
                    Some(x) => x,
                    None => panic!("Unknown interrupt code!"),
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
