use std::fs::{File, self};
use std::io::stdin;
use std::collections::HashMap;
use std::process::exit;
use serde_json;

fn main() {
    let mut data = match File::open(".friendstime.json") {
        Ok(file) => serde_json::from_reader(file).unwrap(),
        Err(_) => first_run()
    };

    loop {
        for name in &data {
            println!("{} | {}", name.0, name.1);
        }

        let user_response: char = loop { match get_data("Would you like to [A]dd a new friend's time or [R]emove a friend's time? (Or [E]xit.").trim().parse() {
            Ok(response) => break response,
            Err(_) => {
                println!("Input needs to be a single character!");
                continue;
            }
        }};

        match user_response {
            'A' => {
                let new_name = get_data("What is their name?").trim().to_string();
                let new_offset: i8 = loop { match get_data("What is their UTC offset?").trim().parse() {
                    Ok(num) => break num,
                    Err(_) => {
                        println!("This is not a valid number!");
                        continue;
                    }
                }};

                data.insert(new_name.to_string(), new_offset);

                continue;
            }
            'R' => {
                let to_remove = get_data("Which friend's time do you want to remove?");
                data.remove(&to_remove);

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

fn get_data(msg: &str) -> String {
    println!("{msg}");
    let mut data_in = String::new();
    
    stdin().read_line(&mut data_in).unwrap();

    return data_in;
}

fn first_run() -> HashMap<String, i8> {
    let mut data = HashMap::new();

    let name = get_data("What is your name?").trim().to_string();
    let offset: i8 = loop {
            match get_data("What is your UTC offset?").trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("That isn't a valid number!");
                continue;
            }
        }
    };

    data.insert(name, offset);

    return data;
}