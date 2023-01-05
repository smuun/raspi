use core::arch::asm;
use core::ptr::write_volatile;

// IVT needs to be at 0x0 but the raspberry pi starts execution at 0x8000
unsafe fn move_ivt() {
    let vec = 0x0 as *mut u32;
    //4 byte increment
    let inc = 0x4 as *mut u32;
    //the first entry should be the reset vector
    write_volatile(vec, RESET_VECTOR as u32);
}

// IDIOT.
// you need to copy the IVT into memory as a first step.
// https://forums.raspberrypi.com/viewtopic.php?p=451228&hilit=ivt#p451228
#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    // This is where we start execution.
    // Copy the IVT into the appropriate memory address.

    // we want to copy the 16 word vector table
    // that's 16
    move_ivt();
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn DefaultExceptionHandler() {
    loop {}
}

#[link_section = ".ivt.reset"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

#[link_section = ".ivt.exceptions"]
#[no_mangle]
pub static IVT: [Vector; 14] = [
    Vector { handler: NMI },
    Vector { handler: HardFault },
    Vector { handler: MemManage },
    Vector { handler: BusFault },
    Vector {
        handler: UsageFault,
    },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: SVCall },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: PendSV },
    Vector { handler: SysTick },
];

pub union Vector {
    reserved: u32,
    handler: unsafe extern "C" fn(),
}

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
