use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MockOptions {
    pub response: MockResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MockResponse {
    pub status_code: i16,
    pub body: Option<String>,
}
