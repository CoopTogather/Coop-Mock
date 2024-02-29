use serde::{Deserialize, Serialize};

use super::response::MockResponseImpl;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockOptions {
    pub response: Option<MockResponseImpl>,
}

impl Default for MockOptions {
    fn default() -> Self {
        Self { response: None }
    }
}
