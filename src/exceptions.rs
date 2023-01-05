use core::arch::asm;

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    asm!("b _start");

    loop {}
}

#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;