extern crate cc;
use std::error::Error;
use cc::Build;

fn main() -> Result<(), Box<dyn Error>> {
    Build::new()
        .file("src/boot.s")
        .flag("-mabi=lp64")
        .flag("-g")
        .flag("-O0")
        .compile("asm");
    
    Ok(())
}
