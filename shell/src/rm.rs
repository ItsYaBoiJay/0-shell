use std::fs;
use std::io;

pub fn rm(files: &[&str], recursive: bool, force: bool) -> io::Result<()> {
    for file in files {
        if recursive {
            if force {
                fs::remove_dir_all(file)?;
            } else {
                fs::remove_dir_all(file)?;
            }
        } else {
            if force {
                fs::remove_file(file)?;
            } else {
                fs::remove_file(file)?;
            }
        }
    }
    Ok(()) // Return Ok(()) to indicate success
}
