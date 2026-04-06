use leptos::prelude::*;
use crate::components::connection_card::{ConnectionCard, ConnectionSummary};
use crate::components::page_header::SectionHeader;
use crate::components::summary_tiles::{SummaryTile, SummaryTiles};

#[component]
pub fn ResultsPage() -> impl IntoView {
    let results = vec![
        ConnectionSummary {
            departure: "10:12",
            arrival: "11:05",
            duration: "53 min",
            transfers: "1 átszállás",
            lines: &["22", "M3"],
            status: "Live",
            live: true,
        },
        ConnectionSummary {
            departure: "10:30",
            arrival: "11:20",
            duration: "50 min",
            transfers: "0 átszállás",
            lines: &["7", "M2"],
            status: "Pontosan",
            live: false,
        },
        ConnectionSummary {
            departure: "10:45",
            arrival: "11:42",
            duration: "57 min",
            transfers: "2 átszállás",
            lines: &["105", "H7", "4"],
            status: "Késés +3p",
            live: false,
        },
    ];

    let stats = vec![
        SummaryTile { label: "Keresett útvonal", value: "Nyugati pályaudvar → Deák Ferenc tér" },
        SummaryTile { label: "Dátum", value: "2026. április 5." },
        SummaryTile { label: "Indulás", value: "10:00" },
    ];

    view! {
        <div class="space-y-8">
            <section class="rounded-[36px] border border-slate-800/90 bg-slate-900/90 p-8 shadow-soft sm:p-10">
                <div class="space-y-4">
                    <SectionHeader
                        eyebrow="Keresési eredmények"
                        title="Találd meg a legjobb kapcsolatot"
                        description="Ezek a legfrissebb ajánlatok a keresésed alapján. Szűrhetsz idő, átszállás vagy késés szerint."
                    />
                    <SummaryTiles items=stats/>
                </div>
            </section>

            <section class="space-y-4">
                {results.into_iter().map(|summary| view! {
                    <ConnectionCard summary=summary/>
                }).collect::<Vec<_>>() }
            </section>
        </div>
    }
}
