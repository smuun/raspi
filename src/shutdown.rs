use core::arch::asm;
#[allow(dead_code)]
/// Trigger a system shutdown.
pub fn kernel_halt() -> ! {
    //TODO add hardware support
    qemu_angel_exit(QemuExitCode::Ok);
    loop {}
}

pub enum QemuExitCode {
    Ok,
    Fail,
}

/// Use QEMU angel mode (-semihosting must be enabled)
/// to exit.  Use for testing.
pub fn qemu_angel_exit(code: QemuExitCode) {
    fool_rustc();
    match code {
        QemuExitCode::Ok => unsafe {
        },
        QemuExitCode::Fail => unsafe {
        },
    }
}

// for some reason this is necessary not to optimize away the entire
// setup.s????
pub fn fool_rustc() {
    unsafe{
        asm!("ldr r0, =_start");
    }
}