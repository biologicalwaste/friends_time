use core::num;
use std::fs;
use std::fs::File;
use std::io::{self, stdin};
use time::OffsetDateTime;

fn main() {
    match File::open(".friendstimedata") {
        Ok(_) => read_file(),
        Err(_) => {
            let _ = fs::write(".friendstimedata", "OFFSET:");
            get_offset();
            read_file();
        }
    };
}

fn get_offset() {
    let mut user_offset = String::new();
    println!("Please input your UTC offset!");
    let mut offset_in = String::new();

    loop {
        match stdin().read_line(&mut offset_in) {
            Ok(_) => break,
            Err(_) => continue,
        }
    }
}

fn read_file() {
    let mut time_data = String::new();
    match fs::read_to_string(".friendstimedata") {
        Ok(data) => time_data = data,
        Err(_) => println!("File not readable! Is it there? Is it on fire?"),
    }
    println!("{time_data}");
}
