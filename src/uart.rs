use crate::utils::*;
use spin::Mutex;
use lazy_static::lazy_static;
use core::fmt;

// Raspi1 has peripheral base address 0x20000000
// (see refs peripheral refs for details)
const UART: u32 = 0x20201000;
const UART_DR: *mut u8 = (UART + 0x0) as *mut u8;
const UART_FR: *mut u32 = (UART + 0x18) as *mut u32;
const UART_CR: *mut u32 = (UART + 0x30) as *mut u32;
const UART_LCRH: *mut u32 = (UART + 0x2c) as *mut u32;

fn set_uarten(state: bool) {
    // disable the UART
    // UARTEN is control reg bit 0
    write_bit(UART_CR, 0, state);
}

fn set_fifos(state: bool) {
    // FEN (lcrh bit 4)
    write_bit(UART_LCRH, 4, state);
}

fn set_byte_wlen() {
    write_bit(UART_LCRH, 5, true);
    write_bit(UART_LCRH, 6, true);
}

fn set_rx(state: bool) {
    // rx bit 9
    write_bit(UART_CR, 9, state);
}

fn set_tx(state: bool) {
    // tx bit 8
    write_bit(UART_CR, 8, state);
}

/// Initialize the UART.  Sets the UART to tx mode.
pub fn uart_init() {
    // must disable before configuring the LCRH
    set_uarten(false);
    // wait for busy (tx and rx)
    spin_while(UART_FR, 1 << 5);
    spin_until(UART_FR, 1 << 4);
    set_fifos(false);
    set_byte_wlen();
    set_fifos(true);
    set_mode(UartMode::Tx);
}

/// Output a &str.  The UART must be initialized and in tx mode but can be busy.
pub fn uart_write(s: &str) {
    const OOPS: u8 = '?' as u8;
    spin_while(UART_FR, 1 << 5);
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

/// Set / change the UART mode.  
pub fn set_mode(m: UartMode) {
    // must disable before configuring the LCRH
    set_uarten(false);
    // wait for busy (tx and rx)
    spin_while(UART_FR, 1 << 5);
    spin_until(UART_FR, 1 << 4);
    // configure the CR
    match m {
        UartMode::Tx => {
            set_rx(false);
            set_tx(true);
        }
        UartMode::Rx => {
            set_rx(true);
            set_tx(false);
        }
    }
    set_uarten(true);
}

/// Get a character from the UART.  The UART must be initialized and in rx
/// mode.  The UART must not have a full rx fifo.
pub fn getc() -> u8 {
    spin_while(UART_FR, 1 << 4);
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
