use axum::extract::FromRef;
use db::models::VehiclePosition;
use leptos::config::LeptosOptions;
use sqlx::PgPool;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub redis: redis::Client,
    pub leptos_options: LeptosOptions,
    /// Broadcast channel for live vehicle positions
    pub positions_tx: broadcast::Sender<Vec<VehiclePosition>>,
}

impl FromRef<AppState> for LeptosOptions {
    fn from_ref(state: &AppState) -> Self {
        state.leptos_options.clone()
    }
}
