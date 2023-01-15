#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(raspi::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
use core::arch::asm;
use core::panic::PanicInfo;

use raspi::{
    log, print, println, readc, shutdown::kernel_halt, sys_timer::init_timer,
};

#[no_mangle]
pub extern "C" fn kernel_main() {
    log!("boot complete");
    println!();

    #[cfg(test)]
    test_main();

    init_timer();
    log!("timer enabled");
    fun_cli_app();
    shutdown_tasks();
    kernel_halt();
}

fn fun_cli_app() {
    // this fails at 1095 characters or something? interesting...
    println!("Scratchpad mode. Type q to shutdown.");
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
    loop {}
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test_case]
    fn stack_pointer_is_reset_on_undefined_exception_return() {
        let init_sp = raspi::read_sp();
        unsafe {
            asm!("trap");
        }
        let final_sp = raspi::read_sp();
        assert_eq!(init_sp, final_sp);
    }
    #[test_case]
    fn trigger_all_exceptions() {
        log!("in kernel sp = 0x{:x}", raspi::read_sp());
        log!("executing trap");
        unsafe {
            asm!("trap");
        }

        log!("in kernel sp = 0x{:x}", raspi::read_sp());
        log!("raising swi");
        unsafe {
            asm!("swi 1");
        }
    }
}
