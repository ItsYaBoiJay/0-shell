use std::fs;
use std::io;

pub fn mv(source: &str, destination: &str) -> io::Result<()> {
    fs::rename(source, destination)?;
    Ok(())
}
