use std::sync::Arc;

use container::AppContainer;
use poem::{listener::TcpListener, middleware::AddData, EndpointExt, Route, Server};

mod api;
mod container;
mod domain;
mod infrastructure;
mod schema;
mod services;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().nest("/settings", api::endpoints::settings::settings_routes());

    let app = app.with(AddData::new(Arc::new(AppContainer::new())));

    Server::new(TcpListener::bind("0.0.0.0:3033"))
        .run(app)
        .await
}
