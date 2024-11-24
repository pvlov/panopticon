use std::collections::{HashMap, HashSet};

use rand::{seq::SliceRandom, thread_rng};
use rstar::{primitives::GeomWithData, Point, RTree};
use uuid::Uuid;

use crate::{
    scenario::{Customer, Position, Scenario},
    solvers::{calc_wait_stats, Solution},
};

use super::{ScenarioSolver, TaskAction, VehicleTask};

pub struct TreeSolver;

pub struct Node<'a> {
    pub state: Solution<'a>,
    pub children: Vec<&'a Node<'a>>,
}

impl ScenarioSolver for TreeSolver {
    fn solve(&self, scenario: &Scenario) -> Box<dyn Iterator<Item = VehicleTask>> {
        let start_time = std::time::Instant::now();

        let customer_ids: Vec<Uuid> = scenario.customers.values().map(|c| c.id).collect();

        let vehicle_ids: Vec<Uuid> = scenario.vehicles.values().map(|v| v.id).collect();

        // TODO: Implement tree solver lmao

        let elapsed = start_time.elapsed();

        let assignments: Vec<VehicleTask> = todo!();

        log::info!(
            "Tree solver took {} s for {} Customers and {} Vehicles",
            elapsed.as_secs_f64(),
            scenario.customers.len(),
            scenario.vehicles.len()
        );

        let solution = Solution {
            tasks: assignments.clone(),
            scenario: scenario.clone(),
        };
        let wait_times = solution.measure_waits();
        let (min, max, avg, median) = calc_wait_stats(wait_times);

        log::info!("Tree solver solution wait times stats:");
        log::info!("Min:\t\t{}", min);
        log::info!("Max:\t\t{}", max);
        log::info!("Avg:\t\t{}", avg);
        log::info!("Median:\t{}", median);

        Box::new(assignments.into_iter())
    }
}
