pub struct UARTBuffer {
    address: *mut u8,
}

use core::fmt::{Error, Write};

impl UARTBuffer {
    pub fn new(address: *mut u8) -> Self {
        Self { address }
    }
}

impl Write for UARTBuffer {
    fn write_str(&mut self, msg: &str) -> Result<(), Error> {
        for c in msg.chars() {
            unsafe {
                *self.address = c as u8;
            }
        }
        Ok(())
    }
}
