use coop_service::{container::AppContainer, domain::models::endpoints::CreateEndpointDto};
use poem::{
    get, handler,
    http::StatusCode,
    post,
    web::{Data, Json},
    Body, IntoResponse, Response, Route,
};

use std::sync::Arc;

pub fn settings_routes() -> Route {
    Route::new()
        .at("/endpoints", get(get_mocks))
        .at("/endpoint", post(create_mock))
}

#[handler]
pub async fn create_mock(
    Json(create_dto): Json<CreateEndpointDto>,
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

#[handler]
pub async fn get_mocks(app_container: Data<&Arc<AppContainer>>) -> impl IntoResponse {
    let settings_service = &app_container.services_container.settings_service;

    let result = settings_service.get_mocks().await;

    match result {
        Ok(endpoints) => Response::builder()
            .status(StatusCode::OK)
            .content_type("application/json")
            .body(Body::from_string(
                serde_json::to_string(&endpoints).unwrap(),
            )),
        Err(_) => Response::builder().status(StatusCode::NOT_FOUND).finish(),
    }
}
