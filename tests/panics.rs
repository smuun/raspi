#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::arch::asm;
use core::panic::PanicInfo;
use raspi::{println, Testable};
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
fn trap_instruction_causes_panic() {
    unsafe {
        asm!("trap");
    }
}

#[test_case]
fn invalid_write_causes_panic() {
    println!("invalid write:    in main sp = {:#x}", unsafe { raspi::read_sp() });
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