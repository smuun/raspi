use core::{panic, ptr::read_volatile};

use crate::{
    error, log, read_sp,
    sys_timer::{
        clear_timer_interrupts, disable_timer_interrupts,
        enable_timer_interrupts, set_timer, TimerID, IRQ_1_PEND,
    },
    warn,
};

static mut COUNT: u32 = 0;
pub const TIMER_INC: u32 = 6000000;

#[no_mangle]
pub unsafe extern "C" fn handle_default() {
    panic!("unhandled exception: unknown");
}

#[no_mangle]
pub unsafe extern "C" fn handle_undefined_instruction() {
    log!("in exception handler sp = {:#x}", read_sp());
    error!("ignoring undefined instruction");
}

#[no_mangle]
pub unsafe extern "C" fn handle_swi() {
    log!("in swi handler sp = {:#x}", read_sp());
    log!("ignoring swi");
}

#[no_mangle]
pub unsafe extern "C" fn handle_prefetch_abrt() {
    log!("ignoring prefetch abort: breakpoint?");
}

#[no_mangle]
pub unsafe extern "C" fn handle_data_abrt() {
    panic!("ABORTING: unhandled data abort");
}
#[no_mangle]
pub unsafe extern "C" fn handle_irq() {
    match get_irq_reason() {
        IRQReason::SysTimer => {
            tick();
        }
        IRQReason::Other => {
            warn!("ignoring unknown or multiple IRQ");
        }
    }
}

enum IRQReason {
    SysTimer,
    Other,
}

fn get_irq_reason() -> IRQReason {
    let x: u32;
    unsafe {
        x = read_volatile(IRQ_1_PEND);
    }
    // I think the lowest 4 bits all correspond to a system timer
    // but this is undocumented.  In any case it seems clear that
    // bit 1 corresponds to timer 1.
    if (x & 0b1111) != 0 {
        return IRQReason::SysTimer;
    }
    IRQReason::Other
}

fn tick() {
    disable_timer_interrupts();
    clear_timer_interrupts(TimerID::One);
    set_timer(TimerID::One, TIMER_INC);
    unsafe {
        let msg: &str;
        if COUNT % 2 == 0 {
            msg = "tick"
        } else {
            msg = "tock"
        }
        log!("{}", msg);
        COUNT += 1;
    }
    enable_timer_interrupts();
}

#[allow(dead_code)]
extern "C" {
    // TODO implement these?
    fn handle_fiq();
}
