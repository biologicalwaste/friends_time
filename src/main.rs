use std::fs::{File, self};
use std::collections::HashMap;
use serde_json;

use file_io::*;

mod file_io;

fn main() {
    let mut data = match File::open(".friendstime.json") {
        Ok(file) => serde_json::from_reader(file).unwrap(),
        Err(_) => first_run()
    };

    data.insert("Rosa".to_string(), 16);

    let data_out = serde_json::to_string_pretty(&data).expect("This doesn't parse for some reason!");
    fs::write(".friendstime.json", data_out).unwrap();f
}
