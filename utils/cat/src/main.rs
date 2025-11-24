use std::{
    env::args,
    fs::File,
    io::{BufReader, Read, Result, Write, stdout},
};

fn main() -> Result<()> {
    let args: Vec<String> = args().skip(1).collect();
    if !args.is_empty() {
        for x in args {
            let mut reader = BufReader::new(File::open(&x)?);
            let stdout = stdout();
            let mut handle = stdout.lock();
            let mut buffer = [0_u8; 8192];
            loop {
                let n = reader.read(&mut buffer)?;
                if n == 0 {
                    break;
                }
                handle.write_all(&buffer[..n])?;
            }
        }
    }
    Ok(())
}
