use leptos::prelude::*;

#[component]
pub fn SectionHeader(eyebrow: &'static str, title: &'static str, description: &'static str) -> impl IntoView {
    view! {
        <div class="max-w-2xl space-y-3">
            <p class="text-sm uppercase tracking-[0.28em] text-sky-300">{eyebrow}</p>
            <h1 class="text-3xl font-semibold tracking-tight text-white sm:text-4xl">{title}</h1>
            <p class="text-slate-300 sm:text-lg">{description}</p>
        </div>
    }
}
