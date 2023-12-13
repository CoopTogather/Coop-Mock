#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CreateSettingsRequestDto {
    pub name: String,
    pub path: String,
    pub options: Option<serde_json::Value>,
}
