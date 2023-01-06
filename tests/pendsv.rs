#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::arch::asm;
use raspi::println;
use raspi::shutdown::{qemu_angel_exit, QemuExitCode};
use raspi::uart::uart_init;

#[no_mangle]
pub extern "C" fn kernel_main() {
    uart_init();
    test_main();
}

#[cfg(test)]
#[no_mangle]
pub unsafe extern "C" fn HardFault() {
    asm!("ldr r0, =0x456");
    qemu_angel_exit(QemuExitCode::Ok);
}

#[test_case]
fn catch_trap() {
    println!("Executing trap (should be caught and exit)...");
    unsafe {
        asm!("trap");
    }
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
        println!("[test did not panic]");
        qemu_angel_exit(QemuExitCode::Fail);
    }
    qemu_angel_exit(QemuExitCode::Ok);
}
