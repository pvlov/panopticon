/*
 * OpenAPI definition
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// VehicleDataDto : The vehicle data transfer object
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VehicleDataDto {
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "totalTravelTime")]
    pub total_travel_time: f32,
    #[serde(rename = "totalTrips")]
    pub total_trips: u32,
    #[serde(rename = "travelTimes")]
    pub travel_times: String,
}

impl VehicleDataDto {
    /// The vehicle data transfer object
    pub fn new(
        id: uuid::Uuid,
        total_travel_time: f32,
        total_trips: u32,
        travel_times: String,
    ) -> VehicleDataDto {
        VehicleDataDto {
            id,
            total_travel_time,
            total_trips,
            travel_times,
        }
    }
}
