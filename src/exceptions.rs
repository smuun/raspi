use core::panic;

use crate::{error, log, println, read_sp, warn};

#[no_mangle]
pub unsafe extern "C" fn handle_default() {
    panic!("unhandled exception: unknown");
}

#[no_mangle]
pub unsafe extern "C" fn handle_undefined_instruction() {
    log!("in exception handler sp = {:#x}", read_sp());
    panic!("ABORTING: undefined instruction");
}

#[no_mangle]
pub unsafe extern "C" fn handle_swi() {
    log!("in swi handler sp = {:#x}", read_sp());
}

#[no_mangle]
pub unsafe extern "C" fn handle_prefetch_abrt() {
    log!("prefetch abort: breakpoint? ignoring.");
    // loop {}
}

#[no_mangle]
pub unsafe extern "C" fn handle_data_abrt() {
    panic!("ABORTING: unhandled data abort");
}
#[no_mangle]
pub unsafe extern "C" fn handle_irq() {
    log!("handling IRQ");
    loop {}
}

#[allow(dead_code)]
extern "C" {
    fn _reset();
    // TODO implement these?
    fn handle_fiq();
}
