use poem::{handler, http::StatusCode, IntoResponse, Request};

pub mod endpoint_template;

const MOCK_PATH: &str = "/mock/{id:u32}";

#[handler]
pub async fn handle_mock_request(req: &Request) -> impl IntoResponse {
    let path = req.uri().path();

    if path == MOCK_PATH {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}
