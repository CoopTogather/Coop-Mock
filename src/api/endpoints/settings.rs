use std::sync::Arc;

use coop_service::{container::AppContainer, domain::models::endpoints::CreateEndpointDto};
use poem::{
    handler,
    http::StatusCode,
    post,
    web::{Data, Json},
    IntoResponse, Route,
};

use crate::api::models::settings_models::CreateSettingsRequestDto;

pub fn settings_routes() -> Route {
    Route::new().at("/endpoint", post(create_mock))
}

#[handler]
pub async fn create_mock(
    Json(create_dto): Json<CreateSettingsRequestDto>,
    app_container: Data<&Arc<AppContainer>>,
) -> impl IntoResponse {
    let settings_service = &app_container.services_container.settings_service;

    let result = settings_service
        .create_mock(CreateEndpointDto {
            name: create_dto.name,
            path: create_dto.path,
            options: create_dto.options,
            enabled: create_dto.enabled,
            method: create_dto.method,
            description: None,
        })
        .await;

    match result {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}