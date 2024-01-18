use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockOptions {
    pub response: Option<MockResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockResponse {
    pub status_code: i16,
    pub body: Option<String>,
}

impl Default for MockOptions {
    fn default() -> Self {
        Self { response: None }
    }
}
