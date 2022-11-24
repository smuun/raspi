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

fn spinlock(ptr: *const u32, mask: u32) {
    unsafe{
        while !((read_volatile(ptr) & mask) == mask)  {}
    }
}
fn uart_init() {
}


fn uart_write(s: &str) {
    const oops: u8 = '?' as u8;
    for c in s.chars() {
        let out: u8 = c.is_ascii() ? c as u8 : oops;
        uart_writec(&out);
    }
}

fn uart_writec(c: &u8) {

    unsafe {
        write_volatile(UART_DR, *c);
    }
}
#[no_mangle]
pub extern "C" fn kernel_main() {
    /*
    mov r0, #0b0
    mov r1, #0b1

    // i think this isnt working??  not sure

    // UART CONFIG
    ldr r2, =CR_UARTEN
    str r0, [r2]  // disable the UART

    // wait for end of transmission / reception
    ldr r2, =FR_BUSY
    1:
        ldr r3, [r2]
        cmp r3, r1
        beq 1b

    // configure the LCRH
    ldr r2, =LCRH_FEN
    str r0, [r2] // flush the tx FIFO

    mov r4, #0b11
    ldr r2, =LCRH_WLEN
    str r4, [r2] // set the two bits at WLEN and WLEN + 1 (sets word length to 1 byte)

    ldr r2, =LCRH_FEN
    str r1, [r2] // reenable the tx FIFO

    // configure the CR
    ldr r2, =CR_RXE
    str r0, [r2] // unset RXE

    ldr r2, =CR_TXE
    str r1, [r2] // set TXE

    ldr r2, =CR_UARTEN
    str r1, [r2] // reenable the UART

    // write AAAAAAAAAAAAAAAAAAAAAAAAAA
    mov r5, #65
    ldr r2, =DR
    ldr r4, =FR_BUSY
    2:
        3:
            ldr r3, [r4]
            cmp r3, r1
            beq 3b

        str r5, [r2]
    b 2b

    bx lr"
    */
    let a: u8 = 65;
    uart_init();
    // FR_BUSY is at bit 5
    let busy_mask: u32 = 1 << 5;
    uart_write("Hello, world!");
}
