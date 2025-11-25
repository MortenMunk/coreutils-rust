use std::{env::args, fs::create_dir, io::Result};

fn main() -> Result<()> {
    for arg in args().skip(1) {
        create_dir(arg)?;
    }
    Ok(())
}
