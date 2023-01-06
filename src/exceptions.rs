// use core::{ptr::write_volatile, arch::asm};

// use crate::shutdown::kernel_halt;

// IVT needs to be at 0x0 but the raspberry pi starts execution at 0x8000
// unsafe fn move_ivt() {
    // let vec = 0x0 as *mut u32;
    // //4 byte increment
    // let _inc = 0x4 as *mut u32;
    // //the first entry should be the reset vector
    // write_volatile(vec, RESET_VECTOR as u32);
// }

// extern "C" {fn _bootldr();}

// IDIOT.
// you need to copy the IVT into memory as a first step.
// https://forums.raspberrypi.com/viewtopic.php?p=451228&hilit=ivt#p451228
// #[no_mangle]
// pub unsafe extern "C" fn Reset() -> ! {
    // This is where we start execution.
    // Copy the IVT into the appropriate memory address.

    // we want to copy the 16 word vector table

    // asm!("
        // /* move the reset vector to address 0 */
        // ldr r0, =_reset
        // mov r2, #0
        // ldr r1, [r0, #4]
        // str r1, [r2, #4]
// 
        // /* move the rest of the IVT */
        // ldr r0, =_ivt
        // b 2f
        // 1:
            // ldr r1, [r0, #4]
            // str r1, [r2, #4]
        // 2:
            // cmp r2, #0x14
            // blt 1b

    // "
    // );
    // _bootldr();
    // loop{};
// }

use core::arch::asm;

#[no_mangle]
pub unsafe extern "C" fn DefaultExceptionHandler() {
    asm!("ldr r0, =0x123");
    loop{}
}

#[allow(dead_code)]
extern "C" {
    fn NMI();
    fn MemManage();
    fn BusFault();
    fn UsageFault();
    fn SVCall();
    fn PendSV();
    fn SysTick();
    fn HardFault();
}
