use std::str::FromStr;

use reqwest::{Client, Url};
use scenario_runner::models::{ScenarioDto, StandardMagentaVehicleDto};
use uuid::Uuid;

use crate::{CustomerID, ScenarioID, VehicleID};

#[derive(Clone, Debug)]
pub struct ApiWrapper {
    scenario_manager_url: Url,
    scenario_runner_url: Url,
    client: reqwest::Client,
}

impl ApiWrapper {
    pub fn new(scenario_manager_url: Url, scenario_runner_url: Url, client: Client) -> Self {
        ApiWrapper {
            scenario_manager_url,
            scenario_runner_url,
            client,
        }
    }

    pub async fn get_scenarios(&self) -> Result<Vec<ScenarioDto>, anyhow::Error> {
        // curl -X 'GET' \
        //   'http://localhost:8080/scenarios' \
        //   -H 'accept: application/json'

        log::trace!("Getting scenarios from scenario manager");

        let scenarios_url = self.scenario_manager_url.with_path("/scenarios");
        let res = self
            .client
            .get(scenarios_url)
            .header("accept", "application/json")
            .send()
            .await?;

        let body = res.text().await?;

        log::trace!("Got scenarios response: {}", body);

        Ok(serde_json::from_str(&body)?)
    }

    async fn get_scenario_str(&self, id: Uuid) -> Result<String, anyhow::Error> {
        log::trace!("Getting scenario {} from scenario manager", id);
        let scenario_url = self
            .scenario_manager_url
            .with_path(&format!("/scenarios/{}", id));
        let res = self
            .client
            .get(scenario_url)
            .header("accept", "application/json")
            .send()
            .await?;
        let body = res.text().await?;
        log::trace!("Got scenario {} response: {}", id, body);
        Ok(body)
    }

    pub async fn get_scenario(&self, id: Uuid) -> Result<ScenarioDto, anyhow::Error> {
        // curl -X 'GET' \
        //   'http://localhost:8080/scenarios/748e736f-0ff1-4868-9df3-795b130eb4a7' \
        //   -H 'accept: application/json'

        let body = self.get_scenario_str(id).await?;

        Ok(serde_json::from_str(&body)?)
    }

    pub async fn get_vehicle(&self, id: Uuid) -> Result<StandardMagentaVehicleDto, anyhow::Error> {
        // /vehicles/{vehicleId}

        log::trace!("Getting vehicle: {}", id);

        let vehicle_url = self
            .scenario_manager_url
            .with_path(&format!("/vehicles/{}", id));

        let res = self
            .client
            .get(vehicle_url)
            .header("accept", "application/json")
            .send()
            .await?;

        let body = res.text().await?;

        log::trace!("Got vehicle {} response: {}", id, body);

        Ok(serde_json::from_str(&body)?)
    }

    pub async fn assign_vehicle(
        &self,
        scenario_id: Uuid,
        vehicle_id: VehicleID,
        customer_id: CustomerID,
    ) -> anyhow::Result<()> {
        // PUT /Scenarios/update_scenario/{scenario_id}

        log::trace!(
            "Assigning vehicle {} to customer {} in scenario {}",
            vehicle_id,
            customer_id,
            scenario_id
        );

        let update_url = self
            .scenario_manager_url
            .with_path(&format!("/Scenarios/update_scenario/{}", scenario_id));

        let update_str = format!(
            "{{\"vehicles\":[{{\"id\":\"{}\",\"customerId\":\"{}\"}}]}}",
            vehicle_id, customer_id
        );

        let res = self
            .client
            .put(update_url)
            .header("accept", "application/json")
            .header("Content-Type", "application/json")
            .body(update_str)
            .send()
            .await?;

        log::trace!("Assign vehicle response: {:?}", res);

        Ok(())
    }

    pub async fn initialize_scenario(&self, id: ScenarioID) -> anyhow::Result<()> {
        // POST /Scenarios/initialize_scenario
        log::trace!("Initializing scenario {}", id);

        let init_url = self
            .scenario_manager_url
            .with_path("/Scenarios/initialize_scenario");

        let init_str = self.get_scenario_str(id).await?;

        let res = self
            .client
            .post(init_url)
            .header("accept", "application/json")
            .header("Content-Type", "application/json")
            .body(init_str)
            .send()
            .await?;

        log::trace!("Initialize scenario response: {:?}", res);

        if res.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to initialize scenario"))
        }
    }

    pub async fn launch_scenario(&self, id: ScenarioID, speed: Option<f64>) -> anyhow::Result<()> {
        // POST /Runner/launch_scenario/{scenario_id}
        let speed = speed.unwrap_or(0.2);
        log::trace!("Launching scenario {} with speed {}", id, speed);

        let launch_url = self
            .scenario_runner_url
            .with_path(&format!("/launch_scenario/{}", id));

        let res = self
            .client
            .post(launch_url)
            .query(&[("speed", speed)])
            .send()
            .await?;

        log::trace!("Launch scenario response: {:?}", res);

        if res.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to launch scenario"))
        }
    }
}

impl Default for ApiWrapper {
    fn default() -> Self {
        Self {
            scenario_manager_url: Url::from_str("http://localhost:8080").unwrap(),
            scenario_runner_url: Url::from_str("http://localhost:8090/").unwrap(),
            client: reqwest::Client::new(),
        }
    }
}

// Simple trait to make setting Paths for Urls without modifying the original easier
// simply clones and modifies that
trait WithPath {
    fn with_path(&self, path: &str) -> Url;
}

impl WithPath for Url {
    fn with_path(&self, path: &str) -> Url {
        let mut url = self.clone();
        url.set_path(path);
        url
    }
}
