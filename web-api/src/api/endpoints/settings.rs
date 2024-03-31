use coop_service::{
    container::AppContainer,
    domain::models::endpoints::{
        CreateEndpointDto, SearchEndpointRequestDto, UpdateEndpointRequestDto,
    },
};
use endpoint_handler::{endpoint_template::options::MockOptions, utils::validator::Validator};
use poem::{
    delete, get, handler,
    http::StatusCode,
    patch, post, put,
    web::{Data, Json, Path},
    Body, IntoResponse, Request, Response, Route,
};

use std::sync::Arc;

pub fn settings_routes() -> Route {
    Route::new()
        .at("/endpoints", get(get_mocks))
        .at("/endpoint/add", post(create_mock))
        .at("/endpoint/edit", put(update_mock))
        .at("/endpoint/:id/toggle", patch(toggle_mock))
        .at("/endpoint/:id", delete(delete_mock))
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
) -> impl IntoResponse {
    let settings_service = &app_container.services_container.settings_service;

    if update_request.options.is_some() {
        let options = update_request.options.as_ref().unwrap();

        if !MockOptions::is_valid(options) {
            return StatusCode::BAD_REQUEST.into_response();
        }
    }

    match settings_service.update_mock(update_request).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::BAD_REQUEST.into_response(),
    }
}

#[handler]
pub async fn delete_mock(
    Path(id): Path<i32>,
    app_container: Data<&Arc<AppContainer>>,
) -> impl IntoResponse {
    let settings_service = &app_container.services_container.settings_service;

    match settings_service.delete_mock(id).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::BAD_REQUEST.into_response(),
    }
}

#[handler]
pub async fn toggle_mock(
    Path(id): Path<i32>,
    app_container: Data<&Arc<AppContainer>>,
) -> impl IntoResponse {
    let settings_service = &app_container.services_container.settings_service;

    match settings_service.toggle_mock(id).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::BAD_REQUEST.into_response(),
    }
}
