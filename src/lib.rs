#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(error_in_core)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(core_intrinsics, lang_items)]

use core::panic::PanicInfo;
use core::ptr::read_volatile;
use core::ptr::write_volatile;
pub mod uart;
#[allow(unused_imports)]
use crate::uart::uart_init;
pub mod shutdown;
use shutdown::{qemu_angel_exit, QemuExitCode};

#[macro_use]

mod exceptions;
//Run the setup assembly unconditionally.

//Utility functions

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    println!("panicked at {}", info);
    qemu_angel_exit(QemuExitCode::Fail);
    loop {}
}

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
mod tests { }
