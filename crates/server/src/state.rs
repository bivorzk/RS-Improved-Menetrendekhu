use db::models::VehiclePosition;
use sqlx::PgPool;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: redis::Client,
    /// Broadcast channel for live vehicle positions
    pub positions_tx: broadcast::Sender<Vec<VehiclePosition>>,
}
