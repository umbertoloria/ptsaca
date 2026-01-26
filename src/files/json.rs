use serde::Serialize;
use std::fs::File;
use std::io::Write;

pub fn dump_json_in_file<T: Serialize>(file_format: &T, file: &mut File) {
    let json = serde_json::to_string_pretty(file_format).unwrap();
    file.write(json.as_bytes())
        .expect("Unable to write JSON string");
    file.flush().expect("Unable to flush file");
}
