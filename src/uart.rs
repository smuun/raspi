use crate::*;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

// raspi1 has peripheral base address 0x20000000
const UART: u32 = 0x20201000;
const UART_DR: *mut u8 = (UART + 0x0) as *mut u8;
const UART_FR: *mut u32 = (UART + 0x18) as *mut u32;
const UART_CR: *mut u32 = (UART + 0x30) as *mut u32;
const UART_LCRH: *mut u32 = (UART + 0x2c) as *mut u32;


// TODO this singleton pattern from the embedded rust book 
// type SerialPort = ();
// struct Peripherals {
//     serial: Option<SerialPort>,
// }
// impl Peripherals {
//     fn take_serial(&mut self) -> SerialPort {
//     
//         let p = replace(&mut self.serial, None);
//         p.unwrap()
//     }
// }
// static mut PERIPHERALS: Peripherals = Peripherals {
//     serial: Some(SerialPort),
//
//     so: there is only one static mut;
//     when you take_serial you replace PERIPHS.serial with None
//
// };

/// Write a config value to the UART setup registers.
/// Set the config register at address 'register_base' to:
/// its current state, except that every bit in 'set' will
/// be set to its value in Value.
/// set & values & current config must be a valid uart config
/// panics if the write operation fails
fn configure(register_base: *mut u32, values: u32, set: u32) {
    unsafe {
        let mut word: u32 = read_volatile(register_base);
        // if a 'set' bit is high, change the current bit to
        // the corresponding Value
        word &= values & set;
        write_volatile(register_base, word);
        assert_eq!(read_volatile(register_base), word);
    }
}

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

/// wait until tx FIFO is not full
/// and rx FIFO is empty
fn spin_while_busy() {
    spin_while(UART_FR, TXFF);
    spin_until(UART_FR, RXFE);
}

/// Initialize the UART.  
pub fn uart_init() {
    // disable the UART
    configure(UART_CR, 0x0, UARTEN);
    // configure the LCRH:
    spin_while_busy();
    // disable fifos, set word length, reenable fifos
    configure(UART_LCRH, 0x0, FIFOSEN);
    configure(UART_LCRH, u32::MAX, BWLEN); 
    configure(UART_LCRH, u32::MAX, FIFOSEN);
}

/// Output a &str.  The UART must be initialized and in tx mode but can be busy.
pub fn uart_write(s: &str) {
    spin_until(UART_FR, RXFE);
    set_mode(UartMode::Tx);
    const OOPS: u8 = '?' as u8;
    spin_while(UART_FR, TXFF);
    for c in s.chars() {
        if c.is_ascii() {
            let tmp = c as u8;
            uart_writec(&tmp);
        } else {
            uart_writec(&OOPS);
        }
    }
}

pub enum UartMode {
    Rx,
    Tx,
}

/// Set / change the UART mode
/// and wait until ready
pub fn set_mode(m: UartMode) {
    // disable before configuring the LCRH
    configure(UART_CR, 0x0, UARTEN);
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
pub fn getc() -> u8 {
    spin_while(UART_FR, TXFF);
    set_mode(UartMode::Rx);
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
pub fn uart_writec(c: &u8) {
    unsafe {
        write_volatile(UART_DR, *c);
    }
}

pub struct Writer {}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        uart_write(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {});
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

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn uart_should_init() {
        uart_init();
    }

    #[test_case]
    fn uart_should_print() {
        uart_write("Hello!");
    }

}
