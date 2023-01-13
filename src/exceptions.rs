use core::panic;

use crate::{error, log, read_sp, sys_timer::{disable_timer_interrupts, set_timer, TimerID, clear_timer_interrupts, enable_timer_interrupts}};

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
    log!("handling IRQ");
    sys_tick();
}

fn sys_tick() {
    // disable the interrupt
    disable_timer_interrupts();
    // clear the pending interrupt
    clear_timer_interrupts(TimerID::One);
    // add some time to the counter
    // tick
    set_timer(TimerID::One, u32::MAX / 2 );
    log!("tick");
    // return 
}

#[allow(dead_code)]
extern "C" {
    // TODO implement these?
    fn handle_fiq();
}
