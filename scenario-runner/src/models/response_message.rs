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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseMessage {
    #[serde(rename = "message")]
    pub message: String,
}

impl ResponseMessage {
    pub fn new(message: String) -> ResponseMessage {
        ResponseMessage {
            message,
        }
    }
}

