use std::env;

pub fn pwd() {
    let current_dir = env::current_dir().unwrap();
    println!("{}", current_dir.display());
}
