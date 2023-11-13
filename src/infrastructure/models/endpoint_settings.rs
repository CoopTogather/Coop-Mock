use diesel::prelude::*;

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
