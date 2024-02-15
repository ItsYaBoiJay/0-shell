use std::env;
use std::path::Path;

pub fn handle_cd(args: Vec<&str>) -> Result<(), String> {
    if let Some(dir) = args.get(0) {
        // check if the directory exists 
        if !Path::new(dir).exists() {
            return Err(format!("cd: {}: No such file or directory", dir));
        }

        set_current_dir(dir)
    } else {
        // if no directory is given, change to the home directory
        let home = env::var("HOME").unwrap();
        set_current_dir(&home)
    }

}

fn set_current_dir(dir: &str) -> Result<(), String> {
    if let Err(err) = env::set_current_dir(dir) {
        Err(format!("cd: {}", err))
    } else {
        Ok(())
    }
    
}