use core::arch::asm;

use crate::log;

/// Trigger a system shutdown.
pub fn kernel_halt() -> ! {
    // TODO add hardware support
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
    // fool_rustc();
    const EXIT: u32 = 0x18;
    const OK: u32 = 0x20026;
    const FAIL: u32 = 0x20027;

    match code {
        QemuExitCode::Ok => unsafe {
            log!("qemu exit OK");
            // r[0-3] -> a[1-4]
            asm!("svc 0x00123456", in("r0") EXIT, in("r1") OK);
        },
        QemuExitCode::Fail => unsafe {
            log!("qemu exit fail");
            asm!("svc 0x00123456", in("r0") EXIT, in("r1") FAIL);
        },
    }
}
