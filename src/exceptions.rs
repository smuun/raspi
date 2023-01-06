use crate::{uart::uart_init, println};
use core::{arch::asm, panic};

#[no_mangle]
pub unsafe extern "C" fn handle_default() {
    uart_init();
    panic!("unhandled exception: unknown");
}

#[no_mangle]
pub unsafe extern "C" fn handle_undefined_instruction() {
    uart_init();
    println!("warning: undefined instruction");
}

#[allow(dead_code)]
extern "C" {
    fn _reset();

    fn handle_swi();
    fn handle_prefetch_abrt();
    fn handle_data_abrt();
    fn handle_irq();
    fn handle_fiq();
}
