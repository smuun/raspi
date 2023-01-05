use core::arch::asm;

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    //FIXME actually implement the entry point in Rust / asm?
    asm!("b _start");

    loop {}
}

#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;
