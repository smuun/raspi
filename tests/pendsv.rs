#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::ptr::write_volatile;
use raspi::println;
use raspi::shutdown::{qemu_angel_exit, QemuExitCode};

#[no_mangle]
pub extern "C" fn kernel_main() {
    test_main();
}

#[test_case]
fn pendsv_halts() {
    println!("Triggering PendSV...");
    trigger_pendsv();
}
pub extern "C" fn trigger_pendsv() {
    let icsr: *mut u32 = 0xe000ed04 as *mut u32;
    // Pend a PendSV exception using by writing 1 to PENDSVSET at bit 28
    unsafe {
        write_volatile(icsr, 1 << 28);
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
