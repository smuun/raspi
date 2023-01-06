#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(error_in_core)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(core_intrinsics, lang_items)]

use core::arch::asm;
use core::ptr::read_volatile;
use core::ptr::write_volatile;
pub mod uart;
#[cfg(test)]
use crate::uart::uart_init;
pub mod shutdown;
use shutdown::{qemu_angel_exit, QemuExitCode};

#[macro_use]

mod exceptions;
//Run the setup assembly unconditionally.

//Utility functions


// spin while the bit at mask is set
pub fn spin_while(ptr: *const u32, mask: u32) {
    unsafe { while ((read_volatile(ptr)) & mask) == mask {} }
}
// spin while the bit at mask is unset
pub fn spin_until(ptr: *const u32, mask: u32) {
    unsafe { while ((read_volatile(ptr)) & mask) != mask {} }
}

//Testing, generally

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        print!("{}...\t", core::any::type_name::<T>());
        self();
        println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    qemu_angel_exit(QemuExitCode::Ok)
}

pub unsafe fn read_sp() -> u32 {
    let mut x: u32 = 0;
    asm!("mov {}, sp", inout(reg) x);
    x
}

//Testing lib
#[no_mangle]
#[cfg(test)]
pub extern "C" fn kernel_main() {
    uart_init();
    println!(
        "
-------------------------------------------------
Boot complete. Executing in kernel_main (TESTING)
-------------------------------------------------
"
    );
    test_main();
}

#[cfg(test)]
mod tests {
    use crate::read_sp;

    #[test_case]
    fn sp_is_initialized_and_approx_0x8000() {
        let sp = unsafe { read_sp() };
        assert!(sp != 0);
        assert!(sp > 0x7000);
        assert!(sp <= 0x8000);
    }
}

#[cfg(test)]
use core::panic::PanicInfo;

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    qemu_angel_exit(QemuExitCode::Fail);
    loop {}
}