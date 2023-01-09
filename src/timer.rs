// hardware address
// 0x7E00B000
// 0x2000b000
// ARM peripherals doc page 112

// Let's start with normal interrupts
// the ARM also supports fast interrupts (FIQ)

use core::{arch::asm};

use crate::{configure, poll};

/// **important** software addess for `0x7e...` is `0x20...`
const IR_BASE: u32 = 0x2000b000;
const IRQ_BASIC_PEND: *mut u32 = (IR_BASE + 0x200) as *mut u32;

// appears to be same as IR base...
// page 196
const TIMER_BASE: u32 = 0x2000b000;
// in one place it says 40c, another 408
const TIMER_LOAD: *mut u32 = (TIMER_BASE + 0x400) as *mut u32;
const TIMER_CONTROL: *mut u32 = (TIMER_BASE + 0x408) as *mut u32;
const TIMER_CLEAR: *mut u32 = (TIMER_BASE + 0x40c) as *mut u32;
const TIMER_RELOAD: *mut u32 = (TIMER_BASE + 0x418) as *mut u32;

pub fn enable_timer_interrupts() {
    // this from blinker05.c.  I checked, and it matches what I got from the
    // manual exactly, but still... PUT32(ARM_TIMER_CTL,0x003E0000);
    // PUT32(ARM_TIMER_LOD,4000000-1);
    // PUT32(ARM_TIMER_RLD,4000000-1);
    // PUT32(ARM_TIMER_CLI,0);
    // icount=0;
    // enable_irq() (which is asm:)
    // mrs r0,cpsr
    // bic r0,r0,#0x80
    // msr cpsr_c,r0
    // bx lr
    // PUT32(ARM_TIMER_CTL,0x003E00A2);
    // PUT32(ARM_TIMER_CLI,0);

    configure(TIMER_CONTROL, 0x003e0000, u32::MAX);
    configure(TIMER_LOAD, 04000000 - 1, u32::MAX);
    configure(TIMER_RELOAD, 4000000 - 1, u32::MAX);
    configure(TIMER_CLEAR, 0, u32::MAX);
    unsafe {
        asm!("
            mrs r0, cpsr
            bic r0, r0, #0x80
            msr cpsr_c, r0
        ", out("r0") _
        );
    }
    configure(TIMER_CONTROL, 0x003e00a2, u32::MAX);
    configure(TIMER_CLEAR, 0, u32::MAX);
}

pub fn poll_timer_irq() -> bool {
    // timer is at bit 0 of the IRQ pending base
    poll(IRQ_BASIC_PEND, 1 << 0, 1 << 0)
}
