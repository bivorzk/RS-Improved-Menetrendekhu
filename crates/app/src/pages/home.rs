use leptos::prelude::*;

use crate::components::stop_search::StopSearch;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="home">
            <section class="hero">
                <h1>"Menetrend keresés"</h1>
                <p>"Keress megállót vagy járatot"</p>
            </section>
            <StopSearch/>
        </div>
    }
}
