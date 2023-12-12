use poem::{IntoResponse, Route, post, handler};

pub fn settings_routes() -> Route {
    Route::new().at("/settings", post(create_mock))
}

#[handler]
pub async fn create_mock() -> impl IntoResponse {
    "Hello, world!"
}
