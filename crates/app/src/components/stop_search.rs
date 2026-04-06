use leptos::prelude::*;

#[derive(Clone, serde::Deserialize, serde::Serialize, Debug)]
pub struct StopResult {
    pub stop_id: String,
    pub name: String,
    pub lat: f64,
    pub lon: f64,
}

#[component]
pub fn StopSearch() -> impl IntoView {
    let (query, set_query) = signal(String::new());
    let (results, set_results) = signal(Vec::<StopResult>::new());
    let (loading, set_loading) = signal(false);

    // Search on input change — debounce handled server-side (min 2 chars)
    let on_input = move |ev: leptos::ev::Event| {
        let val = event_target_value(&ev);
        set_query.set(val.clone());

        if val.len() < 2 {
            set_results.set(vec![]);
            return;
        }

        set_loading.set(true);
        let val = val.clone();
        leptos::task::spawn_local(async move {
            // TODO: call server action / API endpoint
            // For now just clear
            let _ = val;
            set_results.set(vec![]);
            set_loading.set(false);
        });
    };

    view! {
        <div class="space-y-4 rounded-[28px] border border-slate-200/80 bg-white/95 p-5 shadow-soft">
            <label class="block text-sm font-medium text-slate-700">
                "Keress megállót"
                <input
                    type="text"
                    placeholder="Megálló neve (pl. Győr, autóbusz-állomás)"
                    prop:value=query
                    on:input=on_input
                    class="mt-2 w-full rounded-2xl border border-slate-200 bg-slate-50 px-4 py-3 text-slate-900 outline-none transition focus:border-sky-400 focus:ring-4 focus:ring-sky-100"
                />
            </label>

            {move || loading.get().then(|| view! { <p class="text-sm text-slate-500">"Keresés..."</p> })}

            <ul class="space-y-2">
                {move || results.get().into_iter().map(|stop| {
                    let stop_id = stop.stop_id.clone();
                    view! {
                        <li>
                            <a href=format!("/stop/{}", stop_id) class="block rounded-2xl border border-slate-200 bg-slate-50 px-4 py-3 text-slate-900 transition hover:border-sky-300 hover:bg-sky-50">
                                {stop.name.clone()}
                            </a>
                        </li>
                    }
                }).collect::<Vec<_>>() }
            </ul>
        </div>
    }
}
