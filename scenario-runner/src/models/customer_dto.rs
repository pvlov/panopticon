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

/// CustomerDto : The customer data transfer object
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomerDto {
    #[serde(rename = "awaitingService")]
    pub awaiting_service: bool,
    #[serde(rename = "coordX")]
    pub coord_x: f32,
    #[serde(rename = "coordY")]
    pub coord_y: f32,
    #[serde(rename = "destinationX")]
    pub destination_x: f32,
    #[serde(rename = "destinationY")]
    pub destination_y: f32,
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
}

impl CustomerDto {
    /// The customer data transfer object
    pub fn new(awaiting_service: bool, coord_x: f32, coord_y: f32, destination_x: f32, destination_y: f32, id: uuid::Uuid) -> CustomerDto {
        CustomerDto {
            awaiting_service,
            coord_x,
            coord_y,
            destination_x,
            destination_y,
            id,
        }
    }
}

