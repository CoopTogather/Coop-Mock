use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct EndpointJsonModel {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub method: String,
    pub description: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub enabled: bool,
    pub options: Option<Value>,
}
