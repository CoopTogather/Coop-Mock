use diesel::prelude::*;

use crate::domain::models::settings::CreateSettings;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::endpoints_setting)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EndpointSettings {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub options: serde_json::Value,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::endpoints_setting)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateEndpointSettingsDiesel {
    pub name: String,
    pub path: String,
    pub options: serde_json::Value,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl CreateEndpointSettingsDiesel {
    pub fn from(create_settings: CreateSettings) -> Self {
        Self {
            name: create_settings.name,
            path: create_settings.path,
            options: create_settings.options,
            created_at: chrono::Utc::now().naive_utc().into(),
            updated_at: chrono::Utc::now().naive_utc().into(),
        }
    }
}
