use core::panic::PanicInfo;
pub use core::ptr::read_volatile;
pub use core::ptr::write_volatile;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
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
