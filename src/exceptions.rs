use core::arch::asm;
use crate::{uart::uart_init, println};

#[no_mangle]
pub unsafe extern "C" fn handle_default() {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn handle_undefined_instruction() {
    asm!("mov r1, 0x43");
    let x = 1;
    let y = x | 1;
    asm!("mov {r}, {y}", r = out(reg) _, y = in(reg) y);
    // uart_init();
    loop {
        asm!("mov r1, 0x43");
    }
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
