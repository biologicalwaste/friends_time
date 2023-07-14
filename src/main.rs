use std::fs;
use std::fs::File;
use std::io;
use time::OffsetDateTime;

fn main() {
    match File::open(".friendstimedata") {
        Ok(_) => read_file(),
        Err(_) => {
            let _ = fs::write(".friendstimedata", "OFFSET:");
            get_offset();
        }
    };
}

fn get_offset() {
    
}

fn read_file() {
    let mut time_data = String::new();
    match fs::read_to_string(".friendstimedata") {
        Ok(data) => time_data = data,
        Err(_) => println!("File not readable! Is it there? Is it on fire?"),
    }
    println!("{time_data}");
}
