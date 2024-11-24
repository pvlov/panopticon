use rand::{seq::SliceRandom, thread_rng};
use uuid::Uuid;

use crate::{
    scenario::Scenario,
    solvers::{calc_wait_stats, Solution},
};

use super::{ScenarioSolver, TaskAction, VehicleTask};

pub struct RandomSolver;

impl ScenarioSolver for RandomSolver {
    fn solve(&self, scenario: &Scenario) -> Box<dyn Iterator<Item = VehicleTask>> {
        let start_time = std::time::Instant::now();

        let mut customer_ids: Vec<Uuid> = scenario.customers.values().map(|c| c.id).collect();
        customer_ids.shuffle(&mut thread_rng());

        let vehicle_ids: Vec<Uuid> = scenario.vehicles.values().map(|v| v.id).collect();

        let assignments = customer_ids
            .into_iter()
            .zip(vehicle_ids.into_iter().cycle())
            .map(|(c_id, v_id)| VehicleTask {
                vehicle_id: v_id,
                action: TaskAction::Assignment(c_id),
            });

        let assignments = assignments.collect::<Vec<_>>();

        let elapsed = start_time.elapsed();

        log::info!(
            "Random solver took {} s for {} Customers and {} Vehicles",
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

        log::info!("Random solver solution wait times stats:");
        log::info!("Min:\t\t{}", min);
        log::info!("Max:\t\t{}", max);
        log::info!("Avg:\t\t{}", avg);
        log::info!("Median:\t{}", median);

        Box::new(assignments.into_iter())
    }
}
