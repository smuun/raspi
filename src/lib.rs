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

pub mod timer;
pub mod uart;

pub mod shutdown;
use shutdown::{qemu_angel_exit, QemuExitCode};
use uart::Uart;

#[macro_use]
mod exceptions;

// Config
pub enum LogLevel {
    All,
    Warn,
    Error,
}
pub const LOGLEVEL: LogLevel = LogLevel::All;

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
    // log!("configuring register at {}", register_base as u32);
    unsafe {
        let mut word: u32 = read_volatile(register_base);
        // if a 'set' bit is high, change the current bit to
        // the corresponding Value

        // first, zero out all the bits in 'set'
        // i.e., invert set: then we don't care about all the high bits
        // and we want the low bits to be zero
        // AND this with the current value: then all the low bits are zero
        // and the high bits are equal to current value
        word = word & !set;

        // now: we have the current value, except that all the bits we
        // care about are zeroed.  OR it with high values that we care about:
        word = word | (values & set);
        write_volatile(register_base, word);
        // assert_eq!(read_volatile(register_base) & set, values & set);
    }
}

/// Read the register at address `register_base` and compare.  Return true if
/// every bit in `set` matches the value provided in `value`.  `register_base`
/// must be valid for volatile read.
///
/// # Example
/// I want the lowest bit set
/// `0b1011 & 0b0001 = 0b0001 ==  0b0001 & 0b0001`
/// or unset:
/// `0b1011 & 0b0000 = 0b0000 ==  0b0001 & 0b0000`
/// but not
/// `0b1011 & 0b0001 = 0b0001 !=  0b0001 & 0b0000`
pub fn poll(register_base: *mut u32, values: u32, set: u32) -> bool {
    unsafe {
        let word: u32 = read_volatile(register_base);
        word & set == values & set
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

pub fn read_sp() -> u32 {
    let mut x: u32 = 0;
    unsafe { asm!("mov {}, sp", inout(reg) x) };
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
        match $crate::LOGLEVEL {
            $crate::LogLevel::All => {
                $crate::println!(
                    "[LOG]  [{} at {}:{}]  {}",
                    core::module_path!(),
                    core::file!(),
                    core::line!(),
                    format_args!($($arg)*)
                )
            },
            _ => {}
        }
    );
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => (
        match $crate::LOGLEVEL {
            $crate::LogLevel::All | $crate::LogLevel::Warn => {
                $crate::println!(
                    "[WRN]  [{} at {}:{}]  {}",
                    core::module_path!(),
                    core::file!(),
                    core::line!(),
                    format_args!($($arg)*)
                )
            },
            _ => {}
        }
    );
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => (
        match $crate::LOGLEVEL {
            _ => {
                $crate::println!(
                    "[ERR]  [{} at {}:{}]  {}",
                    core::module_path!(),
                    core::file!(),
                    core::line!(),
                    format_args!($($arg)*)
                )
            }
        }
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
    fn sp_is_initialized_and_nonzero() {
        let sp = read_sp();
        assert!(sp != 0);
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
