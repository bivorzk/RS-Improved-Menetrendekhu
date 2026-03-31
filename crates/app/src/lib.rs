use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

pub mod components;
pub mod pages;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/bus-tracker.css"/>
        <Title text="Busz Menetrend"/>
        <Meta charset="utf-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1"/>

        <Router>
            <nav class="navbar">
                <a href="/">"🚌 Busz Menetrend"</a>
                <a href="/map">"Térkép"</a>
            </nav>
            <main>
                <Routes fallback=|| view! { <p>"Az oldal nem található."</p> }>
                    <Route path=path!("/") view=pages::home::HomePage/>
                    <Route path=path!("/map") view=pages::map::MapPage/>
                    <Route path=path!("/stop/:stop_id") view=pages::stop::StopPage/>
                </Routes>
            </main>
        </Router>
    }
}
