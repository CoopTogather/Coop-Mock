use std::sync::Arc;

use endpoint_handler::caching::TemplateCaching;
use poem::{handler, http::StatusCode, web::Data, IntoResponse, Request};

use crate::utils::mock_handler::MockEndpointsHandler;

#[handler]
pub async fn handle_mock_request(
    req: &Request,
    mocks_handler: Data<&Arc<MockEndpointsHandler>>,
) -> impl IntoResponse {
    let path = req.uri().path();
    let mocks_handler = mocks_handler.clone();

    let resp = mocks_handler.caching.get(path);

    match resp {
        Some(_) => StatusCode::OK,
        None => StatusCode::NOT_FOUND,
    }
}
