# bus-tracker

Hungarian bus schedule & live tracking — Rust full stack.

**Stack:** Axum · Leptos · PostgreSQL + PostGIS · Redis · GTFS + GTFS-RT

## Prerequisites

- Rust (nightly) — `rustup toolchain install nightly`
- PostgreSQL with PostGIS extension
- Redis
- `cargo-leptos` — `cargo install cargo-leptos`
- `sqlx-cli` — `cargo install sqlx-cli --no-default-features --features postgres`

## Setup

```bash
cp .env.example .env
# edit .env with your DB credentials

# Create DB and run migrations
sqlx database create
sqlx migrate run

# Run dev server
cargo leptos watch
```

## Data sources

| Feed | URL |
|---|---|
| Volánbusz GTFS (static) | https://www.volanbusz.hu/hu/menetrendek/gtfs |
| BKK GTFS (static) | https://go.bkk.hu/api/static/v1/public-gtfs/budapest_gtfs.zip |
| BKK Vehicle Positions (RT) | https://go.bkk.hu/api/query/v1/ws/gtfs-rt/full/VehiclePositions.pb |
| BKK Trip Updates (RT) | https://go.bkk.hu/api/query/v1/ws/gtfs-rt/full/TripUpdates.pb |

## Workspace crates

- `crates/server` — Axum server, SSR, WebSocket
- `crates/app` — Leptos UI components and pages
- `crates/gtfs` — GTFS ZIP ingest + GTFS-RT protobuf decoder
- `crates/realtime` — background poller, Redis cache, broadcast channel
- `crates/db` — sqlx queries, models, migrations
