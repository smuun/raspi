// hardware address
// 0x7E00B000
// 0x2000b000
// ARM peripherals doc page 112

// Let's start with normal interrupts
// the ARM also supports fast interrupts (FIQ)

use core::ptr::write_volatile;

use crate::{configure, poll};

/// **important** software addess for `0x7e...` is `0x20...`
const IR_BASE: u32 = 0x2000b000;
const IRQ_BASIC_PEND: *mut u32 = (IR_BASE + 0x200) as *mut u32;
const IRQ_BASIC_EN: *mut u32 = (IR_BASE + 0x210) as *mut u32;
const IRQ_BASIC_DIS: *mut u32 = (IR_BASE + 0x21c) as *mut u32;

// appears to be same as IR base...
// page 196
const TIMER_BASE: u32 = 0x2000b000;
// in one place it says 40c, another 408
const TIMER_LOAD: *mut u32 = (TIMER_BASE + 0x400) as *mut u32;
const TIMER_VAL: *mut u32 = (TIMER_BASE + 0x404) as *mut u32;
const TIMER_CONTROL: *mut u32 = (TIMER_BASE + 0x408) as *mut u32;
const TIMER_CLEAR: *mut u32 = (TIMER_BASE + 0x40c) as *mut u32;
const TIMER_RIS: *mut u32 = (TIMER_BASE + 0x410) as *mut u32;
const TIMER_MIS: *mut u32 = (TIMER_BASE + 0x414) as *mut u32;
const TIMER_RELOAD: *mut u32 = (TIMER_BASE + 0x418) as *mut u32;
const TIMER_DIVIDE: *mut u32 = (TIMER_BASE + 0x41c) as *mut u32;
const TIMER_CNT: *mut u32 = (TIMER_BASE + 0x420) as *mut u32;


pub fn enable_timer_interrupts() {
    // allow interrupts
    configure(IRQ_BASIC_EN, 1 << 0, 1 << 0);

    // reset the register
    // PUT32(ARM_TIMER_CTL,0x003E0000);
    // PUT32(ARM_TIMER_LOD,);
    // PUT32(ARM_TIMER_RLD,1000000-1);
    // PUT32(ARM_TIMER_DIV,0x000000F9);
    // PUT32(ARM_TIMER_CLI,0);
    // PUT32(ARM_TIMER_CTL,0x003E00A2);
    // load the timer with one clock

    // this is reset, but i think it should be 2000?
    configure(TIMER_CONTROL, 0x003e0000, u32::MAX);
    configure(TIMER_LOAD, 1000000-1, u32::MAX);
    configure(TIMER_RELOAD, 1000000-1, u32::MAX);
    configure(TIMER_DIVIDE, 0xf9, u32::MAX);
    configure(TIMER_CLEAR, 0, u32::MAX);
    configure(TIMER_CONTROL, 0x003e00A2, u32::MAX);

    // enable timer
    // bit 1: 1 => 23 bit, 0 => 16 bit
    // bit 5: interrupt enabled
    // bit 7: timer enabled
    // const SEL: u32 = 1 << 1 | 1 << 5 | 1 << 7;
    // const CFG: u32 = 0 << 1 | 1 << 5 | 1 << 7;
    // configure(TIMER_CONTROL, CFG,  SEL);
    // load some time -- here we want to set all the bits
}

// reading time interrupts
// timer is at bit 0 of the IRQ pending base
// irq pend base at 0x200 [bit 0]
pub fn poll_timer_irq() -> bool {
    poll(IRQ_BASIC_PEND, 1 << 0, 1 << 0)
}

// begin by enabling timer interrupts

// enabling the timer
// IRQ enable 3
// address 0x218
// set bit 0
// disable: same at 0x21c
