#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(raspi::test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::panic::PanicInfo;

use raspi::{
    error, log, print, println, readc,
    shutdown::{kernel_halt, qemu_angel_exit, QemuExitCode},
    warn,
};

#[no_mangle]
pub extern "C" fn kernel_main() {
    log!("log level");
    warn!("warn level");
    error!("error level");
    println!();
    println!();
    log!("executing in kernel_main");

    #[cfg(test)]
    test_main();

    log!("timer IRQ set? {}", raspi::timer::poll_timer_irq());
    log!("enabling timer IRQ...");
    raspi::timer::enable_timer_interrupts();
    log!("timer IRQ set? {}", raspi::timer::poll_timer_irq());
    loop{}
    fun_cli_app();
    log!("timer IRQ set? {}", raspi::timer::poll_timer_irq());
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
    // qemu_angel_exit(QemuExitCode::Fail);
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
