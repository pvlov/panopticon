mod api_wrapper;
mod handler;
mod scenario;
mod scenario_manager;
mod solvers;

use std::sync::Arc;

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
    api_wrapper: Arc<ApiWrapper>,
    metrics: RwLock<Option<Metrics>>,
    current_scenario_id: RwLock<Option<ScenarioID>>,
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
            .service(handler::get_current_scenario)
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}
