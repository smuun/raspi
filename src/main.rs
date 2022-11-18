#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    setup();
    loop {}
}

fn setup() {
    unsafe {
        // 
        asm!("mov sp, #0x8000");
        
    }
}
