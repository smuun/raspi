use core::arch::asm;

// IDIOT.
// you need to copy the IVT into memory as a first step.
// https://forums.raspberrypi.com/viewtopic.php?p=451228&hilit=ivt#p451228
#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    // we start executing here
    // copy the IVT into
    asm!("b _start");

    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn DefaultExceptionHandler() {
    // loop {}
}

#[link_section = ".ivt.reset"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

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


#[link_section = ".ivt.exceptions"]
#[no_mangle]
pub static EXCEPTIONS: [Vector; 14] = [
    Vector { handler: NMI },
    Vector { handler: HardFault },
    Vector { handler: MemManage },
    Vector { handler: BusFault },
    Vector { handler: UsageFault, },
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
