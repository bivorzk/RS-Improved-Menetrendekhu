// Live map powered by Leaflet + WebSocket GTFS-RT feed
// Loaded as a static asset — runs after Leptos mounts the #live-map div

(function () {
  "use strict";

  // Wait for the map container to appear in the DOM
  function initMap() {
    const el = document.getElementById("live-map");
    if (!el) return;

    // Centre on Hungary
    const map = L.map("live-map").setView([47.5, 19.0], 8);

    L.tileLayer("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png", {
      attribution: "© OpenStreetMap contributors",
      maxZoom: 19,
    }).addTo(map);

    const markers = {}; // vehicle_id → Leaflet marker

    const busIcon = L.divIcon({
      className: "bus-icon",
      html: "🚌",
      iconSize: [24, 24],
      iconAnchor: [12, 12],
    });

    function updatePositions(positions) {
      const seen = new Set();

      positions.forEach((v) => {
        seen.add(v.vehicle_id);
        const latlng = [v.lat, v.lon];

        if (markers[v.vehicle_id]) {
          markers[v.vehicle_id].setLatLng(latlng);
        } else {
          const marker = L.marker(latlng, { icon: busIcon })
            .addTo(map)
            .bindPopup(() => {
              return `
                <strong>Járat:</strong> ${v.route_id ?? "—"}<br>
                <strong>Menet:</strong> ${v.trip_id ?? "—"}<br>
                <strong>Sebesség:</strong> ${v.speed != null ? Math.round(v.speed * 3.6) + " km/h" : "—"}<br>
                <strong>Irány:</strong> ${v.bearing != null ? v.bearing + "°" : "—"}<br>
                <strong>Állapot:</strong> ${v.current_status ?? "—"}
              `;
            });
          markers[v.vehicle_id] = marker;
        }
      });

      // Remove markers for vehicles no longer in feed
      Object.keys(markers).forEach((id) => {
        if (!seen.has(id)) {
          map.removeLayer(markers[id]);
          delete markers[id];
        }
      });
    }

    // Initial load via REST (no wait for first WS message)
    fetch("/api/vehicles")
      .then((r) => r.json())
      .then(updatePositions)
      .catch(console.error);

    // Live updates via WebSocket
    const wsProto = location.protocol === "https:" ? "wss" : "ws";
    const ws = new WebSocket(`${wsProto}://${location.host}/ws/vehicles`);

    ws.addEventListener("message", (ev) => {
      try {
        const positions = JSON.parse(ev.data);
        updatePositions(positions);
      } catch (e) {
        console.error("WS parse error", e);
      }
    });

    ws.addEventListener("close", () => {
      console.warn("WS closed — reconnecting in 5s");
      setTimeout(initMap, 5000);
    });
  }

  // Leptos may hydrate asynchronously — retry until mount point exists
  let attempts = 0;
  const interval = setInterval(() => {
    if (document.getElementById("live-map")) {
      clearInterval(interval);
      initMap();
    } else if (++attempts > 20) {
      clearInterval(interval);
      console.warn("live-map element not found after 2s");
    }
  }, 100);
})();
