use std::{collections::HashMap, sync::Arc};

use actix_web::{error::ErrorInternalServerError, get, web, HttpResponse, Responder, Result};
use scenario_runner::models::ScenarioDto;
use uuid::Uuid;

use crate::{
    scenario::{Customer, Scenario, Vehicle},
    scenario_manager::ScenarioManager,
    solvers::{random::RandomSolver, ScenarioSolver},
    AppContext, VehicleID,
};

#[get("/health")]
async fn health() -> impl Responder {
    "UP"
}

// #[get("/api/set_scenario_runner_uri/{uri}")]
// async fn set_scenario_runner_uri(
//     req_params: web::Path<String>,
//     ctx: web::Data<AppContext>,
// ) -> impl Responder {
//     let api_str: String = req_params.into_inner();
//     let Ok(api_uri) = Uri::from_str(&api_str) else {
//         return HttpResponse::InternalServerError().body("Could not parse given url");
//     };

//     ctx.scenario_runner_endpoint.write().await.replace(api_uri);

//     HttpResponse::Ok().finish()
// }

#[get("/api/run_scenario/{scenario_id}/{solver}")]
async fn run_scenario(
    req_params: web::Path<(Uuid, String)>,
    ctx: web::Data<AppContext>,
) -> Result<impl Responder> {
    let (scenario_id, solver_name) = req_params.into_inner();

    let solver: Box<dyn ScenarioSolver> = match solver_name.to_lowercase().as_str() {
        "random" => Box::new(RandomSolver),
        _ => todo!(),
    };

    let ctx = ctx.into_inner();

    let scenario = ctx
        .api_wrapper
        .get_scenario(scenario_id)
        .await
        .map_err(ErrorInternalServerError)?;

    let scenario = scenario.try_into().map_err(ErrorInternalServerError)?;

    let api = Arc::new(ctx.api_wrapper.clone());

    let scenario_manager = ScenarioManager::new(Arc::new(scenario), solver, api, ctx.clone());

    tokio::spawn(async move {
        scenario_manager.start().await.unwrap();
    });

    Ok(HttpResponse::Ok().body(format!("Started scenario ID: {}", scenario_id)))
}

#[get("/api/list_scenarios")]
async fn list_scenarios(ctx: web::Data<AppContext>) -> Result<impl Responder> {
    let ctx = ctx.into_inner();

    let scenarios = ctx
        .api_wrapper
        .get_scenarios()
        .await
        .map_err(ErrorInternalServerError)?;

    serde_json::to_string(&scenarios).map_err(ErrorInternalServerError)
}

#[get("/api/get_scenario/{id}")]
async fn get_scenario(
    req_params: web::Path<Uuid>,
    ctx: web::Data<AppContext>,
) -> Result<impl Responder> {
    let id = req_params.into_inner();
    let ctx = ctx.into_inner();

    let scenario = ctx
        .api_wrapper
        .get_scenario(id)
        .await
        .map_err(ErrorInternalServerError)?;

    serde_json::to_string(&scenario).map_err(ErrorInternalServerError)
}

impl<'a> TryFrom<ScenarioDto> for Scenario<'a> {
    type Error = anyhow::Error;

    fn try_from(dto: ScenarioDto) -> anyhow::Result<Self> {
        // pub vehicles: HashMap<Uuid, Vehicle<'a>>,
        // pub customers: HashMap<Uuid, Customer>,

        let vehicles: HashMap<VehicleID, Vehicle<'a>> = dto
            .vehicles
            .into_iter()
            .map(Vehicle::try_from)
            .map(|res| res.map(|v| (v.id, v)))
            .collect::<Result<HashMap<_, _>, _>>()?;

        let customers: HashMap<Uuid, Customer> = dto
            .customers
            .into_iter()
            .map(Customer::try_from)
            .map(|res| res.map(|c| (c.id, c)))
            .collect::<Result<HashMap<_, _>, _>>()?;

        Ok(Scenario {
            id: dto.id,
            vehicles,
            customers,
            status: dto.status,
        })
    }
}
