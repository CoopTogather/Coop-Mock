#[derive(Clone)]
pub struct CreateSettings {
    pub name: String,
    pub path: String,
    pub options: serde_json::Value,
}
