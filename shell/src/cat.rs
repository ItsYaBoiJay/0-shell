use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::path::Path;

pub fn cat(filename: &str) {
    let path = Path::new(filename);
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => {
                    eprintln!("File '{}' not found", filename);
                }
                std::io::ErrorKind::PermissionDenied => {
                    eprintln!("Permission denied for file '{}'", filename);
                }
                _ => {
                    eprintln!("Error opening file '{}': {}", filename, e);
                }
            }
            return;
        }
    };
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(e) => {
                eprintln!("Error reading file '{}': {}", filename, e);
                return;
            }
        }
    }
}
