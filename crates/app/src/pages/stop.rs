use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::components::departure_board::DepartureBoard;
use crate::components::page_header::SectionHeader;

#[component]
pub fn StopPage() -> impl IntoView {
    let params = use_params_map();
    let stop_id = move || params.read().get("stop_id").unwrap_or_default();

    view! {
        <div class="space-y-6">
            <section class="rounded-[36px] border border-slate-800/90 bg-slate-900/90 p-8 shadow-soft sm:p-10">
                <div class="flex flex-col gap-4 sm:flex-row sm:items-end sm:justify-between">
                    <SectionHeader
                        eyebrow="Megálló részletek"
                        title="Megálló: "
                        description="Tekintsd meg a következő indulásokat és a megállóhoz tartozó közeli járatokat."
                    />
                    <a href="/" class="inline-flex items-center justify-center rounded-2xl bg-sky-600 px-5 py-3 text-sm font-semibold text-white transition hover:bg-sky-700">"Vissza a kereséshez"</a>
                </div>
                <p class="text-slate-400">{stop_id()}</p>
            </section>

            <section class="rounded-[36px] border border-slate-800/90 bg-slate-900/90 p-8 shadow-soft sm:p-10">
                <DepartureBoard stop_id=Signal::derive(stop_id)/>
            </section>
        </div>
    }
}
