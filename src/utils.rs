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
