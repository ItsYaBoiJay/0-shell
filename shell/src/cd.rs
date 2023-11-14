use std::env;

pub fn cd(directory: &str) {
    if let Err(e) = env::set_current_dir(directory) {
        eprintln!("Failed to change directory: {}", e)
    }
}
