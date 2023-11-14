// use std::io::{ self, Write };
// use std::process::Command;
// pub mod echo;
// pub mod cd;
// use echo::echo;
// use cd::cd;

// fn main() {
//     loop {
//         print!("$ ");
//         io::stdout().flush().unwrap();

//         let mut input = String::new();
//         io::stdin().read_line(&mut input).unwrap();

//         let mut parts = input.trim().split_whitespace();
//         let command = parts.next().unwrap();
//         let args = parts;

//         let output = Command::new(command).args(args).output();

//         echo(&input);

//         match output {
//             Ok(output) => {
//                 println!("{}", String::from_utf8_lossy(&output.stdout));
//             }
//             Err(e) => {
//                 println!("Error: {}", e);
//             }
//         }
//         // .expect("failed to execute command");

//         //println!("You entered: {}", input.trim());
//     }
// }

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
// use cp::cp;
// use rm::rm;
// use mv::mv;
// use mkdir::mkdir;
// use exit::exit;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let mut args = parts;
        echo(&input);
        match command {
            // "echo" => {
            //     echo(&input);
            // }
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
            _ => {
                let output = Command::new(command).args(args).output();

                match output {
                    Ok(output) => {
                        println!("{}", String::from_utf8_lossy(&output.stdout));
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
        }
    }
}
