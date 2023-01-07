use core::panic;

use crate::{println, read_sp};

#[no_mangle]
pub unsafe extern "C" fn handle_default() {
    panic!("unhandled exception: unknown");
}

#[no_mangle]
pub unsafe extern "C" fn handle_undefined_instruction() {
    println!("in exception handler sp = {:#x}", read_sp());
    panic!("ABORTING: undefined instruction");
}

#[no_mangle]
pub unsafe extern "C" fn handle_swi() {
    println!("in swi handler sp = {:#x}", read_sp());
}

#[no_mangle]
pub unsafe extern "C" fn handle_prefetch_abrt() {
    println!("prefetch abort: breakpoint? continuing.");
    // loop {}
}

#[no_mangle]
pub unsafe extern "C" fn handle_data_abrt() {
    panic!("ABORTING: unhandled data abort");
}

#[allow(dead_code)]
extern "C" {
    fn _reset();
    // TODO implement these?
    fn handle_irq();
    fn handle_fiq();
}
