#![no_std]
#![no_main]
#![feature(core_intrinsics, lang_items)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::utils::test_runner)]
#![reexport_test_harness_main = "test_main"]


// The "bootloader" code is top level in mod setup
mod setup;
mod uart;

#[macro_use]
mod utils;
use crate::{uart::*, utils::kernel_halt};

#[no_mangle]
pub extern "C" fn kernel_main() {
    uart_init();
    println!(
        "
-----------------------------------------
Boot complete.  Executing in kernel_main.
-----------------------------------------
");
    #[cfg(test)]
    test_main();

    // this fails at 1095 characters? Interesting.
    set_mode(UartMode::Tx);
    for i in 0..10  {
        // set_mode(UartMode::Rx);
        // let c = getc();
        let c = 65;
        let num = 0x060 + i as u8;
        if c == '\r' as u8 {
            // Entered a CR, i.e. \r, ASCII 13
            // Convert it to \n
            println!("");
        }
        println!("{} number {}", c as char, num as char);
    }
    kernel_halt();
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test_case]
    fn is_ok() {
        assert!(1 == 1);
        println!("[ok]");
    }
}
