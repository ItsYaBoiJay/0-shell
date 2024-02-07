use std::env;
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

fn main() {
    loop {
       let current_dir = env::current_dir().unwrap();
        let current_dir_str = current_dir
            .to_str()
            .unwrap()
            .replace(env::var("HOME").unwrap().as_str(), "~");

        print!("{} $ ", current_dir_str);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!("Ctrl+D (EOF) detected");
                break; // Exit the program when Ctrl+D is detected
            }
            Ok(_) => {
                if input.trim().is_empty() {
                    continue; // Skip the rest of the loop if the input is empty
                }

                let mut parts = input.trim().split_whitespace();
                let command = parts.next().unwrap();
                let args: Vec<&str> = parts.collect();

                match command {
                    "exit" => {
                        exit::exit(0);
                    }
                    "cd" => {
                        if let Some(dir) = args.get(0) {
                            cd::cd(dir);
                        } else {
                            eprintln!("No directory provided for cd command");
                        }
                    }
                    "ls" => {
                        let _ = ls::ls(&args);
                    }
                    "pwd" => {
                        pwd::pwd();
                    }
                    "cat" => {
                        if let Some(filename) = args.get(0) {
                            cat::cat(filename);
                        } else {
                            eprintln!("No filename provided for cat command");
                        }
                    }
                    "cp" => {
                        if args.len() >= 2 {
                            let _ = cp::cp(args[0], args[1]);
                        } else {
                            eprintln!("Usage: cp <source> <destination>");
                        }
                    }
                    "rm" => {
                        let files: Vec<&str> = args
                            .iter()
                            .filter(|&&x| !x.starts_with('-'))
                            .copied()
                            .collect();
                        let recursive = args
                            .iter()
                            .any(|&x| (x == "-r" || x == "-R" || x == "--recursive"));
                        let force = args.iter().any(|&x| (x == "-f" || x == "--force"));
                    
                        match rm::rm(&files, recursive, force) {
                            Ok(_) => println!("Removal successful"),
                            Err(e) => eprintln!("Error: {}", e),
                        }
                    }
                    
                    "mkdir" => {
                        if let Some(dir) = args.get(0) {
                            let _ = mkdir::mkdir(dir, false);
                        } else {
                            eprintln!("No directory name provided for mkdir command");
                        }
                    }
                    "mv" => {
                        if args.len() >= 2 {
                            let mut command = Command::new("mv");
                            command.args(&args);

                            match command.status() {
                                Ok(status) => {
                                    if !status.success() {
                                        eprintln!("Error: mv command failed with exit status {}", status);
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Error: {}", e);
                                }
                            }
                        } else {
                            eprintln!("Usage: mv <source> <destination>");
                        }
                    }

                    "echo" => {
                        let result = echo::echo(&args);
                        if let Err(e) = result {
                            eprintln!("Error: {}", e);
                        }
                    }
                    _ => {
                        let output = Command::new(command).args(&args).output();

                        match output {
                            Ok(output) => {
                                println!("{}", String::from_utf8_lossy(&output.stdout));
                            }
                            Err(e) => {
                                // Handle errors if needed
                                eprintln!("Error: {}", e);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
