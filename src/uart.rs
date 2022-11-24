use crate::utils::*;

// Raspi1 has peripheral base address 0x20000000
// (see refs peripheral refs for details)
const UART: u32 = 0x20201000;
const UART_DR: *mut u8 = (UART + 0x0) as *mut u8;
const UART_FR: *mut u32 = (UART + 0x18) as *mut u32;
const UART_CR: *mut u32 = (UART + 0x30) as *mut u32;
const UART_LCRH: *mut u32 = (UART + 0x2c) as *mut u32;

fn write_bit(base: *mut u32, offset: u8, bit: bool) {
    unsafe {
        let mut scratch: u32 = read_volatile(base);
        if bit {
            scratch |= 1 << offset;
        } else {
            scratch &= !(1 << offset);
        }
        write_volatile(UART_CR, scratch);
    }
}

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
    // rx byte 9
    write_bit(UART_CR, 9, state);
}

fn set_tx(state: bool) {
    // rx byte 9
    write_bit(UART_CR, 9, state);
}

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

pub fn write(s: &str) {
    const OOPS: u8 = '?' as u8;
    spin_while(UART_FR, 1 << 5);
    for c in s.chars() {
        if c.is_ascii() {
            let tmp = c as u8;
            writec(&tmp);
        } else {
            writec(&OOPS);
        }
    }
}

pub enum UartMode {
    Rx,
    Tx,
}
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

pub fn writec(c: &u8) {
    unsafe {
        write_volatile(UART_DR, *c);
    }
}
