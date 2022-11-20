#![no_std]
#![no_main]
#![feature(core_intrinsics, lang_items)]

use core::intrinsics::volatile_load;
use core::intrinsics::volatile_store;
use core::panic::PanicInfo;

// raspi1 has peripheral base address 0x20000000
// see refs for details
const UART_DR: u32 = 0x20201000;
const UART_FR: u32 = 0x20201018;

mod setup {
    use core::arch::global_asm;
    // what would typically be in boot.S.
    // Copied from osdev ref
    global_asm!(r#"
    // AArch32 mode
     
    // To keep this in the first portion of the binary.
    .section ".text.boot"
     
    // Make _start global.
    .globl _start
     
            .org 0x8000
    // Entry point for the kernel.
    // r15 -> should begin execution at 0x8000.
    // r0 -> 0x00000000
    // r1 -> 0x00000C42 - machine id
    // r2 -> 0x00000100 - start of ATAGS
    // preserve these registers as argument for kernel_main
    _start:
        // Setup the stack.
        mov sp, #0x8000
     
        // Clear out bss.
        ldr r4, =__bss_start
        ldr r9, =__bss_end
        mov r5, #0
        mov r6, #0
        mov r7, #0
        mov r8, #0
        b       2f
     
    1:
        // store multiple at r4.
        stmia r4!, {{r5-r8}}
     
        // If we are still below bss_end, loop.
    2:
        cmp r4, r9
        blo 1b
     
        // Call kernel_main
        ldr r3, =kernel_main
        blx r3
     
        // halt
    halt:
        b halt
    "#);
}

#[no_mangle]
pub extern fn kernel_main() {
    write("Hello Rust Kernel world!");
    loop {
        writec(getc())
    }
}



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


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
