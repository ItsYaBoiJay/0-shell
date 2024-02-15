use std::fs::{remove_dir_all, remove_file};

#[doc = "The handle_rm() function handles the rm command.
    It takes a vector of arguments representing the files or directories to remove.
    It returns Ok(()) if successful, otherwise an error message."]
pub fn handle_rm(args: Vec<&str>) -> Result<(), String> {
    // Check if no file or directory name is provided
    if args.is_empty() {
        return Err("rm: missing operand".to_string());
    }

    // Check if the -r flag is present
    let recursive = args.contains(&"-r");

    // Iterate over the provided file or directory names and remove each
    for item in args {
        // Skip the -r flag
        if item == "-r" {
            continue;
        }

        if let Err(err) = remove_item(item, recursive) {
            return Err(err);
        }
    }

    Ok(())
}

#[doc = "Removes a file or directory based on the provided name and recursive flag.
    Returns Ok(()) if successful, otherwise an error message."]
fn remove_item(item: &str, recursive: bool) -> Result<(), String> {
    let path = std::path::Path::new(item);

    if path.is_dir() && recursive {
        // Remove directory recursively
        match remove_dir_all(path) {
            Ok(()) => {
                println!("Directory '{}' and its contents removed.", item);
                Ok(())
            }
            Err(err) => Err(format!("rm: {}", err)),
        }
    } else {
        // Remove file or empty directory
        match remove_file(path) {
            Ok(()) => {
                println!("File or directory '{}' removed.", item);
                Ok(())
            }
            Err(err) => Err(format!("rm: {}", err)),
        }
    }
}
