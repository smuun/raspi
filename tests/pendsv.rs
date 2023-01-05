#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::arch::asm;
use core::ptr::write_volatile;
use raspi::println;
use raspi::shutdown::{qemu_angel_exit, QemuExitCode};

#[no_mangle]
pub extern "C" fn kernel_main() {
    test_main();
}


#[test_case]
fn catch_trap() {
    println!("Executing trap (should hang)...");
    unsafe {asm!("trap"); }
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
