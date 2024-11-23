use std::{collections::HashMap, time::Duration};

use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct Position {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone, Serialize)]
pub enum TaxiStatus<'a> {
    DrivingToDestination(&'a Customer),
    DrivingToCustomer(&'a Customer),
    Idle,
}

#[derive(Debug, Clone, Serialize)]
pub struct Vehicle<'a> {
    pub id: Uuid,
    pub pos: Position,
    pub speed: f64,
    pub status: TaxiStatus<'a>,
    pub distance_travelled: f64,
    pub num_trips: u32,
    pub remaining_travel_time: Option<Duration>,
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

#[derive(Debug, Clone, Serialize)]
pub struct Scenario<'a> {
    pub id: Uuid,
    pub vehicles: HashMap<Uuid, Vehicle<'a>>,
    pub customers: HashMap<Uuid, Customer>,
    pub status: String,
}
