#![no_std]
#![no_main]
#![feature(core_intrinsics, lang_items)]

use core::ptr::write_volatile;
use core::ptr::read_volatile;
use core::arch::asm;

// The "bootloader" code is top level in setup. Including mod setup runs it.
mod setup;
mod utils;

// Raspi1 has peripheral base address 0x20000000
// (see refs peripheral refs for details)
const UART: u32 = 0x20201000;
const UART_DR: u32 = UART + 0x0;
const UART_FR: u32 = UART + 0x18;
const UART_CR: u32 = UART + 0x30;
const UART_LCRH: u32 = UART + 0x2c;
/*
.set LCRH_FEN    (LCRH + 0x04)
.set LCRH_WLEN   (LCRH + 0x05)       //2 bits -- 5 and 6
.set FR_BUSY     (FR   + 0X03)
.set CR_UARTEN,   (CR   + 0x00)
.set CR_TXE,      (CR   + 0x08)
.set CR_RXE,      (CR   + 0x09)
 */ 

fn uart_init() {
}
fn uart_writec(c: &u8) {
    unsafe{
        write_volatile(UART_DR as *mut u8, *c);
    }
}

#[no_mangle]
pub extern fn kernel_main() {
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
    loop {
        uart_writec(&a);
    }
}
