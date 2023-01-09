#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(raspi::test_runner)]
#![reexport_test_harness_main = "test_main"]
use core::panic::PanicInfo;

use raspi::{
    println,
    shutdown::{qemu_angel_exit, QemuExitCode},
};
#[no_mangle]
pub extern "C" fn kernel_main() {
    test_main();
}
#[test_case]
fn test_println() {
    println!("Boot test initialized");
}
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("[ok]");
    println!();
    qemu_angel_exit(QemuExitCode::Ok);
    loop {}
}
