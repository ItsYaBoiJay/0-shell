use std::process::Command;

pub fn ls(args: &[&str]) {
    let output = Command::new("ls").args(args).output().expect("Failed to execute command");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
