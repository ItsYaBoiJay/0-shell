use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn cp(source: &str, destination: &str) -> io::Result<()> {
    let source_path = Path::new(source);
    let destination_path = Path::new(destination);

    if source_path.is_dir() {
        // If the source is a directory, create the destination directory if not exists
        if !destination_path.exists() {
            fs::create_dir_all(destination_path)?;
        }

        // Iterate through the entries in the source directory and copy them to the destination
        for entry in fs::read_dir(source_path)? {
            let entry = entry?;
            let entry_path = entry.path();
            let dest_entry_path = destination_path.join(entry_path.file_name().unwrap());

            if entry_path.is_dir() {
                // Recursively copy subdirectories
                cp(&entry_path.to_str().unwrap(), &dest_entry_path.to_str().unwrap())?;
            } else {
                // Copy files directly
                fs::copy(&entry_path, &dest_entry_path)?;
            }
        }
    } else {
        // If the source is a file, copy it directly to the destination
        let dest_file_path = if destination_path.is_dir() {
            destination_path.join(source_path.file_name().unwrap())
        } else {
            PathBuf::from(destination_path)
        };

        fs::copy(source_path, dest_file_path)?;
    }

    Ok(())
}
