use std::sync::Arc;

use poem::{handler, http::StatusCode, post, web::Data, IntoResponse, Route};

use crate::{container::AppContainer, domain::models::settings::CreateSettings};

pub fn settings_routes() -> Route {
    Route::new().at("/endpoint", post(create_mock))
}

#[handler]
pub async fn create_mock(app_container: Data<&Arc<AppContainer>>) -> impl IntoResponse {
    let settings_service = &app_container.services_container.settings_service;

    let result = settings_service
        .create_mock(CreateSettings {
            name: "test".to_string(),
            path: "/test".to_string(),
            options: Option::None,
        })
        .await;

    match result {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}
