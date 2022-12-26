#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
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

//Run the setup assembly unconditionally.
mod setup;

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

/// Write bit at base (word = 32 bit aligned) + offset.
/// Safe when:
///     - base is a valid, word aligned pointer
///     - we're allowed to write to base + offset
pub fn write_bit(base: *mut u32, offset: u8, bit: bool) {
    unsafe {
        let mut scratch: u32 = read_volatile(base);
        if bit {
            scratch |= 1 << offset;
        } else {
            scratch &= !(1 << offset);
        }
        write_volatile(base, scratch);
    }
}

pub fn read_bit(base: *mut u32, offset: u8) -> bool {
    unsafe{
        let scratch: u32 = read_volatile(base);
        return (scratch & 1 << offset) == 0x0;
    }
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
mod tests {
    use super::*;

    #[test_case]
    fn volatile_write_should_equal_read() {
        write_bit(0xc as *mut u32, 4, true);
        assert_eq!(true, read_bit(0x12345c as *mut u32, 3));
    }

}
