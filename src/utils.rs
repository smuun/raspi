use core::panic::PanicInfo;
use crate::println;
use crate::print;
use core::arch::asm;
pub use core::ptr::read_volatile;
pub use core::ptr::write_volatile;

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    qemu_angel_exit(QemuExitCode::Ok)
}

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    if let Some(s) = _info.payload().downcast_ref::<&str>() {
        println!("panicked: {s:?}");
    } else {
        println!("panicked at unknown");
    }
    qemu_angel_exit(QemuExitCode::Fail);
    loop{}
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

#[allow(dead_code)]
/// Trigger a system shutdown.
pub fn kernel_halt() -> ! {
    //TODO add hardware support
    qemu_angel_exit(QemuExitCode::Ok);
    loop{}
}


pub enum QemuExitCode {
    Ok,
    Fail
}

/// Use QEMU angel mode (-semihosting must be enabled)
/// to exit.  Use for testing.
pub fn qemu_angel_exit(code: QemuExitCode) {
    match code {
        QemuExitCode::Ok => unsafe {
            asm!("b _qemu_halt_normal");
        }
        QemuExitCode::Fail => unsafe {
            asm!("b _qemu_halt_fail");
        }
    }
}


pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T where
T: Fn(), {
    fn run(&self) {
        print!("{}...\t", core::any::type_name::<T>());
        self();
        println!("[ok]");
    }
}
