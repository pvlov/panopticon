pub mod random;

use std::time::Duration;

use serde::Serialize;
use uuid::Uuid;

use crate::scenario::Scenario;
type CustomerID = Uuid;

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

pub trait ScenarioSolver {
    fn solve(&self, scenario: &Scenario) -> Box<dyn Iterator<Item = VehicleTask>>;
}
