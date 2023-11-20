use std::io::{ self, Write };
use std::process::Command;
pub mod echo;
pub mod cd;
pub mod ls;
pub mod pwd;
pub mod cat;
pub mod cp;
pub mod rm;
pub mod mv;
pub mod mkdir;
pub mod exit;
use echo::echo;
use cd::cd;
use ls::ls;
use pwd::pwd;
use cat::cat;
use cp::cp;
use rm::rm;
use mv::mv;
use mkdir::mkdir;
use exit::exit;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let mut args = parts;
        // echo(&input);
        match command {
            "echo" => {
                echo(&input);
            }
            "cd" => {
                if let Some(dir) = args.next() {
                    cd(dir);
                } else {
                    eprintln!("No directory provided for cd command");
                }
            }
            "ls" => {
                ls(args.collect::<Vec<_>>().as_slice());
            }
            "pwd" => {
                pwd();
            }
            "cat" => {
                if let Some(filename) = args.next() {
                    cat(filename);
                } else {
                    eprintln!("No filename provided for cat command");
                }
            }
            "cp" => {
                if let (Some(source), Some(destination)) = (args.next(), args.next()) {
                    match cp(source, destination) {
                        Ok(_) => println!("File copied successfully"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                } else {
                    eprintln!("Usage: cp <source> <destination>");
                }
            }
            "rm" => {
                let mut files = Vec::new();
                let mut recursive = false;
                let mut force = false;

                for arg in args {
                    match arg {
                        "-r" | "-R" | "--recursive" => {
                            recursive = true;
                        }
                        "-f" | "--force" => {
                            force = true;
                        }
                        file => files.push(file),
                    }
                }

                match rm(&files, recursive, force) {
                    Ok(_) => println!("Files removed successfully"),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            "mkdir" => {
                if let Some(directory) = args.next() {
                    match mkdir(directory) {
                        Ok(_) => println!("Directory created successfully"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                } else {
                    eprintln!("No directory name provided for mkdir command");
                }
            }
            "mv" => {
                if let (Some(source), Some(destination)) = (args.next(), args.next()) {
                    match mv(source, destination) {
                        Ok(_) => println!("File or directory moved successfully"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                } else {
                    eprintln!("Usage: mv <source> <destination>");
                }
            }
            "exit" => {
                exit(0);
            }

            _ => {
                let output = Command::new(command).args(args).output();

                match output {
                    Ok(output) => {
                        println!("{}", String::from_utf8_lossy(&output.stdout));
                    }
                    Err(_e) => {
                        //println!("Error: {}", e);
                    }
                }
            }
        }
    }
}
