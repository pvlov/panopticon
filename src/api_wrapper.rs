use std::str::FromStr;

use reqwest::{Client, Url};
use scenario_runner::models::{ScenarioDto, VehicleDataDto};
use uuid::Uuid;

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

    pub async fn get_scenario(&self, id: Uuid) -> Result<ScenarioDto, anyhow::Error> {
        // curl -X 'GET' \
        //   'http://localhost:8080/scenarios/748e736f-0ff1-4868-9df3-795b130eb4a7' \
        //   -H 'accept: application/json'

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

        log::trace!("Got scenario response: {}", body);

        Ok(serde_json::from_str(&body)?)
    }

    pub async fn get_vehicle(&self, id: Uuid) -> Result<VehicleDataDto, anyhow::Error> {
        // /vehicles/{vehicleId}

        // log::trace!("")
        todo!()
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

pub trait WithPath {
    fn with_path(&self, path: &str) -> Url;
}

impl WithPath for Url {
    fn with_path(&self, path: &str) -> Url {
        let mut url = self.clone();
        url.set_path(path);
        url
    }
}
