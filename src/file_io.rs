use std::io::stdin;
use std::collections::HashMap;

pub fn get_data(msg: &str) -> String {
    println!("{msg}");
    let mut data_in = String::new();
    
    stdin().read_line(&mut data_in).unwrap();

    return data_in;
}

pub fn first_run() -> HashMap<String, i8> {
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

pub fn add_offset(name: &str, offset: i32) {

}

pub fn get_offset(name: String) {
}