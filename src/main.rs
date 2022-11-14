#![no_std]
#![no_main]
use core::arch::asm;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        asm!("mov sp, #0x8000");
    }
    loop {}
}
