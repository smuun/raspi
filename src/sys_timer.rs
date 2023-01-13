// Use the system timer
use core::ptr::read_volatile;

use crate::{configure, poll};

/// **important** software addess for `0x7e...` is `0x20...`
const IR_BASE: u32 = 0x2000b000;
const IRQEN_1: *mut u32 = (IR_BASE + 0x210) as *mut u32;
const IRQDIS_1: *mut u32 = (IR_BASE + 0x21c) as *mut u32;
const IRQ_1_PEND: *mut u32 = (IR_BASE + 0x204) as *mut u32;

// periphs page 172
const TIMER_BASE: u32 = 0x20003000;

const TIMER_CS: *mut u32 = (TIMER_BASE + 0x0) as *mut u32;
// running counter: lower 32, higher 32
const TIMER_CTLO: *mut u32 = (TIMER_BASE + 0x4) as *mut u32;
const TIMER_CTHI: *mut u32 = (TIMER_BASE + 0x8) as *mut u32;
const TIMER_CMP0: *mut u32 = (TIMER_BASE + 0xc) as *mut u32;
const TIMER_CMP1: *mut u32 = (TIMER_BASE + 0x10) as *mut u32;
const TIMER_CMP2: *mut u32 = (TIMER_BASE + 0x14) as *mut u32;
const TIMER_CMP3: *mut u32 = (TIMER_BASE + 0x18) as *mut u32;

pub fn init_timer() {
    // reset everything
    for reg in [
        TIMER_CS, TIMER_CTLO, TIMER_CTHI, TIMER_CMP0, TIMER_CMP1, TIMER_CMP2,
        TIMER_CMP3,
    ] {
        configure(reg, 0x0, u32::MAX);
    }
    set_timer(TimerID::One, 20000);
    enable_timer_interrupts();
}

pub fn enable_timer_interrupts() {
    // enable timer IRQs
    configure(IRQEN_1, 1 << 1, 1 << 1);
}

pub fn timer_irq_active() -> bool {
    poll(IRQ_1_PEND, 1 << 1, 1 << 1)
}

pub enum TimerID {
    One,
}

pub fn set_timer(_id: TimerID, value: u32) {
    let mut time: u32;
    unsafe {
        time = read_volatile(TIMER_CTLO);
    }
    time += value;
    configure(TIMER_CMP1, time, u32::MAX);
}

pub fn clear_timer_interrupts(_id: TimerID) {
    // assuming it's timer 1: clear the interrupt
    configure(TIMER_CS, 0x0, 1 << 1);
}

pub fn disable_timer_interrupts() {
    configure(IRQDIS_1, 1 << 1, 1 << 1);
}
