use poem::{http::StatusCode, IntoResponse, Response};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockResponseImpl {
    status_code: u16,
    body: Option<serde_json::Value>,
}

/// Represents a trait for generating mock responses.
pub trait MockResponse {
    /// Returns the status code of the response.
    fn status_code(&self) -> u16;

    /// Returns the body of the response, if any.
    fn body(&self) -> Option<String>;
}

impl MockResponse for MockResponseImpl {
    fn status_code(&self) -> u16 {
        self.status_code
    }

    fn body(&self) -> Option<String> {
        match &self.body {
            Some(b) => Some(b.to_string()),
            None => None,
        }
    }
}

impl IntoResponse for MockResponseImpl {
    fn into_response(self) -> Response {
        let body = match self.body() {
            Some(b) => b,
            None => String::new(),
        };

        Response::builder()
            .status(StatusCode::from_u16(self.status_code).unwrap())
            .content_type("application/json")
            .body(body)
            .into_response()
    }
}
