use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::infrastructure::schema::endpoints_setting)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EndpointSettings {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub options: Option<serde_json::Value>,
    pub method: String,
    pub enabled: bool,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::infrastructure::schema::endpoints_setting)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateEndpointSettingsDiesel {
    pub name: String,
    pub path: String,
    pub options: Option<serde_json::Value>,
    pub method: String,
    pub enabled: bool,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl CreateEndpointSettingsDiesel {
    // pub fn from(create_settings: CreateSettingsDto) -> Self {
    //     Self {
    //         name: create_settings.name,
    //         path: create_settings.path,
    //         options: create_settings.options,
    //         method: create_settings.method,
    //         enabled: create_settings.enabled,
    //         created_at: chrono::Utc::now().naive_utc().into(),
    //         updated_at: chrono::Utc::now().naive_utc().into(),
    //     }
    // }
}
