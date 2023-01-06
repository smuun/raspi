#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(raspi::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::arch::asm;
use core::ptr::write_volatile;
use raspi::shutdown::kernel_halt;
use raspi::uart::{getc, uart_init};
use raspi::{print, println};

#[no_mangle]
pub extern "C" fn kernel_main() {
    uart_init();
    println!(
        "
-----------------------------------------
Boot complete.  Executing in kernel_main.
-----------------------------------------
"
    );

    #[cfg(test)]
    test_main();

    trigger_exception();
    fun_cli_app();
    shutdown_tasks();
    kernel_halt();
}

fn shutdown_tasks() {
    println!("Shutdown.");
}

fn trigger_exception() {
    // let icsr = 0xe000ed04 as *mut u32;
    unsafe {
        asm!("trap");
    }
}

fn fun_cli_app() {
    // this fails at 1095 characters? Interesting.
    println!("Enter a character to print 10x.");
    let c = getc() as u8;
    for i in 0..10 {
        let num = 0x030 + i as u8;
        println!("{} number {}", c as char, num as char);
    }
    println!("Done.  Entering scratchpad mode. Type q to shutdown.");
    loop {
        let c = getc() as char;
        if c == 'q' {
            return;
        }
        if c == '\r' {
            println!("");
        } else {
            print!("{}", c as char);
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test_case]
    fn trivial() {
        assert!(1 == 1);
    }
}
