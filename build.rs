extern crate cc;
use cc::Build;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    Build::new()
        .file("src/boot.s")
        .file("src/riscv/helper/jump.s")
        .flag("-mabi=ilp32")
        .flag("-g")
        .flag("-O0")
        .compile("asm");

    Ok(())
}
