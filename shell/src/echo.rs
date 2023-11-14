// use std::string;

use std::process::Command;

pub fn echo(input: &str) {
    let _output = Command::new("sh")
        .arg("-c")
        .arg(format!("{}", input))
        .output()
        .expect("Failed to execute command");

    // println!("{}", String::from_utf8_lossy(&output.stdout));
}
