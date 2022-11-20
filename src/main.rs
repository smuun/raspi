#![no_std]
#![no_main]
#![feature(core_intrinsics, lang_items)]

use core::intrinsics::volatile_load;
use core::intrinsics::volatile_store;

// raspi1 has peripheral base address 0x20000000
// see refs for details
const UART_DR: u32 = 0x20201000;
const UART_FR: u32 = 0x20201018;

fn mmio_write(reg: u32, val: u32) {
    unsafe { volatile_store(reg as *mut u32, val) }
}

fn mmio_read(reg: u32) -> u32 {
    unsafe { volatile_load(reg as *const u32) }
}

fn transmit_fifo_full() -> bool {
    mmio_read(UART_FR) & (1 << 5) > 0
}

fn receive_fifo_empty() -> bool {
    mmio_read(UART_FR) & (1 << 4) > 0
}

fn writec(c: u8) {
    while transmit_fifo_full() {}
    mmio_write(UART_DR, c as u32);
}

fn getc() -> u8 {
    while receive_fifo_empty() {}
    mmio_read(UART_DR) as u8
}

fn write(msg: &str) {
    for c in msg.chars() {
        writec(c as u8)
    }
}

#[no_mangle]
pub extern fn kernel_main() {
    write("Hello Rust Kernel world!");
    loop {
        writec(getc())
    }
}
