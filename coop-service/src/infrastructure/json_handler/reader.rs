pub struct JsonReaderImpl {
    file_path: String,
}

pub trait JsonReader<T> {
    fn new(file_path: String) -> Self;
    fn read(&self) -> T;
}

impl JsonReader for JsonReaderImpl {
    fn new(file_path: Option<String>) -> Self {
        let path = match file_path {
            Some(path) => path,
            None => String::from(""),
        };

        Self { file_path: path }
    }

    fn read(&self) -> String {
        let file = File::open(&self.file_path).unwrap();
        let reader = BufReader::new(file);

        let mut json = String::new();

        for line in reader.lines() {
            json.push_str(&line.unwrap());
        }

        json
    }
}
