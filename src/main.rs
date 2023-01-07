#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(raspi::test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::panic::PanicInfo;

use raspi::{
    print, println, readc,
    shutdown::{kernel_halt, qemu_angel_exit, QemuExitCode},
};

#[no_mangle]
pub extern "C" fn kernel_main() {
    println!(
        "
-----------------------------------------
Boot complete.  Executing in kernel_main.
-----------------------------------------
"
    );
    #[cfg(test)]
    test_main();
    fun_cli_app();
    shutdown_tasks();
    kernel_halt();
}

fn fun_cli_app() {
    // this fails at 1095 characters? Interesting.
    println!("Enter a character to print 10x.");
    let c = readc!() as u8;
    for i in 0..10 {
        let num = 0x030 + i as u8;
        println!("{} number {}", c as char, num as char);
    }
    println!("Done.  Entering scratchpad mode. Type q to shutdown.");
    loop {
        let c = readc!() as char;
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

fn shutdown_tasks() {
    println!("Shutdown.");
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    qemu_angel_exit(QemuExitCode::Fail);
    loop {}
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
