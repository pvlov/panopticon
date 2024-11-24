use std::{collections::HashMap, time::Duration};

use anyhow::anyhow;
use scenario_runner::models::{CustomerDto, StandardMagentaVehicleDto};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize)]
pub enum VehicleStatus<'a> {
    DrivingToDestination(&'a Customer),
    DrivingToCustomer(&'a Customer),
    Idle,
}

#[derive(Debug, Clone, Serialize)]
pub struct Vehicle<'a> {
    pub id: Uuid,
    pub pos: Position,
    pub speed: f64,
    pub status: VehicleStatus<'a>,
    pub distance_travelled: f64,
    pub num_trips: u32,
    pub remaining_travel_time: Option<Duration>,
}

impl<'a> TryFrom<StandardMagentaVehicleDto> for Vehicle<'a> {
    type Error = anyhow::Error;

    fn try_from(val: StandardMagentaVehicleDto) -> Result<Self, Self::Error> {
        let pos = Position {
            x: val.coord_x,
            y: val.coord_y,
        };

        // TODO check (more) that this is a "start" vehicle dto
        if let Some(speed) = val.vehicle_speed {
            if speed > 0.0 {
                return Err(anyhow!(
                    "Not an initial vehicle dto (speed is non-null and >0)"
                ));
            }
        }

        Ok(Self {
            id: val.id,
            pos,
            speed: val.vehicle_speed.unwrap_or_default(),
            status: VehicleStatus::Idle,
            distance_travelled: 0.0,
            num_trips: 0,
            remaining_travel_time: None,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum CustomerStatus {
    /// Customer waiting, no taxi is making its way to this customer.
    Unassigned,
    /// Customer waiting for a taxi that is making its way to them.
    Waiting,
    /// Customer is currently in a taxi and being driven to their destination.
    Riding(Uuid),
    /// Customer has arrived at their destination and is no longer occupying a taxi.
    Served,
}

#[derive(Debug, Clone, Serialize)]
pub struct Customer {
    pub id: Uuid,
    pub pos: Position,
    pub dest: Position,
    pub status: CustomerStatus,
}

impl TryFrom<CustomerDto> for Customer {
    type Error = anyhow::Error;

    fn try_from(val: CustomerDto) -> Result<Self, Self::Error> {
        if !val.awaiting_service {
            return Err(anyhow!(
                "Not an initial customer dto (awaiting_service is false)"
            ));
        }

        let pos = Position {
            x: val.coord_x,
            y: val.coord_y,
        };

        let dest = Position {
            x: val.destination_x,
            y: val.destination_y,
        };

        Ok(Self {
            id: val.id,
            pos,
            dest,
            status: CustomerStatus::Unassigned,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Scenario<'a> {
    pub id: Uuid,
    pub vehicles: HashMap<Uuid, Vehicle<'a>>,
    pub customers: HashMap<Uuid, Customer>,
    pub status: String,
}
