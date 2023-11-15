use std::fs;
use std::io;

pub fn mkdir(directory: &str) -> io::Result<()> {
    fs::create_dir(directory)?;
    Ok(())
}
