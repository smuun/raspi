use core::{fmt, ptr::replace};

use lazy_static::lazy_static;
use spin::Mutex;

use crate::*;

// raspi1 has peripheral base address 0x20000000
const UART_BASE: u32 = 0x20201000;
const UART_DR: *mut u8 = (UART_BASE + 0x0) as *mut u8;
const UART_FR: *mut u32 = (UART_BASE + 0x18) as *mut u32;
const UART_CR: *mut u32 = (UART_BASE + 0x30) as *mut u32;
const UART_LCRH: *mut u32 = (UART_BASE + 0x2c) as *mut u32;
// CR
const UARTEN: u32 = 1 << 0;
const FIFOSEN: u32 = 1 << 4;
const TX: u32 = 1 << 8;
const RX: u32 = 1 << 9;
// LCRH
const BWLEN: u32 = (1 << 4) | (1 << 5);
// FR
// tx FIFO full
const TXFF: u32 = 1 << 5;
// rx FIFO empty
const RXFE: u32 = 1 << 4;

pub enum UartMode {
    Rx,
    Tx,
}

impl Uart {
    /// Initialize the UART
    fn config_uart_init(&self) {
        // disable the UART
        configure(UART_CR, 0x0, UARTEN);
        // configure the LCRH:
        spin_while(UART_FR, TXFF);
        spin_until(UART_FR, RXFE);
        // disable fifos, set word length, reenable fifos
        configure(UART_LCRH, 0x0, FIFOSEN);
        configure(UART_LCRH, u32::MAX, BWLEN);
        configure(UART_LCRH, u32::MAX, FIFOSEN);
    }

    /// Output a &str.  The UART must be initialized and in tx mode but can be
    /// busy.
    fn uart_write(&self, s: &str) {
        self.set_mode(UartMode::Tx);
        const OOPS: u8 = '?' as u8;
        spin_while(UART_FR, TXFF);
        for c in s.chars() {
            if c.is_ascii() {
                let tmp = c as u8;
                self.uart_writec(&tmp);
            } else {
                self.uart_writec(&OOPS);
            }
        }
    }

    /// Set / change the UART mode
    /// and wait until ready
    fn set_mode(&self, m: UartMode) {
        // disable before configuring the LCRH
        configure(UART_CR, 0x0, UARTEN);
        spin_while(UART_FR, TXFF);
        spin_until(UART_FR, RXFE);
        match m {
            UartMode::Tx => {
                configure(UART_CR, TX, TX | RX);
            }
            UartMode::Rx => {
                configure(UART_CR, RX, TX | RX);
            }
        }
        configure(UART_CR, u32::MAX, UARTEN);
    }

    /// Get a character from the UART.  The UART must be initialized and in rx
    /// mode.  The UART must not have a full rx fifo.
    fn getc(&self) -> u8 {
        // wait until I enter a character ?? why this order
        self.set_mode(UartMode::Rx);
        spin_while(UART_FR, RXFE);
        let mut c;
        unsafe {
            c = read_volatile(UART_DR);
        }
        if !(c as char).is_ascii() {
            c = '?' as u8;
        }
        c
    }

    /// Print a character to the UART.  The UART must be initialized and in tx
    /// mode.  The UART must not have a full tx fifo.
    fn uart_writec(&self, c: &u8) {
        unsafe {
            write_volatile(UART_DR, *c);
        }
    }
}

lazy_static! {
    pub static ref UART: Mutex<Uart> =
        Mutex::new(unsafe { PERIPHERALS.take_uart() });
}

pub struct Uart {}

impl Peripherals {
    unsafe fn take_uart(&mut self) -> Uart {
        // init the physical UART
        // and return it
        let p = replace(&mut self.uart, None);
        let r = p.unwrap();
        r.config_uart_init();
        r.set_mode(UartMode::Tx);
        r
    }
}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.uart_write(s);
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    UART.lock().write_fmt(args).unwrap();
}
#[doc(hidden)]
pub fn _getc() -> u8 {
    UART.lock().getc()
}

#[macro_export]
macro_rules! readc {
    () => {
        $crate::uart::_getc();
    };
}
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::uart::_print(format_args!($($arg)*)));
}
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[cfg(test)]
mod tests {
    // use super::*;
    #[test_case]
    fn todo() {}
}
