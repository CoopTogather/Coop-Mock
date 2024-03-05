use poem::{handler, http::StatusCode, web::Data, IntoResponse, Request};
use std::sync::Arc;

use crate::utils::mock_handler::MockEndpointHandler;

#[handler]
pub async fn handle_mock_request(
    req: &Request,
    mocks_handler: Data<&Arc<dyn MockEndpointHandler>>,
) -> impl IntoResponse {
    let mocks_handler = mocks_handler.clone();

    let response = mocks_handler.handle_mock_request(req).await;

    if response.is_none() {
        return StatusCode::NOT_FOUND.into_response();
    }

    response.unwrap()
}
