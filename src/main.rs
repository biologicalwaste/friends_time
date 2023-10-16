use file_io::*;

mod file_io;

fn main() {
    let path = ".friendstime.json".to_string();
    first_run(&path);
    write_offset_to_file(&path, "Rosa", 16);
}
