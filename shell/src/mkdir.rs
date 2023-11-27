use std::fs;
use std::io;

pub fn mkdir(directory: &str, create_parents: bool) -> io::Result<()> {
    if create_parents {
        fs::create_dir_all(directory)?;
    } else {
        fs::create_dir(directory)?;
    }
    Ok(())
}

pub fn mkdir_with_flag(args: &[&str]) -> io::Result<()> {
    let mut create_parents = false;
    let mut directory = "";

    for (index, arg) in args.iter().enumerate() {
        if *arg == "-p" {
            create_parents = true;
        } else {
            directory = args[index];
        }
    }

    mkdir(directory, create_parents)
}
