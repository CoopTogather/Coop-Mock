use coop_service::container::AppContainer;
use endpoint_handler::handle_mock_request;
use poem::{listener::TcpListener, middleware::AddData, EndpointExt, Route, Server};
use std::sync::Arc;

mod api;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/*", handle_mock_request)
        .nest("/settings", api::endpoints::settings::settings_routes());

    let app = app.with(AddData::new(Arc::new(AppContainer::new())));

    Server::new(TcpListener::bind("0.0.0.0:3033"))
        .run(app)
        .await
}
