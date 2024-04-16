use coop_service::domain::models::endpoints::{
    CreateEndpointDto, SearchEndpointDto, UpdateEndpointDto,
};
use serde::{Deserialize, Serialize};

/// DTO for updating an endpoint.
#[derive(Clone, Serialize, Deserialize)]
pub struct UpdateEndpointRequestDto {
    pub id: i32,
    pub name: Option<String>,
    pub path: Option<String>,
    pub method: Option<String>,
    pub description: Option<String>,
    pub options: Option<serde_json::Value>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SearchEndpointRequestDto {
    pub name: Option<String>,
    pub path: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateEndpointRequestDto {
    pub name: String,
    pub path: String,
    pub method: String,
    pub enabled: bool,
    pub description: Option<String>,
    pub options: Option<serde_json::Value>,
}

impl Into<CreateEndpointDto> for CreateEndpointRequestDto {
    fn into(self) -> CreateEndpointDto {
        CreateEndpointDto {
            name: self.name,
            path: self.path,
            method: self.method,
            enabled: self.enabled,
            description: self.description,
            options: self.options,
        }
    }
}

impl Into<SearchEndpointDto> for SearchEndpointRequestDto {
    fn into(self) -> SearchEndpointDto {
        SearchEndpointDto {
            name: self.name,
            path: self.path,
        }
    }
}

impl Default for SearchEndpointRequestDto {
    fn default() -> Self {
        Self {
            name: None,
            path: None,
        }
    }
}

impl Into<UpdateEndpointDto> for UpdateEndpointRequestDto {
    fn into(self) -> UpdateEndpointDto {
        UpdateEndpointDto {
            id: self.id,
            name: self.name,
            path: self.path,
            method: self.method,
            description: self.description,
            options: self.options,
        }
    }
}
