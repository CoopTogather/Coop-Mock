#[derive(Clone)]
pub struct CreateSettings {
    pub name: String,
    pub path: String,
    pub options: Option<serde_json::Value>,
}
