use std::fs;
use std::path::Path;

pub fn handle_mv(args: Vec<&str>) -> Result<(), String> {
    // Check if the number of arguments is valid
    if args.len() != 2 {
        return Err("Usage: mv <source> <destination>".to_string());
    }

    let source = args[0];
    let destination = args[1];

    // Call the move_file function
    move_file(source, destination)
}

fn move_file(source: &str, destination: &str) -> Result<(), String> {
    let source_path = Path::new(source);
    let destination_path = Path::new(destination);

    // Check if the source path exists
    if !source_path.exists() {
        return Err(format!("mv: {}: No such file or directory", source));
    }

    // If the destination is an existing directory, append the filename to it
    let destination_path = if destination_path.is_dir() {
        destination_path.join(source_path.file_name().unwrap())
    } else {
        destination_path.to_path_buf()
    };

    // Perform the move operation
    if let Err(err) = fs::rename(source_path, &destination_path) {
        return Err(format!("mv: {}: {}", source, err));
    }

    Ok(())
}