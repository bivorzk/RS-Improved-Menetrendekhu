use leptos::prelude::*;
use crate::components::page_header::SectionHeader;
use crate::components::summary_tiles::{SummaryTile, SummaryTiles};

#[component]
pub fn MapPage() -> impl IntoView {
    let stats = vec![
        SummaryTile { label: "Frissítés", value: "15s" },
        SummaryTile { label: "Járművek", value: "85 élő jármű" },
        SummaryTile { label: "Zóna", value: "Budapest teljes területe" },
    ];

    view! {
        <div class="space-y-8">
            <section class="rounded-[36px] border border-slate-800/90 bg-slate-900/90 p-8 shadow-soft sm:p-10">
                <div class="space-y-4">
                    <SectionHeader
                        eyebrow="Térképes nézet"
                        title="Élő járműkövetés térképen"
                        description="Kövesd az aktuális járatokat és a forgalmi viszonyokat egy modern, interaktív felületen."
                    />
                    <SummaryTiles items=stats/>
                </div>
            </section>

            <section class="rounded-[36px] border border-slate-800/90 bg-slate-900/90 p-4 shadow-soft">
                <div id="live-map" class="h-[70vh] w-full rounded-[32px] border border-slate-800/90 bg-slate-950"></div>
            </section>
        </div>
    }
}
