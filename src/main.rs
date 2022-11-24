#![no_std]
#![no_main]
#![feature(core_intrinsics, lang_items)]

// The "bootloader" code is top level in setup. Including mod setup runs it.
mod setup;
mod uart;
mod utils;
use crate::uart::*;

#[no_mangle]
pub extern "C" fn kernel_main() {
    uart_init();
    write(
        "
-----------------------------------------
Boot complete.  Executing in kernel_main.
-----------------------------------------
\n\n
",
    );
    // this fails at 1095 characters? Interesting.
    loop {
        set_mode(UartMode::Rx);
        let c = getc();
        set_mode(UartMode::Tx);
        writec(&c);
    }
}
