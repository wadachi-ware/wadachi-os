use super::CSRegister;

#[derive(PartialEq, Default, Clone, Debug)]
pub struct MTVal {
    value: usize,
}

impl MTVal {
    #[allow(unused)]
    #[inline]
    pub fn get_value(&self) -> usize {
        self.value
    }
    #[allow(unused)]
    #[inline]
    pub fn set_value(mut self, value: usize) -> Self {
        // TODO: is this needs?
        self.value = value;
        self
    }
}

impl CSRegister for MTVal {
    unsafe fn write(stvec: Self) {
        write_csr!("stvec");
        internal_write(stvec.value);
    }
    fn read() -> Self {
        read_csr!("stvec");
        Self {
            value: internal_read(),
        }
    }
}
