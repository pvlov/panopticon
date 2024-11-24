use std::{collections::HashMap, sync::Arc, time::Duration};

use anyhow::anyhow;
use scenario_runner::models::StandardMagentaVehicleDto;
use tokio::task;

use crate::{
    api_wrapper::ApiWrapper,
    scenario::{Position, Scenario},
    solvers::{ScenarioSolver, TaskAction},
    AppContext, VehicleID,
};

pub enum SolverChoice {
    RandomSolver,
}

pub struct ScenarioManager<'a> {
    scenario: Arc<Scenario<'a>>,
    solver: Box<dyn ScenarioSolver>,
    api: Arc<ApiWrapper>,
    app_ctx: Arc<AppContext>,
}

impl<'a> ScenarioManager<'a> {
    pub fn new(
        scenario: Arc<Scenario<'a>>,
        solver: Box<dyn ScenarioSolver>,
        api: Arc<ApiWrapper>,
        app_ctx: Arc<AppContext>,
    ) -> Self {
        ScenarioManager {
            scenario,
            solver,
            api,
            app_ctx,
        }
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        // initialise scenario
        log::debug!("Initialising scenario {}", self.scenario.id);
        self.api.initialize_scenario(self.scenario.id).await?;

        // launch scenario
        log::debug!("Launching scenario {}", self.scenario.id);
        self.api
            .launch_scenario(self.scenario.id, Some(0.2))
            .await?;

        let mut last_tasks = HashMap::<VehicleID, task::JoinHandle<anyhow::Result<()>>>::new();

        log::debug!("Lazily solving scenario {}", self.scenario.id);
        let instructions = self.solver.solve(&self.scenario);

        for instruction in instructions {
            let scenario_id = self.scenario.id;
            let vehicle_id = instruction.vehicle_id;
            let last_future = last_tasks.remove(&vehicle_id);
            let api = self.api.clone();

            let task: task::JoinHandle<anyhow::Result<()>> = task::spawn(async move {
                // await the vehicles last Future so that we don't send
                // anything until the previous Future has completed
                if let Some(handle) = last_future {
                    let _ = handle.await?; // an Error will propagate down the task chain
                }

                loop {
                    let vehicle_info = api.get_vehicle(vehicle_id).await?;
                    let status = get_vehicle_status(&vehicle_info);

                    if let SimpleVehicleStatus::Idle = status {
                        break;
                    } else {
                        let remaining_time_f64 = vehicle_info
                            .remaining_travel_time
                            .ok_or_else(|| anyhow!("Mispredicted vehicle status"))?;

                        let remaining_time = Duration::from_secs_f64(remaining_time_f64);

                        tokio::time::sleep(remaining_time).await;
                    }
                }

                log::trace!("Sending {:?} to Vehicle {}", &instruction, vehicle_id);

                // send the instruction(/wait)

                match instruction.action {
                    TaskAction::Assignment(customer_id) => {
                        api.assign_vehicle(scenario_id, vehicle_id, customer_id)
                            .await?
                    }
                    TaskAction::Idle(duration) => tokio::time::sleep(duration).await,
                };

                Ok(())
            });

            last_tasks.insert(vehicle_id, task);
        }

        // wait for all tasks to complete
        for (_, task) in last_tasks {
            let _ = task.await?;
        }

        Ok(())
    }
}

fn haversine_dist(pos1: Position, pos2: Position) -> f64 {
    let r = 6378.137; // Radius of Earth in km

    let lat1 = pos1.x;
    let lon1 = pos1.y;
    let lat2 = pos2.x;
    let lon2 = pos2.y;

    let d_lat = (lat2 - lat1).to_radians();
    let d_lon = (lon2 - lon1).to_radians();

    let a = (d_lat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (d_lon / 2.0).sin().powi(2);

    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    let distance_km = r * c;

    distance_km * 1000.0 // metres
}

// the references got too complex and I don't always need all that info
#[derive(Debug, Clone, Copy)]
pub enum SimpleVehicleStatus {
    DrivingToDestination,
    DrivingToCustomer,
    Idle,
}

// TODO alt: check remaining travel time?
pub fn get_vehicle_status(v: &StandardMagentaVehicleDto) -> SimpleVehicleStatus {
    if v.customer_id.is_some() {
        SimpleVehicleStatus::DrivingToDestination
    } else if let Some(speed) = v.vehicle_speed {
        if speed > 0.0 {
            SimpleVehicleStatus::DrivingToCustomer
        } else {
            SimpleVehicleStatus::Idle
        }
    } else {
        SimpleVehicleStatus::Idle
    }
}
