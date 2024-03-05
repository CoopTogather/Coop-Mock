use std::sync::Arc;

use endpoint_handler::{caching::TemplateCaching, endpoint_template::Template};
use poem::{handler, http::StatusCode, web::Data, IntoResponse, Request};

use crate::utils::mock_handler::DatabaseMockHandlerImpl;

#[handler]
pub async fn handle_mock_request(
    req: &Request,
    mocks_handler: Data<&Arc<DatabaseMockHandlerImpl>>,
) -> impl IntoResponse {
    let path = req.uri().path();
    let mocks_handler = mocks_handler.clone();

    let match_template = mocks_handler
        .caching
        .find_template(path, req.method().as_str());

    match match_template {
        Some(template) => template.into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}
