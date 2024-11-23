use rand::{seq::SliceRandom, thread_rng};
use uuid::Uuid;

use crate::scenario::{Customer, Scenario};

use super::{ScenarioSolver, VehicleAssignment, VehicleTask};

pub struct RandomSolver;

impl ScenarioSolver for RandomSolver {
    fn solve(&self, scenario: &Scenario) -> Box<dyn Iterator<Item = VehicleTask>> {
        let mut customer_ids: Vec<Uuid> = scenario.customers.values().map(|c| c.id).collect();
        customer_ids.shuffle(&mut thread_rng());

        let vehicle_ids: Vec<Uuid> = scenario.vehicles.values().map(|v| v.id).collect();

        let assignments = customer_ids
            .into_iter()
            .zip(vehicle_ids.into_iter().cycle())
            .map(|(c_id, v_id)| VehicleAssignment {
                customer: c_id,
                vehicle: v_id,
            })
            .map(VehicleTask::Assignment);

        Box::new(assignments)
    }
}
