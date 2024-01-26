use std::fs;
use std::io;

pub fn rm(files: &[&str], recursive: bool, force: bool) -> io::Result<()> {
    for file in files {
        let path = std::path::Path::new(file);

        if recursive && path.is_dir() {
            if force {
                fs::remove_dir_all(file)?;
            } else {
                fs::remove_dir(file)?;
            }
        } else if path.is_file() {
            if force {
                fs::remove_file(file)?;
            } else {
                fs::remove_file(file)?;
            }
        } else {
            // Handle the case where it's neither a file nor a directory
            eprintln!("Error: {} is neither a file nor a directory", file);
        }
    }
    Ok(()) // Return Ok(()) to indicate success
}
