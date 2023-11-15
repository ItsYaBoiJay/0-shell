use std::fs;
use std::io;

pub fn cp(source: &str, destination: &str) -> io::Result<()> {
    fs::copy(source, destination)?;
    Ok(())
}
