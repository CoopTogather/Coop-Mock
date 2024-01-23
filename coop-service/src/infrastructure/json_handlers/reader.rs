use std::{
    fs::File,
    io::{BufReader, Error},
};

use serde::de::DeserializeOwned;

pub fn read_json_file<T>(path: &str) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let result = serde_json::from_reader(reader)?;

    Ok(result)
}
