// use std::string;

/*use std::process::Command;

pub fn echo(input: &str) {
    let _output = Command::new("sh")
        .arg("-c")
        .arg(format!("{}", input))
        .output()
        .expect("Failed to execute command");

    // println!("{}", String::from_utf8_lossy(&output.stdout));
}
*/

use std::fs::File;
use std::io::Write;

pub fn echo(args: &[&str]) {
    if let Some(redirect_index) = args.iter().position(|&x| x == ">") {
        // If redirection symbol found
        if let Some(filename) = args.get(redirect_index + 1) {
            if let Ok(mut file) = File::create(filename) {
                let content = args[..redirect_index].join(" ");
                if let Err(e) = writeln!(file, "{}", content) {
                    eprintln!("Error writing to file: {}", e);
                }
                return;
            } else {
                eprintln!("Error creating file '{}'", filename);
                return;
            }
        } else {
            eprintln!("No filename provided for redirection");
            return;
        }
    }

    // If no redirection, echo to console
    let content = args.join(" ");
    println!("{}", content);
}

