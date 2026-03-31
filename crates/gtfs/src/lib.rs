pub mod ingest;
pub mod realtime;

// Include prost-generated GTFS-RT types
pub mod gtfs_rt {
    include!(concat!(env!("OUT_DIR"), "/transit_realtime.rs"));
}
