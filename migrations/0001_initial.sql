-- Enable PostGIS
CREATE EXTENSION IF NOT EXISTS postgis;

-- Agencies
CREATE TABLE IF NOT EXISTS agencies (
    id          UUID PRIMARY KEY,
    agency_id   TEXT NOT NULL UNIQUE,
    name        TEXT NOT NULL,
    url         TEXT NOT NULL,
    timezone    TEXT NOT NULL,
    lang        TEXT,
    phone       TEXT
);

-- Stops
CREATE TABLE IF NOT EXISTS stops (
    id              UUID PRIMARY KEY,
    stop_id         TEXT NOT NULL UNIQUE,
    name            TEXT NOT NULL,
    lat             DOUBLE PRECISION NOT NULL,
    lon             DOUBLE PRECISION NOT NULL,
    code            TEXT,
    desc            TEXT,
    zone_id         TEXT,
    url             TEXT,
    location_type   INTEGER,
    parent_station  TEXT
);

-- Spatial index for nearest-stop queries
CREATE INDEX IF NOT EXISTS stops_geo_idx
    ON stops USING GIST (ST_MakePoint(lon, lat)::geography);

-- Routes (bus lines)
CREATE TABLE IF NOT EXISTS routes (
    id          UUID PRIMARY KEY,
    route_id    TEXT NOT NULL UNIQUE,
    agency_id   TEXT NOT NULL REFERENCES agencies(agency_id) ON DELETE CASCADE,
    short_name  TEXT NOT NULL,
    long_name   TEXT NOT NULL,
    route_type  INTEGER NOT NULL,
    color       TEXT,
    text_color  TEXT,
    desc        TEXT,
    url         TEXT
);

-- Trips
CREATE TABLE IF NOT EXISTS trips (
    id           UUID PRIMARY KEY,
    trip_id      TEXT NOT NULL UNIQUE,
    route_id     TEXT NOT NULL REFERENCES routes(route_id) ON DELETE CASCADE,
    service_id   TEXT NOT NULL,
    headsign     TEXT,
    short_name   TEXT,
    direction_id INTEGER,
    block_id     TEXT,
    shape_id     TEXT
);

CREATE INDEX IF NOT EXISTS trips_route_idx ON trips(route_id);

-- Stop times
CREATE TABLE IF NOT EXISTS stop_times (
    id                  UUID PRIMARY KEY,
    trip_id             TEXT NOT NULL REFERENCES trips(trip_id) ON DELETE CASCADE,
    stop_id             TEXT NOT NULL REFERENCES stops(stop_id) ON DELETE CASCADE,
    stop_sequence       INTEGER NOT NULL,
    arrival_time        TIME,
    departure_time      TIME,
    stop_headsign       TEXT,
    pickup_type         INTEGER,
    drop_off_type       INTEGER,
    shape_dist_traveled DOUBLE PRECISION,
    UNIQUE (trip_id, stop_sequence)
);

CREATE INDEX IF NOT EXISTS stop_times_stop_idx   ON stop_times(stop_id);
CREATE INDEX IF NOT EXISTS stop_times_trip_idx   ON stop_times(trip_id);
CREATE INDEX IF NOT EXISTS stop_times_depart_idx ON stop_times(departure_time);
