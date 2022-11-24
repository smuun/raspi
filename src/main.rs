#![no_std]
#![no_main]
#![feature(core_intrinsics, lang_items)]

use core::ptr::read_volatile;
use core::ptr::write_volatile;

// The "bootloader" code is top level in setup. Including mod setup runs it.
mod setup;
mod utils;

// Raspi1 has peripheral base address 0x20000000
// (see refs peripheral refs for details)
const UART: u32 = 0x20201000;
const UART_DR: *mut u8 = (UART + 0x0) as *mut u8;
const UART_FR: *mut u32 = (UART + 0x18) as *mut u32;
const UART_CR: *mut u32 = (UART + 0x30) as *mut u32;
const UART_LCRH: *mut u32 = (UART + 0x2c) as *mut u32;
/*
.set LCRH_FEN    (LCRH + 0x04)
.set LCRH_WLEN   (LCRH + 0x05)       //2 bits -- 5 and 6
.set FR_BUSY     (FR   + 0X03)
.set CR_UARTEN,   (CR   + 0x00)
.set CR_TXE,      (CR   + 0x08)
.set CR_RXE,      (CR   + 0x09)
 */

// spin while the busy bit at mask is set
fn spinlock(ptr: *const u32, mask: u32) {
    unsafe{
        while (((read_volatile(ptr)) & mask) == mask)  {}
    }
}

fn write_bit(base: *mut u32, offset: u8, bit: bool) {
    unsafe {
        let mut scratch: u32 = read_volatile(base); 
        if bit {
            scratch |= (1 << offset);
        } else {
            scratch &= !(1 << offset);
        }
        write_volatile(UART_CR, scratch);
    }
}

fn set_uart(state: bool) {
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

fn uart_init() {
        // must disable before configuring the LCRH
        set_uart(false);
        // wait for busy
        spinlock(UART_FR, 1 << 5);
        // configure the LCRH
        set_fifos(false);
        set_byte_wlen();
        set_fifos(true);
        // configure the CR
        set_rx(false);
        set_tx(true);
        set_uart(true);
}


fn uart_write(s: &str) {
    const oops: u8 = '?' as u8;
    for c in s.chars() {
        if c.is_ascii() {
            let tmp = c as u8;
            uart_writec(&tmp);
        } else {
            uart_writec(&oops);
        }
    }
}

fn uart_writec(c: &u8) {
    unsafe {
        write_volatile(UART_DR, *c);
    }
}
#[no_mangle]
pub extern "C" fn kernel_main() {
    let a: u8 = 65;
    uart_init();
    // wait until bit 5 (tx_full) is clear
    spinlock(UART_FR, 1 << 5);
    uart_write("Hello, world!");
}
