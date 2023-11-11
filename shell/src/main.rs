// use std::io::{self, Write};
// use std::process::Command;
// pub mod echo;
// use echo::echo;

// fn main() {
//     loop {
//         print!("$ ");
//         io::stdout().flush().unwrap();

//         let mut input = String::new();
//         io::stdin().read_line(&mut input).unwrap();

//         let mut parts = input.trim().split_whitespace();
//         let command = parts.next().unwrap();
//         let args = parts;

//         let output = Command::new(command)
//         .args(args)
//         .output();

//         match output {
//             Ok(output) => {
//                 println!("{}", String::from_utf8_lossy(&output.stdout));
//             },
//             Err(e) => {
//                 println!("Error: {}", e);
//             }
//         }
//        // .expect("failed to execute command");
        
//         echo(&input);
//         //println!("You entered: {}", input.trim());
//     }
// }


use std::process::Command;
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "echo" => {
                let output = args.collect::<Vec<_>>().join(" ");
                println!("{}", output);
            },
            "cd" => {
                let path = args.peekable().peek().map_or("/", |x| *x);
                let root = std::path::Path::new(path);
                if let Err(e) = std::env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            },
            "ls" => {
                Command::new("ls")
                        .args(args)
                        .status()
                        .expect("Failed to execute command");
            },
            "pwd" => {
                let path = std::env::current_dir().unwrap();
                println!("{}", path.display());
            },
            "cat" => {
                let filename = args.peekable().peek().map_or("", |x| *x);
                let output = Command::new("cat")
                                 .arg(filename)
                                 .output()
                                 .expect("Failed to execute command");
                println!("{}", String::from_utf8_lossy(&output.stdout));
            },
            "cp" => {
                let mut args = args;
                let src = args.next().unwrap();
                let dest = args.next().unwrap();
                Command::new("cp")
                        .arg(src)
                        .arg(dest)
                        .status()
                        .expect("Failed to execute command");
            },
            "rm" => {
                Command::new("rm")
                        .args(args)
                        .status()
                        .expect("Failed to execute command");
            },
            "mv" => {
                let mut args = args;
                let src = args.next().unwrap();
                let dest = args.next().unwrap();
                Command::new("mv")
                        .arg(src)
                        .arg(dest)
                        .status()
                        .expect("Failed to execute command");
            },
            "mkdir" => {
                Command::new("mkdir")
                        .args(args)
                        .status()
                        .expect("Failed to execute command");
            },
            "exit" => return,
            command => {
                eprintln!("{}: command not found", command);
            }
        }
    }
}
