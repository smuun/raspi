#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(error_in_core)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(core_intrinsics, lang_items)]

use core::{
    arch::asm,
    ptr::{read_volatile, write_volatile},
};

pub mod uart;

pub mod shutdown;
use shutdown::{qemu_angel_exit, QemuExitCode};
use uart::Uart;

#[macro_use]
mod exceptions;

// Peripherals
pub struct Peripherals {
    uart: Option<Uart>,
}

pub static mut PERIPHERALS: Peripherals = Peripherals {
    uart: Some(Uart {}),
};

// Utility functions
/// Write a config value to  a 32bit register
/// Set the config register at address 'register_base' to:
/// its current state, except that every bit in 'set' will
/// be set to its value in Value.
/// set & values & current config must be a valid uart config
/// panics if the write operation fails
pub fn configure(register_base: *mut u32, values: u32, set: u32) {
    unsafe {
        let mut word: u32 = read_volatile(register_base);
        // if a 'set' bit is high, change the current bit to
        // the corresponding Value
        word &= values & set;
        write_volatile(register_base, word);
        assert_eq!(read_volatile(register_base), word);
    }
}

// spin while the bit at mask is set
pub fn spin_while(ptr: *const u32, mask: u32) {
    unsafe { while ((read_volatile(ptr)) & mask) == mask {} }
}
// spin while the bit at mask is unset
pub fn spin_until(ptr: *const u32, mask: u32) {
    unsafe { while ((read_volatile(ptr)) & mask) != mask {} }
}

pub unsafe fn read_sp() -> u32 {
    let mut x: u32 = 0;
    asm!("mov {}, sp", inout(reg) x);
    x
}

// Testing, generally
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

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => (
        $crate::println!(
            "[LOG]  [{} at {}:{}]  {}",
            core::module_path!(),
            core::file!(),
            core::line!(),
            format_args!($($arg)*)
        )
    );
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => (
        $crate::println!(
            "[WRN]  [{} at {}:{}]  {}",
            core::module_path!(),
            core::file!(),
            core::line!(),
            format_args!($($arg)*)
        )
    );
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => (
        $crate::println!(
            "[ERR]  [{} at {}:{}]  {}",
            core::module_path!(),
            core::file!(),
            core::line!(),
            format_args!($($arg)*)
        )
    );
}
// Testing lib
#[no_mangle]
#[cfg(test)]
pub extern "C" fn kernel_main() {
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
