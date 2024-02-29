use poem::{http::StatusCode, IntoResponse, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockOptions {
    pub response: Option<MockResponseImpl>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockResponseImpl {
    status_code: u16,
    body: Option<String>,
}

/// Represents a trait for generating mock responses.
pub trait MockResponse {
    /// Returns the status code of the response.
    fn status_code(&self) -> u16;

    /// Returns the body of the response, if any.
    fn body(&self) -> Option<String>;

    /// Converts the response into a type that can be used as an HTTP response.
    fn into_response(&self) -> Response;
}

impl MockResponse for MockResponseImpl {
    fn status_code(&self) -> u16 {
        self.status_code
    }

    fn body(&self) -> Option<String> {
        self.body.clone()
    }

    fn into_response(&self) -> Response {
        let body = match self.body() {
            Some(b) => b,
            None => String::new(),
        };

        Response::builder()
            .status(StatusCode::from_u16(self.status_code).unwrap())
            .body(body)
            .into_response()
    }
}

impl Default for MockOptions {
    fn default() -> Self {
        Self { response: None }
    }
}
