use std::fmt::Display;

use actix_web::{error::ErrorInternalServerError, get, web, HttpResponse, Responder, Result};
use uuid::Uuid;

use crate::AppContext;

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

#[get("/api/run_scenario_by_id/{scenario_id}")]
async fn run_scenario_by_id(
    req_params: web::Path<Uuid>,
    ctx: web::Data<AppContext>,
) -> Result<impl Responder> {
    let scenario_id = req_params.into_inner();
    let ctx = ctx.into_inner();

    let scenario = ctx
        .api_wrapper
        .get_scenario(scenario_id)
        .await
        .map_err(ErrorInternalServerError)?;


    Ok(HttpResponse::Ok().body(format!("Scenario ID: {}", scenario_id)))
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
