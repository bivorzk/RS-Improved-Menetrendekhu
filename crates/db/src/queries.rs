use anyhow::Result;
use sqlx::PgPool;

use crate::models::{Route, Stop, StopTime};

// --- Stops ---

pub async fn get_stops_near(
    pool: &PgPool,
    lat: f64,
    lon: f64,
    radius_meters: f64,
) -> Result<Vec<Stop>> {
    // Uses PostGIS ST_DWithin for efficient spatial query
    let stops = sqlx::query_as!(
        Stop,
        r#"
        SELECT id, stop_id, name, lat, lon, code, "desc" AS desc, zone_id, url, location_type, parent_station
        FROM stops
        WHERE ST_DWithin(
            ST_MakePoint(lon, lat)::geography,
            ST_MakePoint($1, $2)::geography,
            $3
        )
        ORDER BY ST_Distance(
            ST_MakePoint(lon, lat)::geography,
            ST_MakePoint($1, $2)::geography
        )
        LIMIT 20
        "#,
        lon,
        lat,
        radius_meters
    )
    .fetch_all(pool)
    .await?;

    Ok(stops)
}

pub async fn get_stop_by_id(pool: &PgPool, stop_id: &str) -> Result<Option<Stop>> {
    let stop = sqlx::query_as!(
        Stop,
        r#"
        SELECT id, stop_id, name, lat, lon, code, "desc" AS desc, zone_id, url, location_type, parent_station
        FROM stops WHERE stop_id = $1
        "#,
        stop_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(stop)
}

pub async fn search_stops(pool: &PgPool, query: &str) -> Result<Vec<Stop>> {
    let stops = sqlx::query_as!(
        Stop,
        r#"
        SELECT id, stop_id, name, lat, lon, code, "desc" AS desc, zone_id, url, location_type, parent_station
        FROM stops
        WHERE name ILIKE $1
        ORDER BY name
        LIMIT 20
        "#,
        format!("%{}%", query)
    )
    .fetch_all(pool)
    .await?;

    Ok(stops)
}

// --- Routes ---

pub async fn get_routes_for_stop(pool: &PgPool, stop_id: &str) -> Result<Vec<Route>> {
    let routes = sqlx::query_as!(
        Route,
        r#"
        SELECT DISTINCT r.id, r.route_id, r.agency_id, r.short_name, r.long_name,
               r.route_type, r.color, r.text_color, r."desc" AS desc, r.url
        FROM routes r
        JOIN trips t ON t.route_id = r.route_id
        JOIN stop_times st ON st.trip_id = t.trip_id
        WHERE st.stop_id = $1
        ORDER BY r.short_name
        "#,
        stop_id
    )
    .fetch_all(pool)
    .await?;

    Ok(routes)
}

// --- Departures ---

pub async fn get_departures_from_stop(
    pool: &PgPool,
    stop_id: &str,
    limit: i64,
) -> Result<Vec<StopTime>> {
    let now = chrono::Local::now().time();

    let departures = sqlx::query_as!(
        StopTime,
        r#"
        SELECT st.id, st.trip_id, st.stop_id, st.stop_sequence,
               st.arrival_time, st.departure_time, st.stop_headsign,
               st.pickup_type, st.drop_off_type, st.shape_dist_traveled
        FROM stop_times st
        WHERE st.stop_id = $1
          AND st.departure_time >= $2
        ORDER BY st.departure_time
        LIMIT $3
        "#,
        stop_id,
        now,
        limit
    )
    .fetch_all(pool)
    .await?;

    Ok(departures)
}

// --- Trips ---

pub async fn get_trip_stops(pool: &PgPool, trip_id: &str) -> Result<Vec<(StopTime, Stop)>> {
    // TODO: implement join query
    // Returns ordered stop times with their stop details for a full trip view
    let _ = (pool, trip_id);
    todo!("implement get_trip_stops join query")
}
