use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// A transit agency (e.g. Volánbusz, BKK)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Agency {
    pub id: Uuid,
    pub agency_id: String,
    pub name: String,
    pub url: String,
    pub timezone: String,
    pub lang: Option<String>,
    pub phone: Option<String>,
}

/// A physical bus stop / station
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Stop {
    pub id: Uuid,
    pub stop_id: String,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
    pub code: Option<String>,
    pub desc: Option<String>,
    pub zone_id: Option<String>,
    pub url: Option<String>,
    pub location_type: Option<i32>,
    pub parent_station: Option<String>,
}

/// A transit route (bus line)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Route {
    pub id: Uuid,
    pub route_id: String,
    pub agency_id: String,
    pub short_name: String,
    pub long_name: String,
    pub route_type: i32, // 3 = bus
    pub color: Option<String>,
    pub text_color: Option<String>,
    pub desc: Option<String>,
    pub url: Option<String>,
}

/// A specific trip on a route
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Trip {
    pub id: Uuid,
    pub trip_id: String,
    pub route_id: String,
    pub service_id: String,
    pub headsign: Option<String>,
    pub short_name: Option<String>,
    pub direction_id: Option<i32>,
    pub block_id: Option<String>,
    pub shape_id: Option<String>,
}

/// A scheduled stop time for a trip
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StopTime {
    pub id: Uuid,
    pub trip_id: String,
    pub stop_id: String,
    pub stop_sequence: i32,
    pub arrival_time: Option<NaiveTime>,
    pub departure_time: Option<NaiveTime>,
    pub stop_headsign: Option<String>,
    pub pickup_type: Option<i32>,
    pub drop_off_type: Option<i32>,
    pub shape_dist_traveled: Option<f64>,
}

/// Live vehicle position from GTFS-RT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehiclePosition {
    pub vehicle_id: String,
    pub trip_id: Option<String>,
    pub route_id: Option<String>,
    pub lat: f32,
    pub lon: f32,
    pub bearing: Option<f32>,
    pub speed: Option<f32>,
    pub timestamp: Option<u64>,
    pub current_stop_sequence: Option<u32>,
    pub current_status: Option<String>,
}
