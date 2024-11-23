use actix_web::{get, web, Responder};

#[get("/health")]
async fn health() -> impl Responder {
    "UP"
}

#[get("/api/taxis/{id}")]
async fn taxis(id: web::Path<u32>) -> impl Responder {
    format!("Taxi ID: {}", id)
}
