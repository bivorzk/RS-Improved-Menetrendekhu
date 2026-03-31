use anyhow::Result;
use prost::Message;
use tracing::{info, warn};

use crate::gtfs_rt::FeedMessage;
use db::models::VehiclePosition;

/// BKK (Budapest) GTFS-RT endpoints
pub mod bkk {
    pub const VEHICLE_POSITIONS: &str =
        "https://go.bkk.hu/api/query/v1/ws/gtfs-rt/full/VehiclePositions.pb";
    pub const TRIP_UPDATES: &str =
        "https://go.bkk.hu/api/query/v1/ws/gtfs-rt/full/TripUpdates.pb";
    pub const ALERTS: &str =
        "https://go.bkk.hu/api/query/v1/ws/gtfs-rt/full/Alerts.pb";
}

/// Fetch and decode a GTFS-RT vehicle positions feed.
/// Returns a Vec of VehiclePosition ready to be pushed into Redis.
pub async fn fetch_vehicle_positions(feed_url: &str) -> Result<Vec<VehiclePosition>> {
    info!("Fetching GTFS-RT vehicle positions from {}", feed_url);

    let bytes = reqwest::get(feed_url)
        .await?
        .bytes()
        .await?;

    let feed = FeedMessage::decode(bytes).map_err(|e| {
        anyhow::anyhow!("Failed to decode GTFS-RT protobuf: {}", e)
    })?;

    let mut positions = Vec::new();

    for entity in feed.entity {
        if let Some(vp) = entity.vehicle {
            let pos = match vp.position {
                Some(p) => p,
                None => {
                    warn!("Vehicle entity {} has no position, skipping", entity.id);
                    continue;
                }
            };

            let vehicle_id = vp
                .vehicle
                .as_ref()
                .and_then(|v| v.id.clone())
                .unwrap_or(entity.id);

            let trip_id = vp.trip.as_ref().and_then(|t| t.trip_id.clone());
            let route_id = vp.trip.as_ref().and_then(|t| t.route_id.clone());

            let current_status = vp.current_status.map(|s| match s {
                0 => "INCOMING_AT",
                1 => "STOPPED_AT",
                2 => "IN_TRANSIT_TO",
                _ => "UNKNOWN",
            }.to_string());

            positions.push(VehiclePosition {
                vehicle_id,
                trip_id,
                route_id,
                lat: pos.latitude,
                lon: pos.longitude,
                bearing: pos.bearing,
                speed: pos.speed,
                timestamp: vp.timestamp,
                current_stop_sequence: vp.current_stop_sequence,
                current_status,
            });
        }
    }

    info!("Decoded {} vehicle positions", positions.len());
    Ok(positions)
}
