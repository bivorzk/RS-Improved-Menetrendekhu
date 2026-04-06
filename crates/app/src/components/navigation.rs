use leptos::prelude::*;

#[component]
pub fn TopNav() -> impl IntoView {
    let (locale, set_locale) = signal("HU".to_string());

    let toggle_locale = move |_| {
        set_locale.update(|current| {
            *current = if current == "HU" { "EN".to_string() } else { "HU".to_string() };
        });
    };

    view! {
        <header class="sticky top-0 z-50 border-b border-slate-800/80 bg-slate-950/95 backdrop-blur-lg">
            <div class="mx-auto flex max-w-7xl items-center justify-between gap-4 px-4 py-4 sm:px-6 lg:px-8">
                <a href="/" class="inline-flex items-center gap-3 rounded-2xl bg-slate-900/80 px-4 py-2 text-white transition hover:bg-slate-800">
                    <span class="text-xl">"🚌"</span>
                    <div class="text-left">
                        <p class="text-xs uppercase tracking-[0.28em] text-sky-300">"Transit"</p>
                        <p class="text-sm font-semibold">"Busz Menetrend"</p>
                    </div>
                </a>

                <nav class="hidden items-center gap-6 text-sm font-medium text-slate-300 md:flex">
                    <a href="/" class="transition hover:text-white">"Kezdőlap"</a>
                    <a href="/map" class="transition hover:text-white">"Térkép"</a>
                    <a href="/stop/1" class="transition hover:text-white">"Megálló"</a>
                </nav>

                <div class="flex items-center gap-3">
                    <button
                        type="button"
                        class="rounded-full border border-slate-700 bg-slate-900/80 px-4 py-2 text-sm font-semibold text-slate-200 transition hover:bg-slate-800"
                        on:click=toggle_locale
                    >
                        {move || locale.get()}
                    </button>
                </div>
            </div>
        </header>
    }
}
