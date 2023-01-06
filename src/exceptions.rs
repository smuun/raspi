use core::arch::asm;

use crate::{uart::uart_init, println};

#[no_mangle]
pub unsafe extern "C" fn DefaultExceptionHandler() {
    // asm!("mov r0, {}", in(reg) z);
    // asm!("mov pc, r14");
    // let x = 1; 
    // let y = 2;
    // let z = x & y;
    // asm!("reset");
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn PendSV() {
    asm!("mov r1, 0x42");
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn HardFault() {
    asm!("mov r1, 0x42");
    loop {}
}

#[allow(dead_code)]
extern "C" {
    fn NMI();
    fn MemManage();
    fn BusFault();
    fn UsageFault();
    fn SVCall();
    fn SysTick();
}
