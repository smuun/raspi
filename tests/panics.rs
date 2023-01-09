#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::{arch::asm, panic::PanicInfo};

use raspi::{
    println,
    shutdown::{qemu_angel_exit, QemuExitCode},
    Testable,
};
#[no_mangle]
pub extern "C" fn kernel_main() {
    test_main();
}
#[test_case]
fn trap_instruction_causes_panic() {
    unsafe {
        asm!("trap");
    }
}
#[test_case]
fn invalid_write_causes_panic() {
    println!("invalid write:    in main sp = {:#x}", raspi::read_sp());
    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    }
}
pub fn test_runner(tests: &[&dyn Testable]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
        println!("[test did not panic]");
        qemu_angel_exit(QemuExitCode::Ok);
    }
    qemu_angel_exit(QemuExitCode::Ok);
}
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("[ok]");
    qemu_angel_exit(QemuExitCode::Ok);
    loop {}
}
