mod routes;
mod state;
mod ws;

use anyhow::Result;
use axum::{routing::{get, post}, Router};
use db::models::VehiclePosition;
use dotenvy::dotenv;
use leptos::{config::get_configuration, view};
use leptos_axum::{generate_route_list, handle_server_fns, render_route};
use state::AppState;
use tokio::sync::broadcast;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use app::App;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,server=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".into());

    // DB
    let pool = db::create_pool(&database_url).await?;
    db::run_migrations(&pool).await?;
    info!("Database connected and migrations run");

    // Redis
    let redis_client = redis::Client::open(redis_url)?;
    info!("Redis connected");

    // Broadcast channel for live positions
    let (positions_tx, _) = broadcast::channel::<Vec<VehiclePosition>>(32);

    // Start GTFS-RT background poller
    realtime::start_poller(redis_client.clone(), positions_tx.clone()).await;
    info!("GTFS-RT poller started");

    let conf = get_configuration(Some("Cargo.toml"))?;
    let leptos_options = conf.leptos_options;

    let state = AppState {
        db: pool,
        redis: redis_client,
        leptos_options,
        positions_tx,
    };

    let routes = generate_route_list(|| view! { <App/> });

    let app = Router::new()
        .route("/hello", get(|| async { "ok" }))
        .route("/api/*fn_name", post(handle_server_fns))
        .route("/", get(render_route::<AppState, _>(routes.clone(), || view! { <App/> })))
        .route("/*path", get(render_route::<AppState, _>(routes, || view! { <App/> })))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = "0.0.0.0:3000";
    info!("Listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
