use std::collections::{HashMap, HashSet};

use rand::{seq::SliceRandom, thread_rng};
use rstar::{primitives::GeomWithData, Point, RTree};
use uuid::Uuid;

use crate::{
    scenario::{Customer, Position, Scenario},
    solvers::{calc_wait_stats, Solution},
};

use super::{ScenarioSolver, TaskAction, VehicleTask};

pub struct NearestSolver;

type CustomerLocation = GeomWithData<[f64; 2], Uuid>;

impl ScenarioSolver for NearestSolver {
    fn solve(&self, scenario: &Scenario) -> Box<dyn Iterator<Item = VehicleTask>> {
        let start_time = std::time::Instant::now();

        let customer_ids: Vec<Uuid> = scenario.customers.values().map(|c| c.id).collect();

        let vehicle_ids: Vec<Uuid> = scenario.vehicles.values().map(|v| v.id).collect();

        let mut virtual_vehicle_positions = scenario
            .vehicles
            .values()
            .map(|v| (v.id, v.pos))
            .collect::<HashMap<_, _>>();

        let mut assignments = Vec::new();
        let mut tree: RTree<CustomerLocation> = RTree::new();

        for (customer_id, customer) in &scenario.customers {
            tree.insert(CustomerLocation::new(
                [customer.pos.x, customer.pos.y],
                *customer_id,
            ));
        }

        for _ in 0..scenario.customers.len().div_ceil(scenario.vehicles.len()) {
            for vehicle_id in &vehicle_ids {
                let vehicle_pos = virtual_vehicle_positions.get(vehicle_id).unwrap();
                if let Some(nearest_customer) =
                    tree.pop_nearest_neighbor(&[vehicle_pos.x, vehicle_pos.y])
                {
                    assignments.push(VehicleTask {
                        vehicle_id: *vehicle_id,
                        action: TaskAction::Assignment(nearest_customer.data),
                    });

                    let c = scenario.customers.get(&nearest_customer.data).unwrap();

                    virtual_vehicle_positions.insert(*vehicle_id, c.dest);
                } else {
                    break;
                }
            }
        }

        let elapsed = start_time.elapsed();

        log::info!(
            "Nearest R*-Tree solver took {} s for {} Customers and {} Vehicles",
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

        log::info!("Nearest solver solution wait times stats:");
        log::info!("Min:\t\t{}", min);
        log::info!("Max:\t\t{}", max);
        log::info!("Avg:\t\t{}", avg);
        log::info!("Median:\t{}", median);

        Box::new(assignments.into_iter())
    }
}
