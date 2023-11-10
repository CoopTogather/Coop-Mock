use actix_web::{
    get,
    web::{resource, route},
    App, HttpResponse, HttpServer, Responder,
};

mod container;
mod services;
mod schema;

#[get("/")]
async fn mock_action() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(mock_action)
            .service(resource("/mocks").get(services::mocks_service::MockService::create_mock))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
