use std::{collections::HashMap, future::Future, process::Output, sync::Arc};

use futures::stream::{FuturesOrdered, FuturesUnordered};
use tokio::task;
use uuid::Uuid;

use crate::{
    api_wrapper::ApiWrapper,
    scenario::Scenario,
    solvers::{random::RandomSolver, ScenarioSolver, VehicleTask},
};

enum SolverChoice {
    RandomSolver,
}

type VehicleID = Uuid;

struct ScenarioManager<'a> {
    scenario: &'a Scenario<'a>,
    solver_choice: SolverChoice,
    api: Arc<ApiWrapper>,
}

impl<'a> ScenarioManager<'a> {
    async fn start(&self) {
        let solver: Box<dyn ScenarioSolver> = match self.solver_choice {
            SolverChoice::RandomSolver => Box::new(RandomSolver),
        };

        let mut last_tasks = HashMap::<VehicleID, task::JoinHandle<()>>::new();

        let instructions = solver.solve(self.scenario);

        for instruction in instructions {
            let vehicle_id = instruction.vehicle_id;
            let last_future = last_tasks.remove(&vehicle_id);
            let api = self.api.clone();

            // TODO fallibility?
            // nah, I'd win
            let task = task::spawn(async move {
                // await the vehicles last Future so that we don't send
                // anything until the previous Future has completed
                if let Some(handle) = last_future {
                    handle.await;
                }

                // check vehicle status

                // wait until it is done / update status

                // send the instruction

                ()
            });
        }
    }
}
