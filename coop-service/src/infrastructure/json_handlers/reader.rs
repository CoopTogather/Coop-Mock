use std::{fs::File, io::BufReader};

use serde::de::DeserializeOwned;

use crate::errors::CustomError;

pub fn read_json_file<T>(path: &str) -> Result<T, CustomError>
where
    T: DeserializeOwned,
{
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let result = serde_json::from_reader(reader)?;

    Ok(result)
}
