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
fn uart_init() {
    unsafe{
        // disable the UART
        // UARTEN is control reg bit 0
        let mut scratch: u32 = read_volatile(UART_CR); 
        scratch &= !(1 << 0);
        write_volatile(UART_CR, scratch);
        
        // wait while busy
        spinlock(UART_FR, 1 << 5);

        // configure the LCRH
        // disable the fifos (lcrh bit 4)
        scratch = read_volatile(UART_LCRH); 
        scratch &= !(1 << 4);
        write_volatile(UART_LCRH, scratch);
        
        // configure word length
        // must be done with fifos disabled
        scratch = read_volatile(UART_LCRH);
        // 5th bit should be 0b11 for 1 byte word
        scratch |= (1 << 5);
        scratch |= (1 << 6);
        write_volatile(UART_LCRH, scratch);

        // reenable the fifos 
        scratch = read_volatile(UART_LCRH); 
        scratch |= (1 << 4);
        write_volatile(UART_LCRH, scratch);

        // configure the CR
        // disable RX and enable TX
        scratch = read_volatile(UART_CR);
        // rx off 
        scratch &= !(1 << 9);
        // tx on
        scratch |= (1 << 8);
        write_volatile(UART_CR, scratch);

        // renable the UART
        scratch = read_volatile(UART_CR); 
        scratch |= (1 << 0);
        write_volatile(UART_CR, scratch);
    }
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
