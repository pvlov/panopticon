pub mod random;
pub mod nearest;

use std::time::Duration;

use serde::Serialize;
use uuid::Uuid;

use crate::{scenario::Scenario, CustomerID};

#[derive(Debug, Clone, Serialize)]
pub enum TaskAction {
    Assignment(CustomerID),
    Idle(Duration),
}

#[derive(Debug, Clone, Serialize)]
pub struct VehicleTask {
    pub vehicle_id: Uuid,
    pub action: TaskAction,
}

pub trait ScenarioSolver: Send + Sync {
    fn solve(&self, scenario: &Scenario) -> Box<dyn Iterator<Item = VehicleTask>>;
}
