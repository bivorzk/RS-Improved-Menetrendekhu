use std::io::{Cursor, Read};

use anyhow::{Context, Result};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::{info, warn};
use uuid::Uuid;

/// Known Hungarian GTFS feed URLs
pub mod feeds {
    pub const VOLANBUSZ: &str = "https://www.volanbusz.hu/hu/menetrendek/gtfs";
    pub const BKK: &str =
        "https://go.bkk.hu/api/static/v1/public-gtfs/budapest_gtfs.zip";
}

// Raw CSV row structs — map 1:1 to GTFS spec column names

#[derive(Debug, Deserialize)]
struct AgencyRow {
    agency_id: Option<String>,
    agency_name: String,
    agency_url: String,
    agency_timezone: String,
    agency_lang: Option<String>,
    agency_phone: Option<String>,
}

#[derive(Debug, Deserialize)]
struct StopRow {
    stop_id: String,
    stop_name: String,
    stop_lat: f64,
    stop_lon: f64,
    stop_code: Option<String>,
    stop_desc: Option<String>,
    zone_id: Option<String>,
    stop_url: Option<String>,
    location_type: Option<i32>,
    parent_station: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RouteRow {
    route_id: String,
    agency_id: Option<String>,
    route_short_name: String,
    route_long_name: String,
    route_type: i32,
    route_color: Option<String>,
    route_text_color: Option<String>,
    route_desc: Option<String>,
    route_url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TripRow {
    trip_id: String,
    route_id: String,
    service_id: String,
    trip_headsign: Option<String>,
    trip_short_name: Option<String>,
    direction_id: Option<i32>,
    block_id: Option<String>,
    shape_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct StopTimeRow {
    trip_id: String,
    stop_id: String,
    stop_sequence: i32,
    arrival_time: Option<String>,
    departure_time: Option<String>,
    stop_headsign: Option<String>,
    pickup_type: Option<i32>,
    drop_off_type: Option<i32>,
    shape_dist_traveled: Option<f64>,
}

/// Download a GTFS feed and ingest it into the database.
/// Call this on startup and then periodically (e.g. daily).
pub async fn ingest_feed(pool: &PgPool, feed_url: &str) -> Result<()> {
    info!("Downloading GTFS feed from {}", feed_url);

    let bytes = reqwest::get(feed_url)
        .await
        .context("Failed to download GTFS feed")?
        .bytes()
        .await
        .context("Failed to read GTFS response bytes")?;

    info!("Downloaded {} bytes, parsing ZIP...", bytes.len());

    let cursor = Cursor::new(bytes);
    let mut zip = zip::ZipArchive::new(cursor).context("Failed to open GTFS ZIP")?;

    // Process each file in the ZIP
    let file_names: Vec<String> = (0..zip.len())
        .filter_map(|i| zip.by_index(i).ok().map(|f| f.name().to_string()))
        .collect();

    info!("GTFS files in archive: {:?}", file_names);

    // Helper: extract a file from the zip into a String
    let read_zip_file = |zip: &mut zip::ZipArchive<Cursor<bytes::Bytes>>, name: &str| -> Result<String> {
        let mut file = zip.by_name(name)
            .with_context(|| format!("Missing {} in GTFS zip", name))?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    };

    // --- agency.txt ---
    if let Ok(content) = read_zip_file(&mut zip, "agency.txt") {
        let mut rdr = csv::Reader::from_reader(content.as_bytes());
        for result in rdr.deserialize::<AgencyRow>() {
            match result {
                Ok(row) => {
                    let agency_id = row.agency_id.unwrap_or_else(|| "default".to_string());
                    sqlx::query!(
                        r#"
                        INSERT INTO agencies (id, agency_id, name, url, timezone, lang, phone)
                        VALUES ($1, $2, $3, $4, $5, $6, $7)
                        ON CONFLICT (agency_id) DO UPDATE
                          SET name = EXCLUDED.name, url = EXCLUDED.url
                        "#,
                        Uuid::new_v4(),
                        agency_id,
                        row.agency_name,
                        row.agency_url,
                        row.agency_timezone,
                        row.agency_lang,
                        row.agency_phone,
                    )
                    .execute(pool)
                    .await?;
                }
                Err(e) => warn!("Skipping agency row: {}", e),
            }
        }
        info!("Ingested agency.txt");
    }

    // --- stops.txt ---
    if let Ok(content) = read_zip_file(&mut zip, "stops.txt") {
        let mut rdr = csv::Reader::from_reader(content.as_bytes());
        for result in rdr.deserialize::<StopRow>() {
            match result {
                Ok(row) => {
                    sqlx::query!(
                        r#"
                        INSERT INTO stops (id, stop_id, name, lat, lon, code, "desc", zone_id, url, location_type, parent_station)
                        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                        ON CONFLICT (stop_id) DO UPDATE
                          SET name = EXCLUDED.name, lat = EXCLUDED.lat, lon = EXCLUDED.lon
                        "#,
                        Uuid::new_v4(),
                        row.stop_id,
                        row.stop_name,
                        row.stop_lat,
                        row.stop_lon,
                        row.stop_code,
                        row.stop_desc,
                        row.zone_id,
                        row.stop_url,
                        row.location_type,
                        row.parent_station,
                    )
                    .execute(pool)
                    .await?;
                }
                Err(e) => warn!("Skipping stop row: {}", e),
            }
        }
        info!("Ingested stops.txt");
    }

    // --- routes.txt ---
    if let Ok(content) = read_zip_file(&mut zip, "routes.txt") {
        let mut rdr = csv::Reader::from_reader(content.as_bytes());
        for result in rdr.deserialize::<RouteRow>() {
            match result {
                Ok(row) => {
                    sqlx::query!(
                        r#"
                        INSERT INTO routes (id, route_id, agency_id, short_name, long_name, route_type, color, text_color, "desc", url)
                        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                        ON CONFLICT (route_id) DO UPDATE
                          SET short_name = EXCLUDED.short_name, long_name = EXCLUDED.long_name
                        "#,
                        Uuid::new_v4(),
                        row.route_id,
                        row.agency_id.unwrap_or_default(),
                        row.route_short_name,
                        row.route_long_name,
                        row.route_type,
                        row.route_color,
                        row.route_text_color,
                        row.route_desc,
                        row.route_url,
                    )
                    .execute(pool)
                    .await?;
                }
                Err(e) => warn!("Skipping route row: {}", e),
            }
        }
        info!("Ingested routes.txt");
    }

    // --- trips.txt ---
    if let Ok(content) = read_zip_file(&mut zip, "trips.txt") {
        let mut rdr = csv::Reader::from_reader(content.as_bytes());
        for result in rdr.deserialize::<TripRow>() {
            match result {
                Ok(row) => {
                    sqlx::query!(
                        r#"
                        INSERT INTO trips (id, trip_id, route_id, service_id, headsign, short_name, direction_id, block_id, shape_id)
                        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                        ON CONFLICT (trip_id) DO NOTHING
                        "#,
                        Uuid::new_v4(),
                        row.trip_id,
                        row.route_id,
                        row.service_id,
                        row.trip_headsign,
                        row.trip_short_name,
                        row.direction_id,
                        row.block_id,
                        row.shape_id,
                    )
                    .execute(pool)
                    .await?;
                }
                Err(e) => warn!("Skipping trip row: {}", e),
            }
        }
        info!("Ingested trips.txt");
    }

    // --- stop_times.txt (largest file — stream row by row) ---
    if let Ok(content) = read_zip_file(&mut zip, "stop_times.txt") {
        let mut rdr = csv::Reader::from_reader(content.as_bytes());
        let mut count = 0u64;
        for result in rdr.deserialize::<StopTimeRow>() {
            match result {
                Ok(row) => {
                    // GTFS times can exceed 24:00:00 for overnight trips — handle gracefully
                    let arrival = parse_gtfs_time(row.arrival_time.as_deref());
                    let departure = parse_gtfs_time(row.departure_time.as_deref());

                    sqlx::query!(
                        r#"
                        INSERT INTO stop_times (id, trip_id, stop_id, stop_sequence, arrival_time, departure_time,
                                                stop_headsign, pickup_type, drop_off_type, shape_dist_traveled)
                        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                        ON CONFLICT DO NOTHING
                        "#,
                        Uuid::new_v4(),
                        row.trip_id,
                        row.stop_id,
                        row.stop_sequence,
                        arrival,
                        departure,
                        row.stop_headsign,
                        row.pickup_type,
                        row.drop_off_type,
                        row.shape_dist_traveled,
                    )
                    .execute(pool)
                    .await?;

                    count += 1;
                    if count % 100_000 == 0 {
                        info!("Ingested {} stop_times...", count);
                    }
                }
                Err(e) => warn!("Skipping stop_time row: {}", e),
            }
        }
        info!("Ingested stop_times.txt ({} rows)", count);
    }

    info!("GTFS ingest complete for {}", feed_url);
    Ok(())
}

/// Parse GTFS time string (HH:MM:SS) — handles >24h times for overnight trips
/// by clamping to 23:59:59
fn parse_gtfs_time(s: Option<&str>) -> Option<chrono::NaiveTime> {
    let s = s?;
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 3 {
        return None;
    }
    let h: u32 = parts[0].parse().ok()?;
    let m: u32 = parts[1].parse().ok()?;
    let sec: u32 = parts[2].parse().ok()?;

    // Clamp overnight trips (25:00:00 etc.) to end of day
    let h = h.min(23);
    chrono::NaiveTime::from_hms_opt(h, m, sec)
}
