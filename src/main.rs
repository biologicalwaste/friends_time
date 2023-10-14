use std::fs;
use std::fs::File;
use std::io::stdin;
use time::macros::*;
use time::*;

fn main() {
    // Check for the data file, if found: read it!
    match File::open(".friendstimedata") {
        Ok(_) => read_file(),
        Err(_) => {
            // If not found, make it.
            let _ = fs::write(".friendstimedata", "");
            get_offset();
            read_file();
        }
    };
}

fn get_offset() {
    println!("Please input your UTC offset!");
    let mut offset_in = String::new();

    loop {
        match stdin().read_line(&mut offset_in) {
            Ok(_) => break,
            Err(_) => continue,
        }
    }
}
