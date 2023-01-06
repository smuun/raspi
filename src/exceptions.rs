use crate::{println, read_sp, uart::uart_init};
use core::panic;

#[no_mangle]
pub unsafe extern "C" fn handle_default() {
    uart_init();
    panic!("unhandled exception: unknown");
}

#[no_mangle]
pub unsafe extern "C" fn handle_undefined_instruction() {
    uart_init();
    println!("in exception handler sp = {:#x}", read_sp());
    println!("warning: skipping undefined instruction");
}

#[allow(dead_code)]
extern "C" {
    fn _reset();
    // TODO implement these?
    fn handle_swi();
    fn handle_prefetch_abrt();
    fn handle_data_abrt();
    fn handle_irq();
    fn handle_fiq();
}
