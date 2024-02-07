// cmd_executor.rs

use std::env;
use std::fs;
use std::os::unix::fs::MetadataExt;
use users::{get_user_by_uid, get_group_by_gid};
use chrono::prelude::*;
use chrono::TimeZone;

use std::path::Path;
use fs::read_dir;

#[doc = "The handle_ls() function handles the ls command.
    supporting the -a, -l, and -F flags.
    It takes a vector of arguments and returns an error
    if one occurred while listing the directory.
    Otherwise, it returns Ok(())"]
pub fn handle_ls(args: Vec<&str>) -> Result<(), String> {
    let mut list_hidden = false;
    let mut list_long = false; 
    let mut list_indicator = false;

    for arg in args.iter() {
        match *arg {
            "-a" => list_hidden = true,
            "-l" => list_long = true,
            "-F" => list_indicator = true,
            _ => return Err(format!("ls: invalid option -- '{}'\nusage: ls [-a] [-l] [-F]", arg)),
        }
    }
    // env::current_dir() returns a Result<PathBuf, Error> that contains the current directory.
    // If the current directory cannot be determined, an error is returned.
    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(err) => return Err(format!("ls: {}", err)),
    };
    print_entries(&current_dir, list_long, list_hidden, list_indicator)
}

#[doc = "print_entries() lists the entries in a directory.
    It takes a directory path, a boolean indicating whether to
    list details, a boolean indicating whether to list hidden
    files, and a boolean indicating whether to list indicators.
    it uses the read_dir() method to read the directory entries.
    then filters out hidden files if the -a flag is not set,
    uses print_entry_details() to print the details of each entry,
    and print_indicator() to print an indicator character for each entry.
    it returns an error if one occurred while reading the directory.
    Otherwise, it returns Ok(())"]
    fn print_entries(dir: &Path, details:bool, list_hidden: bool, list_indicator: bool) -> Result<(), String> {
        let mut entries: Vec<_> = match read_dir(dir) {
            Ok(entries) => {
                // Filter out hidden files if the -a flag is not set
                entries.filter_map(Result::ok)
                .filter(|entry: &fs::DirEntry| list_hidden || !is_hidden(entry)).collect()
                        
            }
            Err(err) => return Err(format!("ls: {}", err)),
        };
    
        entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
        // Print the total size of the entries if the -l flag is set
        if details {
            let total_size = calculate_total_size(&entries).unwrap();
            println!("total {}", total_size);
        }

        // Iterate over entries
        for entry in entries {
            // Print entry details
            if details {
                print_entry_details(&entry)?;
            }else{
                print!("{}", entry.file_name().to_string_lossy());
            }
            if list_indicator {
                print_indicator(&entry);
            }
            println!();

        }
    
        Ok(())
    }
    
#[doc = "Prints the details of a directory entry.
    It takes a directory entry and prints the metadata.
    including the mode, number of links, user ID, size,
    and name of the entry. It returns an error if one
    occurred while retrieving the metadata. Otherwise,
    it returns Ok(())"]
    fn print_entry_details(entry: &fs::DirEntry) -> Result<(), String> {
        let metadata = match entry.metadata() {
            Ok(meta) => meta,
            Err(err) => return Err(format!("ls: {}", err)),
        };

        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();
        let user = get_user_by_uid(metadata.uid()).unwrap();
        let group = get_group_by_gid(metadata.gid()).unwrap();
        let formatted_time = Local.timestamp(metadata.mtime(), 0).format("%b %e %H:%M").to_string();
        let permissions = convert_to_permission(&metadata);
        print!("{} {} {} {} {} {} {}",
            permissions,
            metadata.nlink(),
            user.name().to_string_lossy(),
            group.name().to_string_lossy(),
            metadata.size(),
            formatted_time,
            format!("{}", file_name_str),
        );

        Ok(())
    }

fn convert_to_permission(metadata: &fs::Metadata) -> String {
    // Define a mapping of bit positions to permission characters
    let permissions_mapping = [
        ('r', 0o400),
        ('w', 0o200),
        ('x', 0o100),
        ('r', 0o040),
        ('w', 0o020),
        ('x', 0o010),
        ('r', 0o004),
        ('w', 0o002),
        ('x', 0o001),
    ];

    // Include '@' symbol if extended attributes are present
    let extended_attributes = if metadata.file_type().is_symlink()  {
        "@"
    } else {
        " "
    };  
    // Iterate over the mapping, applying it to construct the permissions string
    let permissions: String = permissions_mapping
        .iter()
        .map(|&(char, mask)| if metadata.mode() & mask != 0 { char } else { '-' })
        .collect();

    format!("{}{}", permissions, extended_attributes)
}
    


#[doc ="Calculates the total disk space occupied by the listed files
    and directories.For each entry, it retrieves the metadata and
    accumulates the total size based on the number of blocks used
    by the entry. The result is the 'total' value displayed at
    the beginning of the `ls` output."]
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
#[doc = "Prints an indicator character for a directory entry.
    If the entry is a directory, it prints a forward slash (/).
    If the entry is executable, it prints an asterisk (*).
    Otherwise, it prints nothing."]
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

#[doc = "is_hidden() Determines if a directory entry is hidden.
    It takes a directory entry and returns true if the file name
    starts with a dot, indicating that it is hidden. Otherwise,
    it returns false. If the file name cannot be determined,
    it returns false as well."]
fn is_hidden(entry: &fs::DirEntry) -> bool {
    if let Ok(file_name) = entry.file_name().into_string() {
        file_name.starts_with('.') // Check if file name starts with a dot
    } else {
        false // Unable to determine file name, consider it not hidden
    }
}