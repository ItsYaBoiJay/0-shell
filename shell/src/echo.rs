use std::fs::File;
use std::io::Write;

pub fn echo(args: &[&str]) -> Result<(), std::io::Error> {
    if let Some(redirect_index) = args.iter().position(|&x| x == ">") {
        // If redirection symbol found
        if let Some(filename) = args.get(redirect_index + 1) {
            if let Ok(mut file) = File::create(filename) {
                let content = args[..redirect_index].join(" ");
                if let Err(e) = writeln!(file, "{}", content) {
                    return Err(e);
                }
                return Ok(());
            } else {
                return Err(
                    std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Error creating file '{}'", filename)
                    )
                );
            }
        } else {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "No filename provided for redirection"
                )
            );
        }
    }

    // If no redirection, echo to console
    for (i, arg) in args.iter().enumerate() {
        if i != 0 {
            print!(" ");
        }
        print!("{}", remove_quotes(arg));
    }
    println!();
    Ok(())
}

fn remove_quotes(s: &str) -> &str {
    s.trim_matches(['"', '\''].as_ref())
}
