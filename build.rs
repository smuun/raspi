use std::{env, error::Error, fs::File, io::Write, path::PathBuf};
extern crate cc;

fn main() -> Result<(), Box<dyn Error>> {
    // build directory for this crate
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // extend the library search path
    println!("cargo:rustc-link-search={}", out_dir.display());

    // put `linker.ld` in the build directory
    File::create(out_dir.join("linker.ld"))?.write_all(include_bytes!("src/linker.ld"))?;
    // put `setup.s` in the build directory
    File::create(out_dir.join("setup.S"))?.write_all(include_bytes!("src/setup.S"))?;

    cc::Build::new()
        .file("src/setup.S")
        .flag("-Wall")
        .flag("-nostdlib")
        .flag("-nostartfiles")
        .flag("-ffreestanding")
        .flag("-mcpu=arm1176jzf-s")
        .compile("setup.o");

    println!("cargo:rerun-if-changed=src/linker.ld");
    println!("cargo:rerun-if-changed=src/setup.S");

    Ok(())
}
