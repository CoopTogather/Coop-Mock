use actix_web::{
    get,
    web::{resource, route},
    App, HttpResponse, HttpServer, Responder,
};

mod container;
mod infrastructure;
mod schema;
mod services;

#[get("/")]
async fn mock_action() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(container::AppContainer::new())
            .service(mock_action)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
