#[derive(Clone)]
pub struct CreateSettingsDto {
    pub name: String,
    pub path: String,
    pub options: Option<serde_json::Value>,
}
