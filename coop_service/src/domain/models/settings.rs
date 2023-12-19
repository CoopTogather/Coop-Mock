use crate::infrastructure::models::settings::CreateEndpointSettingsDiesel;

use super::CommandModel;

#[derive(Clone)]
pub struct CreateSettingsDto {
    pub name: String,
    pub path: String,
    pub method: String,
    pub enabled: bool,
    pub options: Option<serde_json::Value>,
}

impl CommandModel<CreateEndpointSettingsDiesel> for CreateSettingsDto {
    fn to_diesel_model(self) -> CreateEndpointSettingsDiesel {
        CreateEndpointSettingsDiesel {
            name: self.name,
            path: self.path,
            method: self.method,
            enabled: self.enabled,
            options: self.options,
            created_at: chrono::Utc::now().naive_utc().into(),
            updated_at: chrono::Utc::now().naive_utc().into(),
        }
    }
}
