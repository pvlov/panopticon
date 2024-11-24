mod api_wrapper;
mod handler;
mod scenario;
mod scenario_manager;
mod solvers;


use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use api_wrapper::ApiWrapper;
use tokio::sync::RwLock;
use uuid::Uuid;

pub type CustomerID = Uuid;
pub type VehicleID = Uuid;
pub type ScenarioID = Uuid;

#[derive(Clone, Default, Debug)]
struct Metrics {
    // num_
}

#[derive(Default, Debug)]
struct AppContext {
    api_wrapper: ApiWrapper,
    metrics: RwLock<Option<Metrics>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(move || {
        let cors = Cors::default();
        let app_data = AppContext::default();
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .wrap(middleware::NormalizePath::default())
            .app_data(web::Data::new(app_data))
            .service(handler::health)
            .service(handler::run_scenario)
            .service(handler::list_scenarios)
            .service(handler::get_init_scenario)
            .service(handler::get_started_scenario)
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}
