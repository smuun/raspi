#![no_std]
#![no_main]
#![feature(core_intrinsics, lang_items)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]


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
\n\n
",
    );
    // this fails at 1095 characters? Interesting.
    loop {
        set_mode(UartMode::Rx);
        let c = getc();
        set_mode(UartMode::Tx);
        print!("{}", c as char);
    }
}

    #[cfg(test)]
    fn test_runner(tests: &[&dyn Fn()]) {
        println!("Running {} tests", tests.len());
        for test in tests {
            test();
        }
    }