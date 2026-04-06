use leptos::prelude::*;

#[derive(Clone)]
pub struct SummaryTile {
    pub label: &'static str,
    pub value: &'static str,
}

#[component]
pub fn SummaryTiles(items: Vec<SummaryTile>) -> impl IntoView {
    view! {
        <div class="grid gap-4 sm:grid-cols-3">
            {items.into_iter().map(|item| view! {
                <div class="rounded-3xl bg-slate-900/80 p-5">
                    <p class="text-xs uppercase tracking-[0.24em] text-slate-400">{item.label}</p>
                    <p class="mt-2 text-lg font-semibold text-white">{item.value}</p>
                </div>
            }).collect::<Vec<_>>() }
        </div>
    }
}
