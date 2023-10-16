use std::fs::{self, File};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::stdin;

pub fn get_data(msg: &str) -> String {
    println!("{msg}");
    let mut offset_in = String::new();
    
    stdin().read_line(&mut offset_in).unwrap();

    return offset_in;
}

pub fn first_run(path: &String) {
    let name = get_data("What is your name?").trim().to_string();
    let offset: i32 = get_data("What is your UTC offset?").trim().parse().expect("huh");

    let first_data = json!({
        "name": [
            &name
        ],
        "offset": [
            &offset
        ]
    }).to_string();

    fs::write(&path, first_data).expect("Something bad!");
}

pub fn write_offset_to_file(path: &str, name: &str, offset: i32) {

    let file = File::open(&path).unwrap();
    let a: Value = serde_json::from_reader(&file).unwrap();
    let old_file = a.to_string();

    let addition = json!({
        "name": &name,
        "offset": &offset
    }).to_string();

    let new_file = old_file + &addition;

    fs::write(&path, new_file).expect("Something bad!");
}

pub fn get_offset_from_file(path: String, name: String) {
}