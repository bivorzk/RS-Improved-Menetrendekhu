use std::time::Duration;

use anyhow::Result;
use db::models::VehiclePosition;
use gtfs::realtime::{bkk, fetch_vehicle_positions};
use redis::AsyncCommands;
use tokio::sync::broadcast;
use tracing::{error, info};

pub const REDIS_POSITIONS_KEY: &str = "bus:positions:live";
pub const POLL_INTERVAL_SECS: u64 = 15;

/// Start the background GTFS-RT polling loop.
/// Fetches vehicle positions every N seconds, stores in Redis,
/// and broadcasts updates to all connected WebSocket clients.
pub async fn start_poller(
    redis_client: redis::Client,
    tx: broadcast::Sender<Vec<VehiclePosition>>,
) {
    tokio::spawn(async move {
        loop {
            match poll_and_broadcast(&redis_client, &tx).await {
                Ok(count) => info!("Broadcasted {} vehicle positions", count),
                Err(e) => error!("GTFS-RT poll error: {}", e),
            }
            tokio::time::sleep(Duration::from_secs(POLL_INTERVAL_SECS)).await;
        }
    });
}

async fn poll_and_broadcast(
    redis_client: &redis::Client,
    tx: &broadcast::Sender<Vec<VehiclePosition>>,
) -> Result<usize> {
    // Fetch from BKK — extend with Volánbusz when their RT feed becomes available
    let positions = fetch_vehicle_positions(bkk::VEHICLE_POSITIONS).await?;

    // Cache in Redis as JSON (TTL slightly longer than poll interval)
    let mut conn = redis_client.get_multiplexed_async_connection().await?;
    let json = serde_json::to_string(&positions)?;
    conn.set_ex(REDIS_POSITIONS_KEY, json, POLL_INTERVAL_SECS * 2).await?;

    let count = positions.len();

    // Broadcast to WebSocket subscribers — ignore if no receivers
    let _ = tx.send(positions);

    Ok(count)
}

/// Get the last cached vehicle positions from Redis.
/// Used for initial data on new WebSocket connections.
pub async fn get_cached_positions(redis_client: &redis::Client) -> Result<Vec<VehiclePosition>> {
    let mut conn = redis_client.get_multiplexed_async_connection().await?;
    let json: Option<String> = conn.get(REDIS_POSITIONS_KEY).await?;

    match json {
        Some(j) => Ok(serde_json::from_str(&j)?),
        None => Ok(vec![]),
    }
}
