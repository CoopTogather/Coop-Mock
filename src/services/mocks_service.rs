use actix_web::{HttpResponse, Responder};

pub struct MockService;

impl MockService {
    pub async fn create_mock() -> impl Responder {
        println!("Mock created!");

        HttpResponse::Ok().body("Mock created!")
    }
}
