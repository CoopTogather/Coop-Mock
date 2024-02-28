use poem::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockOptions {
    pub response: Option<MockResponseImpl>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockResponseImpl {
    pub status_code: i16,
    pub body: Option<String>,
}

pub trait MockResponse {
    fn status_code(&self) -> i16;
    fn body(&self) -> Option<String>;
    fn into_response(&self) -> impl IntoResponse;
}

impl Default for MockOptions {
    fn default() -> Self {
        Self { response: None }
    }
}
