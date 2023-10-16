use std::fs::{File, self};
use std::process::exit;
use serde_json;

use file_io::*;

mod file_io;

fn main() {
    let mut data = match File::open(".friendstime.json") {
        Ok(file) => serde_json::from_reader(file).unwrap(),
        Err(_) => first_run()
    };

    loop {
        let user_response: char = loop { match get_data("Would you like to [A]dd a new friend's time or [R]emove a friend's time? (Or [E]xit.").trim().parse() {
            Ok(response) => break response,
            Err(_) => {
                println!("Input needs to be a single character!");
                continue;
            }
        }};
        match user_response {
            'A' => {
                let new_name = get_data("What is their name?");
                let new_offset: i8 = loop { match get_data("What is their UTC offset?").trim().parse() {
                    Ok(num) => break num,
                    Err(_) => {
                        println!("This is not a valid number!");
                        continue;
                    }
                }};

                data.insert(new_name, new_offset);

                continue;
            }
            'R' => {
                continue;
            }
            'E' => {
                let data_out = serde_json::to_string_pretty(&data).expect("This doesn't parse for some reason!");
                fs::write(".friendstime.json", data_out).unwrap();
                exit(1)
            }
            _ => {
                println!("Type A, R, or E.");
                continue;
            }
        }
    }
}
