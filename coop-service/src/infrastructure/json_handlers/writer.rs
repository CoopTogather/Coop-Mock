use std::{fs::File, io::BufWriter};

use serde::Serialize;

pub fn write_json_file<T: Serialize>(path: &str, data: &T) -> Result<(), String> {
    let file = File::open(path);

    if file.is_err() {
        let new_file = match File::create(path) {
            Ok(file) => file,
            Err(err) => return Err(err.to_string()),
        };

        let writer = BufWriter::new(new_file);

        match serde_json::to_writer(writer, data) {
            Ok(result) => return Ok(result),
            Err(err) => return Err(err.to_string()),
        }
    }

    let writer = BufWriter::new(file.unwrap());

    match serde_json::to_writer(writer, data) {
        Ok(result) => Ok(result),
        Err(err) => return Err(err.to_string()),
    }
}
