#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(raspi::test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::{
    panic::PanicInfo,
    arch::asm,
};

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
    // qemu_angel_exit(QemuExitCode::Fail);
    loop {}
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test_case]
    fn stack_pointer_is_reset_on_undefined_exception_return() {
        let init_sp = raspi::read_sp();
        unsafe {asm!("trap");}
        let final_sp = raspi::read_sp();
        assert_eq!(init_sp, final_sp);

    }
    #[test_case]
    fn trigger_all_exceptions() {
        log!("in kernel sp = 0x{:x}", unsafe {raspi::read_sp() });
        log!("trap");
        unsafe { asm!("trap"); }

        log!("in kernel sp = 0x{:x}", unsafe {raspi::read_sp() });
        log!("swi");
        unsafe { asm!("swi 1"); }

        log!("in kernel sp = 0x{:x}", unsafe {raspi::read_sp() });
        log!("invalid read");
        unsafe {
            asm!("
            mov r1, #0x7fffffff
            ldr r0, [r1]
          ");
        }
        log!("back in kernel sp = 0x{:x}", unsafe {raspi::read_sp() });
    }
}
