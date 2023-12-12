use std::sync::Arc;

use poem::{middleware::AddData, EndpointExt, Route};

mod api;
mod container;
mod domain;
mod infrastructure;
mod schema;
mod services;

#[tokio::main]
async fn main() {
    let app = Route::new();
    let app_container = AddData::new(Arc::new(container::AppContainer::new()));

    app.with(app_container);
}
