mod handler;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};

#[derive(Clone, Default)]
struct AppContext;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let app_data = AppContext::default();

    HttpServer::new(move || {
        let cors = Cors::default();
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(app_data.clone()))
            .service(handler::health)
            .service(handler::taxis)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
