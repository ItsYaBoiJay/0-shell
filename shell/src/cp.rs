use std::fs;
use std::io;

pub fn cp(source: &str, destination: &str) -> io::Result<()> {
    let metadata = fs::metadata(source)?;
    if metadata.is_dir() {
        fs::create_dir_all(destination)?;
        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let path = entry.path();
            let destination_path = format!(
                "{}/{}",
                destination,
                path.file_name().unwrap().to_str().unwrap()
            );
            cp(&path.to_str().unwrap(), &destination_path)?;
        }
    } else {
        fs::copy(source, destination)?;
    }
    Ok(())
}
