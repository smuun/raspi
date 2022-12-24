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
    fun_cli_app();
    shutdown_tasks();
    //bootloader halts upon return from kernel_main
}

fn shutdown_tasks() {
    println!("Shutdown.");
}

fn fun_cli_app() {
    // this fails at 1095 characters? Interesting.
    println!("Enter a character to print 10x.");
    let c = getc() as u8;
    for i in 0..10  {
        let num = 0x030 + i as u8;
        println!("{} number {}", c as char, num as char);
    }
    println!("Done.  Entering scratchpad mode. Type q to shutdown.");
    loop {
        let c = getc() as char;
        if c == 'q' {
            return
        }
        if c == '\r' {
            // Entered a CR, i.e. \r, ASCII 13
            // Convert it to \n
            println!("");
        } else {
            print!("{}", c as char);
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test_case]
    fn trivial() {
        assert!(1 == 1);
    }
}
