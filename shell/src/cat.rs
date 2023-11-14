use std::fs::File;
use std::io::{ BufRead, BufReader };

pub fn cat(filename: &str) {
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}
