use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::departure_board::DepartureBoard;

#[component]
pub fn StopPage() -> impl IntoView {
    let params = use_params_map();
    let stop_id = move || params.read().get("stop_id").unwrap_or_default();

    view! {
        <div class="stop-page">
            <h2>"Megálló: " {stop_id}</h2>
            <DepartureBoard stop_id=Signal::derive(stop_id)/>
        </div>
    }
}
