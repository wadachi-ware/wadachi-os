use lazy_static::lazy_static;
use spin::Mutex;

pub struct UARTBuffer {
    address: &'static mut u8,
}

use core::fmt;
use core::fmt::{Error, Write};

const QEMU_UART_ADDRESS: *mut u8 = 0x1000_0000 as *mut u8;

lazy_static! {
    pub static ref UART_BUF: Mutex<UARTBuffer> =
        Mutex::new(UARTBuffer::new(unsafe { &mut *QEMU_UART_ADDRESS }));
}

impl UARTBuffer {
    pub fn new(address: &'static mut u8) -> Self {
        Self { address }
    }
}

impl Write for UARTBuffer {
    fn write_str(&mut self, msg: &str) -> Result<(), Error> {
        for c in msg.chars() {
            *self.address = c as u8;
        }
        Ok(())
    }
}

#[no_mangle]
pub fn _print(args: fmt::Arguments) {
    UART_BUF.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::stdio::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
