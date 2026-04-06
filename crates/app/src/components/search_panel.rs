use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use leptos_router::NavigateOptions;

#[cfg(target_arch = "wasm32")]
fn default_local_date_time() -> (String, String) {
    let now = js_sys::Date::new_0();
    let date = now.to_iso_string().as_string().unwrap_or_default();
    let mut parts = date.split('T');
    let day = parts.next().unwrap_or_default().to_string();
    let time = parts
        .next()
        .map(|t| t.split(':').take(2).collect::<Vec<_>>().join(":"))
        .filter(|t| !t.is_empty())
        .unwrap_or_else(|| "08:30".to_string());
    (day, time)
}

#[cfg(not(target_arch = "wasm32"))]
fn default_local_date_time() -> (String, String) {
    ("".to_string(), "".to_string())
}

#[component]
pub fn SearchPanel() -> impl IntoView {
    let (origin, set_origin) = signal(String::new());
    let (destination, set_destination) = signal(String::new());
    let (date, set_date) = signal(default_local_date_time().0);
    let (time, set_time) = signal(default_local_date_time().1);

    let swap_route = move |_| {
        let origin_value = origin.get();
        let destination_value = destination.get();
        set_origin.set(destination_value);
        set_destination.set(origin_value);
    };

    let route_summary = Memo::new(move |_| {
        if origin.get().is_empty() || destination.get().is_empty() {
            "Add origin and destination to preview your route.".to_string()
        } else {
            format!("{} → {} • {} at {}", origin.get(), destination.get(), date.get(), time.get())
        }
    });

    let nav = use_navigate();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if origin.get().is_empty() || destination.get().is_empty() {
            return;
        }

        let route = format!("/results?origin={}&destination={}&date={}&time={}",
            origin.get(), destination.get(), date.get(), time.get());
        nav(&route, NavigateOptions::default());
    };

    view! {
        <form class="space-y-6" on:submit=on_submit>
            <div class="grid gap-4 rounded-[28px] border border-slate-800/90 bg-slate-900/90 p-4 shadow-sm sm:grid-cols-[1fr_auto]">
                <label class="grid gap-2 text-sm text-slate-200">
                    "Indulási hely"
                    <input
                        type="text"
                        placeholder="pl. Nyugati pályaudvar"
                        prop:value=origin
                        on:input=move |ev| set_origin.set(event_target_value(&ev))
                        class="rounded-2xl border border-slate-700 bg-slate-950 px-4 py-3 text-slate-100 outline-none transition focus:border-sky-400 focus:ring-4 focus:ring-sky-500/20"
                    />
                </label>

                <label class="grid gap-2 text-sm text-slate-200">
                    "Érkezési cél"
                    <input
                        type="text"
                        placeholder="pl. Deák Ferenc tér"
                        prop:value=destination
                        on:input=move |ev| set_destination.set(event_target_value(&ev))
                        class="rounded-2xl border border-slate-700 bg-slate-950 px-4 py-3 text-slate-100 outline-none transition focus:border-sky-400 focus:ring-4 focus:ring-sky-500/20"
                    />
                </label>

                <button
                    type="button"
                    class="row-span-2 inline-flex h-fit items-center justify-center rounded-2xl border border-slate-700 bg-slate-800 px-4 py-3 text-sm font-semibold text-slate-200 transition hover:bg-slate-700"
                    on:click=swap_route
                >
                    "Csere"
                </button>
            </div>

            <div class="grid gap-4 sm:grid-cols-2">
                <label class="grid gap-2 text-sm text-slate-200">
                    "Dátum"
                    <input
                        type="date"
                        prop:value=date
                        on:input=move |ev| set_date.set(event_target_value(&ev))
                        class="rounded-2xl border border-slate-700 bg-slate-950 px-4 py-3 text-slate-100 outline-none transition focus:border-sky-400 focus:ring-4 focus:ring-sky-500/20"
                    />
                </label>

                <label class="grid gap-2 text-sm text-slate-200">
                    "Időpont"
                    <input
                        type="time"
                        prop:value=time
                        on:input=move |ev| set_time.set(event_target_value(&ev))
                        class="rounded-2xl border border-slate-700 bg-slate-950 px-4 py-3 text-slate-100 outline-none transition focus:border-sky-400 focus:ring-4 focus:ring-sky-500/20"
                    />
                </label>
            </div>

            <div class="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
                <button
                    type="submit"
                    class="inline-flex items-center justify-center rounded-2xl bg-sky-600 px-6 py-3 text-base font-semibold text-white shadow-lg shadow-sky-600/20 transition hover:bg-sky-700"
                >
                    "Járat keresése"
                </button>
                <p class="text-sm text-slate-400">{route_summary()}</p>
            </div>
        </form>
    }
}
