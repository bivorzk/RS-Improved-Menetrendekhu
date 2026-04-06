use leptos::prelude::*;

#[derive(Clone)]
pub struct ConnectionSummary {
    pub departure: &'static str,
    pub arrival: &'static str,
    pub duration: &'static str,
    pub transfers: &'static str,
    pub lines: &'static [&'static str],
    pub status: &'static str,
    pub live: bool,
}

#[component]
pub fn ConnectionCard(summary: ConnectionSummary) -> impl IntoView {
    let status_class = if summary.live {
        "rounded-full bg-emerald-900/90 px-3 py-1 text-sm font-semibold text-emerald-200"
    } else {
        "rounded-full bg-slate-800 px-3 py-1 text-sm font-semibold text-slate-200"
    };

    view! {
        <article class="card p-6 shadow-soft transition hover:-translate-y-1 hover:shadow-lg">
            <div class="flex items-start justify-between gap-4">
                <div>
                    <p class="text-sm font-semibold uppercase tracking-[0.24em] text-sky-300">"Csatlakozás"</p>
                    <div class="mt-4 flex flex-wrap items-center gap-3 text-slate-200">
                        <span class="rounded-full bg-slate-800 px-3 py-1 text-sm">{summary.departure}</span>
                        <span class="text-sm text-slate-400">"→"</span>
                        <span class="rounded-full bg-slate-800 px-3 py-1 text-sm">{summary.arrival}</span>
                    </div>
                </div>

                <span class=status_class>
                    {summary.status}
                </span>
            </div>

            <div class="mt-6 grid gap-4 sm:grid-cols-3">
                <div class="rounded-3xl bg-slate-900/80 p-4">
                    <p class="text-xs uppercase tracking-[0.24em] text-slate-400">"Időtartam"</p>
                    <p class="mt-2 text-lg font-semibold text-white">{summary.duration}</p>
                </div>
                <div class="rounded-3xl bg-slate-900/80 p-4">
                    <p class="text-xs uppercase tracking-[0.24em] text-slate-400">"Átszállás"</p>
                    <p class="mt-2 text-lg font-semibold text-white">{summary.transfers}</p>
                </div>
                <div class="rounded-3xl bg-slate-900/80 p-4">
                    <p class="text-xs uppercase tracking-[0.24em] text-slate-400">"Járatok"</p>
                    <div class="mt-2 flex flex-wrap gap-2">
                        {summary.lines.iter().map(|line| view! {
                            <span class="rounded-full bg-sky-900/90 px-3 py-1 text-sm font-semibold text-sky-200">{*line}</span>
                        }).collect::<Vec<_>>() }
                    </div>
                </div>
            </div>
        </article>
    }
}
