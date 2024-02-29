use super::CommandModel;
use crate::infrastructure::models::end_points;
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateEndpointDto {
    pub name: String,
    pub path: String,
    pub method: String,
    pub enabled: bool,
    pub description: Option<String>,
    pub options: Option<serde_json::Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EndpointDto {
    pub name: String,
    pub path: String,
    pub method: String,
    pub description: Option<String>,
    pub options: Option<serde_json::Value>,
}

impl EndpointDto {
    pub fn from(model: end_points::Model) -> Self {
        Self {
            name: model.name,
            path: model.path,
            method: model.method,
            options: model.options,
            description: model.description,
        }
    }

    pub fn from_option(model: Option<end_points::Model>) -> Option<Self> {
        match model {
            Some(inner_model) => Some(Self {
                name: inner_model.name,
                path: inner_model.path,
                method: inner_model.method,
                options: inner_model.options,
                description: inner_model.description,
            }),
            None => None,
        }
    }
}

impl CommandModel<end_points::ActiveModel> for CreateEndpointDto {
    fn to_entity_model(self) -> end_points::ActiveModel {
        end_points::ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(self.name),
            path: ActiveValue::Set(self.path),
            method: ActiveValue::Set(self.method),
            enabled: ActiveValue::Set(self.enabled),
            description: ActiveValue::Set(self.description),
            options: ActiveValue::Set(self.options),
            created_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
            updated_at: ActiveValue::Set(chrono::Utc::now().naive_utc()),
        }
    }
}
