use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

pub mod components;
pub mod layout;
pub mod pages;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Busz Menetrend"/>
        <Meta charset="utf-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1"/>

        <Router>
            <layout::AppLayout>
                <Routes fallback=|| view! { <p class="text-slate-200">"Az oldal nem található."</p> }>
                    <Route path=path!("/") view=pages::home::HomePage/>
                    <Route path=path!("/results") view=pages::results::ResultsPage/>
                    <Route path=path!("/map") view=pages::map::MapPage/>
                    <Route path=path!("/stop/:stop_id") view=pages::stop::StopPage/>
                </Routes>
            </layout::AppLayout>
        </Router>
    }
}
