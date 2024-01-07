use coop_service::container::AppContainer;
use endpoint_handler::handle_mock_request;
use poem::{get, listener::TcpListener, middleware::AddData, EndpointExt, Route, Server};
use std::sync::Arc;

mod api;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/api/*", handle_mock_request)
        .nest("/settings", api::endpoints::settings::settings_routes());

    let container = Arc::new(AppContainer::new().await);

    let app = app.with(AddData::new(container));

    Server::new(TcpListener::bind("0.0.0.0:3033"))
        .run(app)
        .await
}
