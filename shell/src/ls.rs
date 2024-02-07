use std::{fs, io, os::unix::fs::{MetadataExt, PermissionsExt}};
use chrono::{DateTime, Local};
use users::{get_user_by_uid, get_group_by_gid};

pub fn ls(args: &[&str]) -> Result<(), io::Error> {
    let list_all = args.iter().any(|&arg| arg == "-a");
    let long_format = args.iter().any(|&arg| arg == "-l");
    let classify = args.iter().any(|&arg| arg == "-F");

    let mut entries: Vec<String> = Vec::new();

    // If no arguments are provided, list all entries
    if args.is_empty() || list_all || long_format {
        entries.push(".".to_string());
        entries.push("..".to_string());
    }

    // Read directory and add entries
    entries.extend(
        fs::read_dir(".")?
            .filter_map(|entry| entry.ok().map(|e| e.file_name().to_string_lossy().to_string()))
    );

    // Sort entries
    entries.sort();

    // Calculate total block count
    let total_blocks: usize = if long_format {
        entries.iter().filter_map(|entry| {
            fs::metadata(entry)
                .ok()
                .and_then(|metadata| {
                    if metadata.is_file() {
                        Some(metadata.blocks() as usize / 2)
                    } else {
                        None
                    }
                })
        }).sum()
    } else {
        let total_blocks = if list_all {
            let dot_metadata = fs::metadata(".")?;
            dot_metadata.blocks() as usize / 2
        } else {
            0
        };

        let parent_metadata = fs::metadata("..")?;
        total_blocks + (parent_metadata.blocks() as usize / 2)
    };

    // Print total if in long format
    if long_format {
        println!("total {}", total_blocks);
    }

    // Iterate over entries and print details
    for entry in entries {
        let metadata = fs::metadata(&entry)?;
        let file_name_str = entry.to_string();

        if !list_all && file_name_str.starts_with('.') {
            continue;
        }

        if long_format {
            print_long_format(&metadata, &file_name_str, classify)?;
        } else {
            if classify {
                print!("{}{} ", file_name_str, classify_file(&metadata, &file_name_str));
            } else {
                print!("{}\t", file_name_str);
            }
        }
    }

    println!();
    Ok(())
}

fn print_long_format(metadata: &std::fs::Metadata, file_name: &str, classify: bool) -> Result<(), io::Error> {
    let permissions = format_permissions(metadata);
    let links = metadata.nlink();

    let user_name = get_user_by_uid(metadata.uid())
        .map(|user| user.name().to_string_lossy().into_owned())
        .unwrap_or_else(|| metadata.uid().to_string());

    let group_name = get_group_by_gid(metadata.gid())
        .map(|group| group.name().to_string_lossy().into_owned())
        .unwrap_or_else(|| metadata.gid().to_string());

    let size = metadata.len();
    let datetime: DateTime<Local> = metadata.modified().unwrap_or_else(|_| Local::now().into()).into();
    let datetime_str = datetime.format("%b %e %H:%M").to_string();

    let mut output_string = format!(
        "{} {:>2} {:<12} {:<12} {:>8} {} {}",
        permissions,
        links,
        user_name,
        group_name,
        size,
        datetime_str,
        file_name,
    );

    if classify {
        output_string.push_str(&classify_file(metadata, file_name));
    }

    println!("{}", output_string);
    Ok(())
}

fn classify_file(metadata: &std::fs::Metadata, file_name: &str) -> String {
    if metadata.is_dir() {
        "/".to_owned()
    } else if metadata.file_type().is_symlink() {
        format!("@ {}", file_name) // Include @ and the file name for symbolic links
    } else {
        String::new()
    }
}

fn format_permissions(metadata: &std::fs::Metadata) -> String {
    let mut permissions = String::with_capacity(11);
    let mode = metadata.permissions().mode();

    permissions.push(if metadata.is_dir() { 'd' } else if metadata.is_file() { '-' } else if metadata.file_type().is_symlink() { 'l' } else { '?' });
    permissions.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    permissions.push(if mode & 0o200 != 0 { 'w' } else { '-' });
    permissions.push(if mode & 0o100 != 0 { 'x' } else { '-' });
    permissions.push(if mode & 0o040 != 0 { 'r' } else { '-' });
    permissions.push(if mode & 0o020 != 0 { 'w' } else { '-' });
    permissions.push(if mode & 0o010 != 0 { 'x' } else { '-' });
    permissions.push(if mode & 0o004 != 0 { 'r' } else { '-' });
    permissions.push(if mode & 0o002 != 0 { 'w' } else { '-' });
    permissions.push(if mode & 0o001 != 0 { 'x' } else { '-' });

    if metadata.file_type().is_symlink() {
        permissions.push('@'); // Add "@" if it's a symlink
    }

    permissions
}
