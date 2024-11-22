use actix_web::{get, Responder};


#[get("/health")]
async fn health() -> impl Responder {
    "UP"
}
