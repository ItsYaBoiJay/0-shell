use std::{fs, time::SystemTime};
use std::path::Path;
use chrono::{DateTime, Local};

pub fn ls(args: &[&str]) {
    let list_all = args.iter().any(|&arg| arg == "-a");
    let long_format = args.iter().any(|&arg| arg == "-l");
    let classify = args.iter().any(|&arg| arg == "-F");

    let entries = fs::read_dir(".").expect("Failed to read directory");

    for entry in entries {
        if let Ok(entry) = entry {
            let file_name = entry.file_name().into_string().unwrap();

            if !list_all && file_name.starts_with('.') {
                continue;
            }

            if long_format {
                if let Ok(metadata) = entry.metadata() {
                    print_long_format(metadata, &file_name);
                }
            } else {
                if classify {
                    print!("{}{}", file_name, classify_file(&entry));
                } else {
                    print!("{}\t", file_name);
                }
            }
        }
    }
    println!();
}

fn print_long_format(metadata: fs::Metadata, file_name: &str) {
    let file_type = if metadata.is_dir() {
        "d"
    } else if metadata.is_file() {
        "-"
    } else if metadata.file_type().is_symlink() {
        "l"
    } else {
        "?"
    };

    let datetime: DateTime<Local> = metadata.modified().unwrap_or_else(|_| SystemTime::now()).into();
    let datetime_str = datetime.format("%Y-%m-%d %H:%M").to_string();

    println!(
        "{:<5} {:>19} {:>12} {}",
        file_type,
        datetime_str,
        metadata.len(),
        file_name
    );
}

fn classify_file(entry: &fs::DirEntry) -> String {
    if let Ok(file_type) = entry.file_type() {
        if file_type.is_dir() {
            return "/".to_owned();
        } else if file_type.is_symlink() {
            return "@".to_owned();
        }
    }
    String::new()
}
