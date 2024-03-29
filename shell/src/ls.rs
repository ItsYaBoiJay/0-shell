// cmd_executor.rs

use std::env;
use std::fs;
use std::os::unix::fs::MetadataExt;
use users::{get_user_by_uid, get_group_by_gid};
use chrono::prelude::*;
use xattr::list;
use std::path::Path;
use exacl::{getfacl, setfacl, AclEntry, Perm};

    pub fn handlels(args: Vec<&str>) -> Result<(), String> {
        let mut args = args; // Declare args as mutable
        let directory = if args.len() > 0 && !args[0].starts_with('-') {
            Some(args.remove(0))
        } else {
            None
        };
    
        let mut list_hidden = false;
        let mut list_long = false; 
        let mut list_indicator = false;
        for arg in args.iter() {
            match *arg {
                "-a" => list_hidden = true,
                "-l" => list_long = true,
                "-F" => list_indicator = true,
                 _=> return Err(format!("ls: invalid option -- '{}'\nusage: ls [-a] [-l] [-F]", arg)),
            }
        }
        let dir = match directory {
            Some(dir) => Path::new(dir),
            None => Path::new("."),
        };
        // env::current_dir() returns a Result<PathBuf, Error> that contains the current directory.
        // If the current directory cannot be determined, an error is returned.
    /*     let current_dir = match env::current_dir() {
            Ok(dir) => dir,
            Err(err) => return Err(format!("ls: {}", err)),
        }; */
        print_entries(&dir, list_long, list_hidden, list_indicator)
    }

fn print_entries(dir: &Path, details:bool, list_hidden: bool, list_indicator: bool) -> Result<(), String> {
        let mut entries = match fs::read_dir(dir) {
            Ok(entries) => {
                let mut entries = entries.filter_map(Result::ok).collect::<Vec<_>>();
                if !list_hidden {
                    entries.retain(|entry| !is_hidden(entry)); // Filter out hidden
                } 
                entries
            }
            Err(err) => return Err(format!("ls: {}", err)),
        };

    entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    // Print the total size of the entries if the -l flag is set
    if details {
        let total_size = calculate_total_size(&entries).unwrap();
        println!("total {}", total_size);
    }

    if list_hidden {
        add_current_and_parent_name_to_entries(details, list_indicator);
    }

    // Iterate over entries
    for entry in entries {
        // Print entry details
        if details {
            let print_entry_details: Result<(), String> = print_entry_details(&entry.path());
            if print_entry_details.is_err() {
                return Err(print_entry_details.unwrap_err());
            }
        }
        // Print entry name
        print!(" {}", entry.file_name().to_string_lossy());

        if list_indicator {
            print_indicator(&entry);
        }
        println!();

    }

    Ok(())
}

fn print_entry_details(entry_path:&Path) -> Result<(), String> {
    let metadata = match entry_path.metadata() {
        Ok(meta) => meta,
        Err(err) => return Err(format!("ls: {}", err)),
    };
    let user = get_user_by_uid(metadata.uid()).unwrap();
    let group = get_group_by_gid(metadata.gid()).unwrap();
    let mut formatted_time = String::new();
    // check if the year of the file is the current year or not otherwise print the year instead of %H:%M
    if Local::now().year() != Local.timestamp(metadata.mtime(), 0).year() {
        formatted_time = Local.timestamp(metadata.mtime(), 0).format("%b %e  %Y").to_string();
    } else {
        formatted_time = Local.timestamp(metadata.mtime(), 0).format("%b %e %H:%M").to_string();
    }
    let permissions = convert_to_permission(&metadata, entry_path.to_str().unwrap());
    print!("{:12} {:>3} {:20} {:10} {:6} {:12}",
        permissions,
        metadata.nlink(),
        user.name().to_string_lossy(),
        group.name().to_string_lossy(),
        // print the size of the file in bytes if the file is not a directory
        metadata.size(),
        formatted_time,
    );

    Ok(())
}

fn has_extended_acl(path: &str) -> Result<bool, std::io::Error> {
    let acl: Vec<AclEntry> = getfacl(path, None)?;
    if acl.is_empty() {
        return Ok(false);
    }
    return Ok(true);
}

