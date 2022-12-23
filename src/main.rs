#![no_std]
#![no_main]
#![feature(core_intrinsics, lang_items)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::utils::test_runner)]
#![reexport_test_harness_main = "test_main"]


// The "bootloader" code is top level in setup. Including mod setup runs it.
mod setup;
mod uart;

#[macro_use]
mod utils;
use crate::uart::*;

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
    loop {
        set_mode(UartMode::Rx);
        let c = getc();
        set_mode(UartMode::Tx);
        if c == '\r' as u8 {
            // Entered a CR, i.e. \r, ASCII 13
            // Convert it to \n
            println!("");
        }
        print!("{}", c as char);
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test_case]
    fn prints_10_lines() {
        for i in 0..10 {
            println!("Line {i}");
        }
        println!("[ok]");
    }
}
