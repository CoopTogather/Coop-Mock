use serde::de::DeserializeOwned;
use tokio::{fs::File, io::BufReader};

pub async fn read_json_file<T>(path: &str) -> Result<T, String>
where
    T: DeserializeOwned,
{
    let file = match File::open(path).await {
        Ok(file) => file,
        Err(err) => return Err(err.to_string()),
    };
    let reader = BufReader::new(file);
    todo!()
}
