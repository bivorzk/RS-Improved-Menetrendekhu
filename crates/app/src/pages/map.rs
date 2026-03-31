use leptos::prelude::*;

/// Live map page. The actual Leaflet map is initialized via JS interop
/// in public/map.js — this component just provides the mount point.
#[component]
pub fn MapPage() -> impl IntoView {
    view! {
        <div class="map-page">
            <h2>"Élő járműkövetés"</h2>
            <p class="map-note">
                "Az adatok " <strong>"BKK Futár"</strong> " GTFS-RT feedből frissülnek, ~15 másodpercenként."
            </p>
            // Leaflet mounts here — id must match public/map.js
            <div id="live-map" style="height: 70vh; width: 100%;"></div>
        </div>
    }
}
