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
        .get_init_scenario(scenario_id)
        .await
        .map_err(ErrorInternalServerError)?;

    let scenario = scenario.try_into().map_err(ErrorInternalServerError)?;

    let api = Arc::new(ctx.api_wrapper.clone());

    let scenario_manager = ScenarioManager::new(Arc::new(scenario), solver, api, ctx);

    tokio::spawn(async move {
        let res = scenario_manager.start().await;

        if let Err(e) = res {
            log::error!("Error running scenario: {:?}", e);
            panic!("Error running scenario: {:?}", e);
        }
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

#[get("/api/get_init_scenario/{id}")]
async fn get_init_scenario(
    req_params: web::Path<Uuid>,
    ctx: web::Data<AppContext>,
) -> Result<impl Responder> {
    let id = req_params.into_inner();
    let ctx = ctx.into_inner();

    let scenario = ctx
        .api_wrapper
        .get_init_scenario(id)
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&scenario).map_err(ErrorInternalServerError)?))
}

#[get("/api/get_started_scenario/{id}")]
async fn get_started_scenario(
    req_params: web::Path<Uuid>,
    ctx: web::Data<AppContext>,
) -> Result<impl Responder> {
    let id = req_params.into_inner();
    let ctx = ctx.into_inner();

    let scenario_dto = ctx
        .api_wrapper
        .get_started_scenario_str(id)
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(scenario_dto))
}

#[get("/api/get_current_scenario")]
async fn get_current_scenario(ctx: web::Data<AppContext>) -> Result<impl Responder> {
    let ctx = ctx.into_inner();

    let read = ctx.current_scenario_id.read().await;
    dbg!(&read);
    let scenario_id = read.ok_or_else(|| ErrorInternalServerError("No scenario running"))?;

    let scenario_dto = ctx
        .api_wrapper
        .get_started_scenario_str(scenario_id)
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(scenario_dto))
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
