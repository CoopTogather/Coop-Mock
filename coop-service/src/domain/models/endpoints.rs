use super::{InsertCommandModel, UpdateCommandModel};
use crate::infrastructure::models::end_points;
use sea_orm::{ActiveValue, Set};
use serde::{Deserialize, Serialize};

/// DTO for creating an endpoint.
#[derive(Clone, Serialize, Deserialize)]
pub struct CreateEndpointDto {
    pub name: String,
    pub path: String,
    pub method: String,
    pub enabled: bool,
    pub description: Option<String>,
    pub options: Option<serde_json::Value>,
}

/// DTO for updating an endpoint (used internally).
#[derive(Clone, Default)]
pub struct UpdateEndpointDto {
    pub id: i32,
    pub name: Option<String>,
    pub path: Option<String>,
    pub method: Option<String>,
    pub description: Option<String>,
    pub options: Option<serde_json::Value>,
}

/// DTO for searching endpoints (used internally).
#[derive(Clone, Default)]
pub struct SearchEndpointDto {
    pub name: Option<String>,
    pub path: Option<String>,
}

/// DTO representing an endpoint.
#[derive(Clone, Serialize, Deserialize)]
pub struct EndpointDto {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub method: String,
    pub description: Option<String>,
    pub options: Option<serde_json::Value>,
}

impl EndpointDto {
    /// Converts an `end_points::Model` to an `EndpointDto`.
    pub fn from(model: end_points::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            path: model.path,
            method: model.method,
            options: model.options,
            description: model.description,
        }
    }

    /// Converts an optional `end_points::Model` to an optional `EndpointDto`.
    pub fn from_option(model: Option<end_points::Model>) -> Option<Self> {
        match model {
            Some(inner_model) => Some(Self {
                id: inner_model.id,
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

impl InsertCommandModel<end_points::ActiveModel> for CreateEndpointDto {
    /// Converts a `CreateEndpointDto` to an `end_points::ActiveModel`.
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

impl UpdateCommandModel<end_points::ActiveModel, UpdateEndpointDto> for UpdateEndpointDto {
    /// Sets the update values of an `end_points::ActiveModel` based on the fields of the `UpdateEndpointDto`.
    fn set_update_active_model(
        &self,
        mut active_model: end_points::ActiveModel,
    ) -> end_points::ActiveModel {
        if self.name.is_some() {
            active_model.name = Set(self.name.to_owned().unwrap());
        }

        if self.path.is_some() {
            active_model.path = Set(self.path.to_owned().unwrap());
        }

        active_model.description = Set(self.to_owned().description);

        active_model.options = Set(self.to_owned().options);

        active_model
    }
}
