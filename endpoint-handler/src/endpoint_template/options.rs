use serde::{Deserialize, Serialize};

use crate::utils::validator::Validator;

use super::response::MockResponseImpl;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockOptions {
    pub response: Option<MockResponseImpl>,
}

impl Validator<&serde_json::Value> for MockOptions {
    fn is_valid(value: &serde_json::Value) -> bool {
        serde_json::from_value::<MockOptions>(value.clone()).is_ok()
    }
}

impl Default for MockOptions {
    fn default() -> Self {
        Self { response: None }
    }
}
