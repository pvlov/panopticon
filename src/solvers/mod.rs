pub mod nearest;
pub mod random;

use std::time::Duration;

use serde::Serialize;
use uuid::Uuid;

use crate::{
    scenario::{Position, Scenario},
    CustomerID,
};

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

struct Solution<'a> {
    tasks: Vec<VehicleTask>,
    scenario: Scenario<'a>,
}

impl<'a> Solution<'a> {
    pub fn measure_waits(&self) -> Vec<f64> {
        // Fix vehicle speed to unit because they are random
        use std::collections::HashMap;

        fn distance(pos1: Position, pos2: Position) -> f64 {
            ((pos1.x - pos2.x).powi(2) + (pos1.y - pos2.y).powi(2)).sqrt()
        }

        let mut vehicle_positions: HashMap<Uuid, Position> = self
            .scenario
            .vehicles
            .iter()
            .map(|(id, v)| (*id, v.pos))
            .collect();

        let mut vehicle_available_times: HashMap<Uuid, f64> = self
            .scenario
            .vehicles
            .iter()
            .map(|(id, _)| (*id, 0.0))
            .collect();

        let mut customer_wait_times: HashMap<Uuid, f64> = HashMap::new();

        for task in &self.tasks {
            match &task.action {
                TaskAction::Assignment(customer_id) => {
                    let vehicle_id = task.vehicle_id;
                    let vehicle_pos = *vehicle_positions.get(&vehicle_id).unwrap();
                    let vehicle_available_time = *vehicle_available_times.get(&vehicle_id).unwrap();
                    let customer = self.scenario.customers.get(customer_id).unwrap();

                    let distance_to_customer = distance(vehicle_pos, customer.pos);
                    let travel_time_to_customer = distance_to_customer;

                    let t_pickup = vehicle_available_time + travel_time_to_customer;
                    customer_wait_times.insert(*customer_id, t_pickup);

                    let distance_to_destination = distance(customer.pos, customer.dest);
                    let travel_time_to_destination = distance_to_destination;

                    let t_dropoff = t_pickup + travel_time_to_destination;
                    vehicle_positions.insert(vehicle_id, customer.dest);
                    vehicle_available_times.insert(vehicle_id, t_dropoff);
                }
                TaskAction::Idle(duration) => {
                    let vehicle_id = task.vehicle_id;
                    let vehicle_available_time = *vehicle_available_times.get(&vehicle_id).unwrap();
                    let idle_time = duration.as_secs_f64();
                    vehicle_available_times.insert(vehicle_id, vehicle_available_time + idle_time);
                }
            }
        }

        self.scenario
            .customers
            .values()
            .map(|customer| *customer_wait_times.get(&customer.id).unwrap_or(&0.0))
            .collect()
    }
}


fn calc_wait_stats(wait_times: Vec<f64>) -> (f64, f64, f64, f64) {
    let min = wait_times.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max = wait_times.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let avg = wait_times.iter().sum::<f64>() / wait_times.len() as f64;
    let median = {
        let mut sorted = wait_times.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mid = sorted.len() / 2;
        if sorted.len() % 2 == 0 {
            (sorted[mid] + sorted[mid - 1]) / 2.0
        } else {
            sorted[mid]
        }
    };
    (min, max, avg, median)
}
