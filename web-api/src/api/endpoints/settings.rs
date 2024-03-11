use coop_service::{
    container::AppContainer,
    domain::models::endpoints::{
        CreateEndpointDto, SearchEndpointRequestDto, UpdateEndpointRequestDto,
    },
};
use poem::{
    delete, get, handler,
    http::StatusCode,
    patch, post, put,
    web::{Data, Json},
    Body, IntoResponse, Request, Response, Route,
};

use std::sync::Arc;

pub fn settings_routes() -> Route {
    Route::new()
        .at("/endpoints", get(get_mocks))
        .at("/endpoint", post(create_mock))
        .at("/endpoint", put(update_mock))
        .at("/endpoint/toggle", patch(toggle_mock))
        .at("/endpoint", delete(delete_mock))
}

#[handler]
pub async fn create_mock(
    Json(create_dto): Json<CreateEndpointDto>,
    app_container: Data<&Arc<AppContainer>>,
) -> impl IntoResponse {
    let settings_service = &app_container.services_container.settings_service;

    let result = settings_service.create_mock(create_dto).await;

    match result {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}

#[handler]
pub async fn get_mocks(
    req: &Request,
    app_container: Data<&Arc<AppContainer>>,
) -> impl IntoResponse {
    let settings_service = &app_container.services_container.settings_service;
    let search_params = req.params::<SearchEndpointRequestDto>().unwrap_or_default();

    let result = settings_service.get_mocks(search_params).await;

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

#[handler]
pub async fn update_mock(
    Json(update_request): Json<UpdateEndpointRequestDto>,
    app_container: Data<&Arc<AppContainer>>,
) {
    let settings_service = &app_container.services_container.settings_service;

    let result = settings_service.update_mock(update_request).await;
}

#[handler]
pub async fn delete_mock(app_container: Data<&Arc<AppContainer>>) {}

#[handler]
pub async fn toggle_mock(app_container: Data<&Arc<AppContainer>>) {}