fn convert_to_permission(metadata: &fs::Metadata, path: &str) -> String {
    // Define a mapping of bit positions to permission characters
    // the codes are octal literals representing file mode permission bits in Unix-like systems
    /// Mapping of permission characters to their corresponding octal values.
    /// Each tuple represents a permission character and its corresponding octal value.
    /// The first element of the tuple represents the permission character ('r', 'w', or 'x'),
    /// and the second element represents the octal value.
    const PERMISSIONS_MAPPING: [(char, u32); 9] = [
        ('r', 0o0400),  // Read permission for owner
        ('w', 0o0200),  // Write permission for owner
        ('x', 0o0100),  // Execute permission for owner
        ('r', 0o0040),  // Read permission for group
        ('w', 0o0020),  // Write permission for group
        ('x', 0o0010),  // Execute permission for group
        ('r', 0o0004),  // Read permission for others
        ('w', 0o0002),  // Write permission for others
        ('x', 0o0001),  // Execute permission for others
    ];
    // Check if extended attributes are present for the given path
    let extended_attributes_present = match list(path) {
        Ok(attributes) => attributes.count() != 0, // Extended attributes found
        Err(_) => false, // Error occurred or no extended attributes found
    };
    let acl_indicator =  has_extended_acl(path).unwrap();

    // Determine the extended attributes indicator based on the result
    let extended_attributes_indicator: &str = if extended_attributes_present { "@" } else { " " };
    // Determine the directory indicator based on the mode bits
    let directory_indicator = if metadata.is_dir() { 'd' } else { '-' };

    // Iterate over the mapping, applying it to construct the permissions string
    let permissions: String = PERMISSIONS_MAPPING
        .iter()
        .map(|&(char, mask)| if metadata.mode() & mask != 0 { char } else { '-' })
        .collect();

    format!("{}{}{}", directory_indicator, permissions, extended_attributes_indicator)
}

fn calculate_total_size(entries: &[fs::DirEntry]) -> Result<u64, String> {
    let mut total_size: u64 = 0;
    for entry in entries {
        // The metadata() method returns a Result<Metadata, Error> that contains the metadata for the file.
        // If the metadata cannot be retrieved, an error is returned.
        let metadata = match entry.metadata() {
            Ok(meta) => meta,
            Err(err) => return Err(format!("ls: {}", err)),
        };
        // The blocks() method returns the number of 512-byte blocks allocated for the file.
        total_size += metadata.blocks();
    }
    Ok(total_size)
}

fn print_indicator(entry: &fs::DirEntry) {
    if let Ok(metadata) = entry.metadata() {
        if metadata.is_dir() {
            print!("/");
        } else if metadata.mode() & 0o111 != 0 {
            print!("*");
        } else if metadata.mode() & 0o1000 != 0 {
            print!("|"); // Symbol for FIFOs or pipes
        } else if metadata.mode() & 0o2000 != 0 {
            print!("="); // Symbol for sockets
        } else if metadata.mode() & 0o20000 != 0 {
            print!("%"); // Symbol for whiteouts
        } else {
            // Print a space for other file types
            print!(" ");
        }
    }
}

fn is_hidden(entry: &fs::DirEntry) -> bool {
    if let Ok(file_name) = entry.file_name().into_string() {
        file_name.starts_with('.') // Check if file name starts with a dot
    } else {
        false // Unable to determine file name, consider it not hidden
    }
}

/// Adds the entries for the current directory (`.`) and parent directory (`..`) to the list of entries.
fn add_current_and_parent_name_to_entries(details:bool, list_indicator: bool) { 

    if !details{
        // only check if the current directory and parent directory are exist and then print . and ..
        let current_dir = Path::new(".");
        let parent_dir = Path::new("..");
        if let Ok(_current_dir_entry) = fs::metadata(current_dir) { 
            if list_indicator {
                println!("./");
            }else{
                println!(".");
            
            }
        }
        if let Ok(_parent_dir_entry) = fs::metadata(parent_dir) {
            if list_indicator {
                println!("../");
            }else{
                println!("..");
            
            }
        }
        return; 
    }
    // if the -l flag is set, we need to print the current directory and parent directory details as well
    // The metadata() method returns a Result<Metadata, Error> that contains the metadata for the file.
    // If the metadata cannot be retrieved, an error is returned.
    if let Ok(current_dir_name) = env::current_dir() {
        let current_dir = Path::new(&current_dir_name);
        print_entry_details(&current_dir).unwrap();
        print!(" .");
        if list_indicator {
            print!("/");
        }
        println!();
        if let Ok(parent_dir_name) = env::current_dir() {
            let parent_dir: &Path = Path::new(&parent_dir_name).parent().unwrap();
            print_entry_details(&parent_dir).unwrap();
            print!(" ..");
            if list_indicator {
                print!("/");
            }
            println!();
        }
    }
}