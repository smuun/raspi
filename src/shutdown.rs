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
    match code {
        QemuExitCode::Ok => unsafe {
            asm!("b _qemu_halt_normal");
        },
        QemuExitCode::Fail => unsafe {
            asm!("b _qemu_halt_fail");
        },
    }
}
